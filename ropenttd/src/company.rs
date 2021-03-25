//! Company related types

use crate::chunk_reader::ChunkReader;
use crate::error::Error;

/// Company informations
pub struct CompanyInfo {
    pub name: String
}

impl CompanyInfo {

    /// Parse the company information
    pub fn parse(buffer: &Vec<u8>) -> Result<CompanyInfo, Error> {

        let mut chunk = ChunkReader::find(buffer, "PLYR")?;

        println!("name2 {:?}", chunk.fetch::<u8>());
        println!("name1 {:?}", chunk.fetch::<u16>());
        println!("name {:?}", chunk.fetch::<String>());

        todo!("??")
    }
}

/// Company access
pub trait Company {
    /// Return the compnay details/information
    fn company(&mut self) -> Result<CompanyInfo, Error>;
}
