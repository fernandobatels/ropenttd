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

        let mut name = String::new();

        // First segment(optional)
        if let Some(fi) = seed_chance_bias(0, table::NAME_ORIGINAL_ENGLISH_1.len(), seed, 50) {
            name.push_str(table::NAME_ORIGINAL_ENGLISH_1[fi]);
        }

	      // Middle segments
	      name.push_str(table::NAME_ORIGINAL_ENGLISH_2[seed_chance(4, table::NAME_ORIGINAL_ENGLISH_2.len(), seed)]);
	      name.push_str(table::NAME_ORIGINAL_ENGLISH_3[seed_chance(7, table::NAME_ORIGINAL_ENGLISH_3.len(), seed)]);
	      name.push_str(table::NAME_ORIGINAL_ENGLISH_4[seed_chance(10, table::NAME_ORIGINAL_ENGLISH_4.len(), seed)]);
	      name.push_str(table::NAME_ORIGINAL_ENGLISH_5[seed_chance(13, table::NAME_ORIGINAL_ENGLISH_5.len(), seed)]);

        // Last segment(optional)
        if let Some(fi) = seed_chance_bias(15, table::NAME_ORIGINAL_ENGLISH_6.len(), seed, 60) {
            name.push_str(table::NAME_ORIGINAL_ENGLISH_6[fi]);
        }

        name
    }
}

/// Return the an index number from given seed
fn seed_chance(shift_by: u8, max: usize, seed: u32) -> usize {
    // Fetch some bits from the seed
    let gb = (seed >> shift_by) & ((1 << 16) - 1);

    ((gb * (max as u32)) >> 16) as usize
}

/// Return the an index number from given seed,
/// but with a limitator
fn seed_chance_bias(shift_by: u8, max: usize, seed: u32, bias: u16) -> Option<usize> {
    let chance = seed_chance(shift_by, (max as u16 + bias) as usize, seed) as i32 - bias as i32;

    if chance >= 0 {
        Some(chance as usize)
    } else {
        None
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
