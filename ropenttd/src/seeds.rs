//! Openttd seeds processors
//!
//! Original source:
//! - https://github.com/OpenTTD/OpenTTD/blob/master/src/townname.cpp#L152
//! - https://github.com/OpenTTD/OpenTTD/blob/master/src/core/bitmath_func.hpp#L32

/// Fetch n bits from x, started at bit s.
pub fn gb(x: u32, s: u8, n: u8) -> u32 {
	  return (x >> s) & ((1 << n) - 1);
}

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
