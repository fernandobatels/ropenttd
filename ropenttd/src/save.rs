//! Save game object

use std::io::{Read, BufReader};
use std::fs::File;

use crate::format::Format;
use crate::error::Error;

/// Save game
pub struct SaveGame {
    format: Format
}

impl SaveGame {

    /// Load the save from bytes
    pub fn load(mut buffer: BufReader<File>) -> Result<SaveGame, Error> {

        let format = {
            let mut tag = [0 as u8; 4];

            buffer.read_exact(&mut tag)?;

            Format::identify(tag)?
        };

        Ok(SaveGame {
            format
        })
    }
}
