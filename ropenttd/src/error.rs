
use std::fmt;
use std::io::Error as IoError;

/// Error
#[derive(Debug)]
pub enum Error {

    /// Errors on load the save file
    Load(String),

}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::Load(e.to_string())
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Error::Load(e) => e
        })
    }
}
