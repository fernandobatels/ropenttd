//! Openttd strings related API
//!
//! Functions for read the strings from the
//! OpenTTD implementation.

use crate::error::Error;
use crate::names_generators::*;

/// Number of bits for the StringIndex within a StringTab
static TAB_SIZE_BITS: u8 = 11;

pub type StringID = u16;
pub type StringTab = u16;

pub struct OpenString {
    pub id: StringID,
    pub id_param: u32,
    pub index: u16,
    pub tab: StringTab
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
        match self.tab {
            // TEXT_TAB_SPECIAL
            14 => self.to_special_string(),
            _ => unimplemented!("String tab {}", self.tab)
        }
    }

    /// Special string generation
    fn to_special_string(self) -> Result<String, Error> {
        let tp = self.index - 0xE4;

        match tp {
            // Foobar & Co company names
            2 => todo!("Foobar & Co company names"),
            // President name
            3 => todo!("President name"),
            // Town name
            6 => TownName::generate(tp - 6, self.id_param),
            _ => unimplemented!("Special string, type {}", tp)
        }
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
