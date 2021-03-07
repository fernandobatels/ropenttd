//!
//! OpenTTD reader
//!
//! Lib to work with OpenTTD save files
//!

mod format;
mod save;
mod error;

pub use format::Format;
pub use save::SaveGame;
pub use error::Error;
