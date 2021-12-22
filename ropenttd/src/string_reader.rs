//! Openttd strings related API
//!
//! Functions foo read the strings from the
//! OpenTTD implementation.


use crate::error::Error;

/// Number of bits for the StringIndex within a StringTab
static TAB_SIZE_BITS: u8 = 11;

/// Read the string by an StringID
pub fn get_string(id: u16, id_param: u32) -> Result<String, Error> {

    let index = get_string_index(id);
    let tab = get_string_tab(id);

    println!("{0} {1} {2} {3} {4}", id, index, tab, id_param, index - 0xC0);

    todo!()
}

/// Extract the StringIndex from a StringID
fn get_string_index(id: u16) -> u16 {
	  id - (get_string_tab(id) << TAB_SIZE_BITS)
}

/// Extract the StringTab from a StringID
fn get_string_tab(id: u16) -> u16 {
    let tab = id >> TAB_SIZE_BITS;

    match tab {
        t if t >= 64 => 64, // Start of NewGRF supplied strings
        t if t >= 32 => 32, // Start of GameScript supplied strings
        _ => tab
    }
}
