use alloc::string::String;
use alloc::vec::Vec;

use alloc::collections::btree_map::BTreeMap;

pub mod rule;
pub mod rules;
pub mod tools;

pub type ItemList     = BTreeMap<String, Vec<String>>;
pub type TypeItem     = BTreeMap<String, ItemList>;
pub type LanguageItem = BTreeMap<String, TypeItem>;
