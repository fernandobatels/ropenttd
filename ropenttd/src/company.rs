//! Company related types

use crate::chunk_reader::ChunkReader;
use crate::error::Error;
use crate::string_reader::*;
use crate::money::{Money, currencies};

/// Company informations
#[derive(Debug, PartialEq)]
pub struct CompanyInfo {
    /// Company name
    pub name: String,
    /// President name
    pub president: String,
    /// Total company money
    pub money: Money,
    /// Amount of money borrowed from the bank
    pub loan: Money,
    /// Year of starting the company
    pub inaugurated_year: u32,
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

        chunk.advance::<u32>()?; // President face

        // Company total money
        let money = {
            let money = chunk.fetch::<i64>()?;
            Money::new(money, currencies::GBP)
        };

        // Company current loan
        let loan = {
            let loan = chunk.fetch::<i64>()?;
            Money::new(loan, currencies::GBP)
        };

        chunk.advance::<u8>()?; // Colour
        chunk.advance::<u8>()?; // Money fraction
        chunk.advance::<u8>()?; // Block preview
        chunk.advance::<i32>()?; // Location of HQ
        chunk.advance::<i32>()?; // Last build coordinate

        // Start company year
        let inaugurated_year = chunk.fetch::<u32>()?;

        Ok(CompanyInfo {
            name,
            president,
            money,
            loan,
            inaugurated_year
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

    use crate::money::currencies;
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
        assert_eq!(1950, company.inaugurated_year);

        Ok(())
    }

    /// Company money
    #[test]
    fn money() -> Result<(), String> {

        let company = CompanyInfo::parse(&PLYR.to_vec())
            .map_err(|e| e.to_string())?;

        assert_eq!(3_647_337, company.money.value);
        assert_eq!(14_589_348, company.money.exchange(currencies::BRL).value);
        assert_eq!(0, company.loan.value);
        assert_eq!(0, company.loan.exchange(currencies::BRL).value);

        Ok(())
    }

    static PLYR: [u8; 100] = [0x50, 0x4c, 0x59, 0x52, 0x1, 0x91, 0x1f, 0x83, 0x2a, 0xa, 0xcb, 0x70, 0xea, 0x0, 0x70, 0xe7, 0x1c, 0xb8, 0xed, 0x2d, 0x0, 0xc0, 0x8, 0x80, 0xa8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x37, 0xa7, 0x69, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xc, 0x68, 0x0, 0x0, 0xd, 0xb0, 0x25, 0x0, 0xe, 0x60, 0x78, 0x00, 0x00, 0x07, 0x9e, 0xff, 0xff, 0xff, 0xff, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd7, 0x4b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xea, 0xa4, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x6a, 0xaa, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x59, 0x6b, 0x00, 0x00];
}
