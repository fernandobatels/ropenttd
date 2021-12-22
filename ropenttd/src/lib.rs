//!
//! OpenTTD reader
//!
//! Lib to work with OpenTTD save files
//!

mod format;
mod save;
mod error;
mod company;
mod chunk_reader;
mod string_reader;

pub use format::Format;
pub use save::SaveGame;
pub use company::Company;
pub use error::Error;
