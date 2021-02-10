use std::vec::Vec;

use std::collections::HashMap;

pub mod rule;
mod rules;

pub type ItemList     = HashMap<String, Vec<String>>;
pub type TypeItem     = HashMap<String, ItemList>;
pub type LanguageItem = HashMap<String, TypeItem>;
