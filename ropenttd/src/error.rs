//! Error API

use std::fmt;
use std::io::Error as IoError;
use lzma::LzmaError;

#[derive(Debug)]
pub enum Error {
    /// Errors on load the save file
    Load(String),
    /// Errors on decompress the content
    Decompress(String)
}

impl From<LzmaError> for Error {
    fn from(e: LzmaError) -> Self {
        match e {
            LzmaError::Io(em) => Self::Load(em.to_string()),
            _ => Self::Decompress(e.to_string()),
        }
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::Load(e.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Error::Load(e) => e,
            Error::Decompress(e) => e
        })
    }
}
