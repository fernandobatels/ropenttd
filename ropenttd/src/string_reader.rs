//! Openttd strings related API
//!
//! Functions for read the strings from the
//! OpenTTD implementation.

use crate::error::Error;

/// Number of bits for the StringIndex within a StringTab
static TAB_SIZE_BITS: u8 = 11;

pub type StringID = u16;
pub type StringTab = u16;

pub struct OpenString {
    id: StringID,
    id_param: u32,
    index: u16,
    tab: StringTab
}

impl OpenString {

    pub fn new(id: StringID, id_param: u32) -> OpenString {

        let index = get_string_index(id);
        let tab = get_string_tab(id);

        OpenString {
            id,
            id_param,
            index,
            tab
        }
    }

    /// Read the string by an StringID
    pub fn to_string(self) -> Result<String, Error> {

        println!("{0} {1} {2} {3} {4}", self.id, self.index, self.tab, self.id_param, self.index - 0xE4);

        todo!()
    }
}

/// Extract the string index from a StringID
fn get_string_index(id: StringID) -> u16 {
	  id - (get_string_tab(id) << TAB_SIZE_BITS)
}

/// Extract the StringTab from a StringID
fn get_string_tab(id: StringID) -> StringTab {
    let tab = id >> TAB_SIZE_BITS;

    match tab {
        t if t >= 64 => 64, // Start of NewGRF supplied strings
        t if t >= 32 => 32, // Start of GameScript supplied strings
        _ => tab
    }
}
