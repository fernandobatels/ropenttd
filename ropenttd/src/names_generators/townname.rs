//! Openttd town names generation API

use crate::error::Error;
use crate::table::townname as table;
use super::{NameGeneration, NameLanguage};

pub struct TownName {}

impl NameGeneration for TownName {
    /// Generate the town name
    fn generate(language: NameLanguage, seed: u32) -> Result<String, Error> {

        let generated = match language {
            0 => Ok(TownName::generate_english_original(seed)),
            _ => Err(Error::TypeNotSupportedYet("Towname language".to_string(), language))
        }?;

        Ok(format!("{} Transport", generated))
    }
}

impl TownName {
    fn generate_english_original(seed: u32) -> String {
        table::NAME_ORIGINAL_ENGLISH_1[0].to_string()
    }
}

#[cfg(test)]
mod test {

    use super::{TownName, NameGeneration};

    #[test]
    fn generate_english_petfield() -> Result<(), String> {

        let gen = TownName::generate(0, 2200570571)
            .map_err(|e| e.to_string())?;

        assert_eq!("Petfield Transport".to_string(), gen);

        Ok(())
    }
}
