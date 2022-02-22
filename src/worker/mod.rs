/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::string::String;
use alloc::vec::Vec;

use alloc::collections::btree_map::BTreeMap;

pub mod rule;
pub mod rules;
pub mod tools;

pub type ItemList     = BTreeMap<String, Vec<String>>;
pub type TypeItem     = BTreeMap<String, ItemList>;
pub type LanguageItem = BTreeMap<String, TypeItem>;
