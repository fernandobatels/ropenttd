//! Save formats and his compression filters
///
/// See the OpenTTD formats here: https://github.com/OpenTTD/OpenTTD/blob/master/src/saveload/saveload.cpp#L2322

use crate::error::Error;

/// Save formats
#[derive(Debug)]
pub enum Format {
    Ottx,
    //Ottd,
    //Ottn,
    //Ottz
}

impl Format {

    /// Identify the fiel save format/type.
    ///
    /// Non identifield or support types will
    /// raise an error.
    pub fn identify(tag: [u8; 4]) -> Result<Format, Error> {
        match tag {
            [0x4f, 0x54, 0x54, 0x58] => Ok(Format::Ottx),
            _ => Err(Error::Load(format!("File format not supported: {:?}", tag)))
        }
    }
}
