//! Company related types

use crate::chunk_reader::ChunkReader;
use crate::error::Error;
use crate::string_reader::*;

/// Company informations
#[derive(Debug)]
pub struct CompanyInfo {
    pub name: String,
    pub president: String
}

impl CompanyInfo {

    /// Parse the company information
    pub fn parse(buffer: &Vec<u8>) -> Result<CompanyInfo, Error> {

        let mut chunk = ChunkReader::find(buffer, "PLYR")?; // 50 4c 59 52

        // Fields from https://github.com/OpenTTD/OpenTTD/blob/9e47df298faf6889c8be7dd0b0eeedeb65db1cdc/src/saveload/company_sl.cpp#L444

        // Company name
        let name = {
            let name2 = chunk.fetch::<u32>()?; // name_2
            let name1 = chunk.fetch::<StringID>()?; // name_1
            let name = chunk.fetch::<String>()?; // name

            if !name.is_empty() {
                name
            } else {
                OpenString::new(name1, name2).to_string()?
            }
        };

        // President name
        let president = {
            let name1 = chunk.fetch::<StringID>()?; // president_name_1
            let name2 = chunk.fetch::<u32>()?; // president_name_2
            let name = chunk.fetch::<String>()?; // president_name

            if !name.is_empty() {
                name
            } else {
                OpenString::new(name1, name2).to_string()?
            }
        };

        Ok(CompanyInfo {
            name,
            president
        })
    }
}

/// Company access
pub trait Company {
    /// Return the company details/information
    fn company(&mut self) -> Result<CompanyInfo, Error>;
}

#[cfg(test)]
mod test {

    use crate::company::CompanyInfo;

    /// When you change the original name
    #[test]
    fn name_on_plyr_chunk() -> Result<(), String> {

        let mut buffer = [0x50, 0x4c, 0x59, 0x52, 0x1, 0x91, 0x33, 0x83, 0x2a, 0xa, 0xcb, 0x70, 0xea, 0x14, 0x50, 0x65, 0x74, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x32, 0x70, 0xe7, 0x1c, 0xb8, 0xed, 0x2d, 0x0, 0xc0, 0x8, 0x80].to_vec();
        buffer.resize_with(100, || 0x0);

        let company = CompanyInfo::parse(&buffer)
            .map_err(|e| e.to_string())?;

        assert_eq!("Petfield Transport 2".to_string(), company.name);
        assert_eq!("D. Nelson".to_string(), company.president);

        Ok(())
    }

    /// The auto-generated name
    #[test]
    fn name_outside_plyr_chunk() -> Result<(), String> {

        let company = CompanyInfo::parse(&PLYR.to_vec())
            .map_err(|e| e.to_string())?;

        assert_eq!("Petfield Transport".to_string(), company.name);
        assert_eq!("D. Nelson".to_string(), company.president);

        Ok(())
    }

    static PLYR: [u8; 50] = [0x50, 0x4c, 0x59, 0x52, 0x1, 0x91, 0x1f, 0x83, 0x2a, 0xa, 0xcb, 0x70, 0xea, 0x0, 0x70, 0xe7, 0x1c, 0xb8, 0xed, 0x2d, 0x0, 0xc0, 0x8, 0x80, 0xa8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x37, 0xa7, 0x69, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xc, 0x68, 0x0, 0x0, 0xd, 0xb0, 0x25, 0x0, 0xe];
}
