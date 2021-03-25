//! Error API

use std::fmt;
use std::io::Error as IoError;
use lzma::LzmaError;
use std::sync::PoisonError;

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
    UnexpectedValueType(u8)
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Error::UnexpectedValueType(tp) => format!("type fetched: {}", tp),
            Error::ChunkLockError => "Error on lock the chunk".to_string(),
            Error::ChunkNotFound(id) => format!("chunk id: {}", id),
            Error::ChunkNotSupported(id) => format!("chunk id: {}", id),
            Error::Load(e) => e.to_string(),
            Error::DataCorruption(e) => e.to_string(),
            Error::Decompress(e) => e.to_string()
        })
    }
}
