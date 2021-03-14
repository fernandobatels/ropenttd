//! Save game object

use std::io::{Read, BufReader, Seek, SeekFrom};
use std::fs::File;

use crate::format::Format;
use crate::error::Error;
use crate::company::{Company, CompanyInfo};

/// Save game
pub struct SaveGame {
    pub format: Format,
    raw: Vec<u8>
}

impl SaveGame {

    /// Load the save from bytes
    pub fn load(bf_reader: &mut BufReader<File>) -> Result<SaveGame, Error> {

        let format = {
            let mut tag = [0 as u8; 4];

            bf_reader.read_exact(&mut tag)?;

            Format::identify(tag)?
        };

        bf_reader.seek(SeekFrom::Start(8))?;

        let mut buffer = vec![];
        bf_reader.read_to_end(&mut buffer)?;

        let raw = lzma::decompress(&mut buffer)?;

        Ok(SaveGame {
            format,
            raw
        })
    }
}

impl Company for SaveGame {
    fn company(&mut self) -> Result<CompanyInfo, Error> {
        CompanyInfo::parse(&self.raw)
    }
}
