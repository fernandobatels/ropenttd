//! Openttd seeds processors
//!
//! Original source:
//! - https://github.com/OpenTTD/OpenTTD/blob/master/src/townname.cpp#L152

use crate::bitmath::gb;

/// Return the an index number from given seed
pub fn seed_chance(shift_by: u8, max: usize, seed: u32) -> usize {
    ((gb(seed, shift_by, 16) * (max as u32)) >> 16) as usize
}

/// Return the an index number from given seed,
/// but with 8u size instead of 16u
pub fn seed_chance8(shift_by: u8, max: usize, seed: u32) -> usize {
    ((gb(seed, shift_by, 8) * (max as u32)) >> 8) as usize
}

/// Return the an index number from given seed,
/// but with a limitator
pub fn seed_chance_bias(shift_by: u8, max: usize, seed: u32, bias: u16) -> Option<usize> {
    let chance = seed_chance(shift_by, (max as u16 + bias) as usize, seed) as i32 - bias as i32;

    if chance >= 0 {
        Some(chance as usize)
    } else {
        None
    }
}
