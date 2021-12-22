//! Error API

use std::fmt;
use std::io::Error as IoError;
use lzma::LzmaError;
use std::sync::PoisonError;
use std::string::FromUtf8Error;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum Error {
    /// Errors on load the save file
    Load(String),
    /// Errors on decompress the content
    Decompress(String),
    /// Chunk of data not found
    ChunkNotFound(String),
    /// Chunk, or chunk format, is not supported
    ChunkNotSupported(String),
    /// Data is corrupted
    DataCorruption(String),
    /// Fail on lock the chunk
    ChunkLockError,
    /// Unexpected fetched value type
    UnexpectedValueType(u8, u8),
    /// UTF8 Decode
    Utf8Decode(String)
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

impl From<PoisonError<&mut bytes::Bytes>> for Error {
    fn from(_e: PoisonError<&mut bytes::Bytes>) -> Self {
        Self::ChunkLockError
    }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Self::Utf8Decode(e.to_string())
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Self::Utf8Decode(e.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Error::Utf8Decode(e) => format!("Found a value with an invalid UTF-8 string: {}", e),
            Error::UnexpectedValueType(exp, get) => format!("type fetched {}, expected {}", get, exp),
            Error::ChunkLockError => "Error on lock the chunk".to_string(),
            Error::ChunkNotFound(id) => format!("chunk id: {}", id),
            Error::ChunkNotSupported(id) => format!("chunk id: {}", id),
            Error::Load(e) => e.to_string(),
            Error::DataCorruption(e) => e.to_string(),
            Error::Decompress(e) => e.to_string()
        })
    }
}
