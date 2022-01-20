//! Openttd town names generation API

use crate::error::Error;
use super::{NameGeneration, NameLanguage};

pub struct TownName {}

impl NameGeneration for TownName {
    /// Generate the town name
    fn generate(language: NameLanguage, seed: u32) -> Result<String, Error> {
        todo!("????????????")
    }
}

