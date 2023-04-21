//! Openttd president names generation API

use crate::error::Error;
use crate::table::personname as table;
use super::{NameGeneration, NameLanguage};
use crate::seeds;

pub struct PresidentName {}

impl NameGeneration for PresidentName {
    /// Generate the president name
    fn generate(_: NameLanguage, seed: u32) -> Result<String, Error> {

        let mut name = String::new();

        // initial name letter
        name.push_str(table::INITIAL_NAME_LETTERS[seeds::seed_chance8(0, table::INITIAL_NAME_LETTERS.len(), seed)]);
        name.push_str(". ");

        // surname
        name.push_str(table::SURNAMES[seeds::seed_chance8(16, table::SURNAMES.len(), seed)]);

        Ok(name)
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
