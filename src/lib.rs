//! OpenTTD reader
//!
//! A crate to read the OpenTTD save files

mod format;
mod save;
mod error;
mod company;
mod chunk_reader;
mod string_reader;
mod names_generators;
mod table;
mod seeds;
mod money;
mod vehicle;
mod bitmath;
#[cfg(test)]
pub mod tests;

pub use format::Format;
pub use save::SaveGame;
pub use company::Company;
pub use vehicle::{Vehicles, Train};
pub use error::Error;
pub use money::{Money, Currency, currencies};
