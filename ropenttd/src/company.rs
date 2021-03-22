//! Company related types

use subslice::bmh;
use bytes::{Bytes, Buf};
use std::str;

use crate::error::Error;

/// Company informations
pub struct CompanyInfo {
    pub name: String
}

impl CompanyInfo {

    /// Parse the company information
    pub fn parse(buffer: &Vec<u8>) -> Result<CompanyInfo, Error> {

        let pos_init = match bmh::find(buffer, "PLYR".as_bytes()) {
            Some(pos) => Ok(pos),
            None => Err(Error::ChunkNotFound("PLYR".to_string()))
        }?;

        let (_,company_chunk) = buffer.split_at(pos_init);

        println!("chunk type: {}", company_chunk[4]); // ARRAY

        let mut company_chunk = Bytes::copy_from_slice(&company_chunk);
        company_chunk.advance(5); // PLYR + chunk type

        // SL_VAR
        println!("name2 type: {}", company_chunk.get_u8() & 0xF); // U8
        println!("name2 val: {}", company_chunk.get_u8());

        // SL_VAR
        println!("name1 type: {}", company_chunk.get_u8() & 0xF); // U16
        println!("name1 val: {}", company_chunk.get_u16());

        // SL_STR
        company_chunk.advance(3); // Some bytes not understood

        let name_len = company_chunk.get_u8() as usize;
        println!("name len: {:?}", name_len); // 20
        let name = company_chunk.copy_to_bytes(name_len);
        println!("{:?}", &name);
        println!("name val: {:?}", str::from_utf8(&name));

        todo!("??")
    }
}

/// Company access
pub trait Company {
    /// Return the compnay details/information
    fn company(&mut self) -> Result<CompanyInfo, Error>;
}
