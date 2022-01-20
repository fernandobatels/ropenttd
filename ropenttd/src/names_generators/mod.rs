//! Openttd names generation API

use crate::error::Error;

mod townname;
pub use townname::TownName;

pub type NameLanguage = u16;

/// Names generation API
pub trait NameGeneration {
    fn generate(lang: NameLanguage, seed: u32) -> Result<String, Error>;
}
