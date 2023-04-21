//! Openttd bit's utils
//!
//! Original source:
//! - https://github.com/OpenTTD/OpenTTD/blob/master/src/core/bitmath_func.hpp

/// Fetch n bits from x, started at bit s
pub fn gb(x: u32, s: u8, n: u8) -> u32 {
	  return (x >> s) & ((1 << n) - 1);
}

/// Checks if a bit in a value is set
pub fn has_bit(x: u32, b: u8) -> bool {
	  return (x & (1 << b)) != 0;
}
