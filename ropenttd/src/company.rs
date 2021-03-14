//! Company related types

use crate::error::Error;

/// Company informations
pub struct CompanyInfo {
    pub name: String
}

impl CompanyInfo {

    /// Parse the company information
    pub fn parse(buffer: &Vec<u8>) -> Result<CompanyInfo, Error> {
        todo!("??")
    }
}

/// Company access
pub trait Company {
    /// Return the compnay details/information
    fn company(&mut self) -> Result<CompanyInfo, Error>;
}
