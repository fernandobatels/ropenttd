//! Openttd names generation API

use crate::error::Error;

mod town;
mod president;
pub use town::TownName;
pub use president::PresidentName;

pub type NameLanguage = u16;

/// Names generation API
pub trait NameGeneration {
    fn generate(lang: NameLanguage, seed: u32) -> Result<String, Error>;
}
