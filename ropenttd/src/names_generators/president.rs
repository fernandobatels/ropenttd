//! Openttd president names generation API

use crate::error::Error;
use crate::table::personname as table;
use super::{NameGeneration, NameLanguage};

pub struct PresidentName {}

impl NameGeneration for PresidentName {
    /// Generate the president name
    fn generate(_: NameLanguage, seed: u32) -> Result<String, Error> {
        Ok("??".to_string())
    }
}


#[cfg(test)]
mod test {

    use super::{PresidentName, NameGeneration};

    #[test]
    fn generate_dnelson() -> Result<(), String> {

        let gen = PresidentName::generate(0, 481881389)
            .map_err(|e| e.to_string())?;

        assert_eq!("D. Nelson".to_string(), gen);

        Ok(())
    }
}
