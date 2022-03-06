/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::config::SmartCalcConfig;
use crate::config::DynamicType;
use crate::worker::tools::get_text;
use crate::worker::tools::get_dynamic_type;
use crate::{tokinizer::Tokinizer, types::{TokenType}};
use crate::tokinizer::{TokenInfo};

pub fn dynamic_type_convert(config: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Arc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("type") && fields.contains_key("target") {
        let target = get_text("target", &fields).unwrap();
        let (number, dynamic_type) = get_dynamic_type("type", &fields).unwrap();
        
        let group = config.types.get(&dynamic_type.group_name).unwrap();
        let values: Vec<Arc<DynamicType>> = group.values().cloned().collect();
        
        if let Some(target) = values.iter().find(|&s| s.names.contains(&target)) {                        
            let mut search_index = match dynamic_type.index > target.index {
                true => dynamic_type.index - 1,
                false => dynamic_type.index + 1
            };
            
            let mut multiplier: f64 = 1.0;

            loop {
                let next_item = group.get(&search_index).unwrap();
                multiplier *= next_item.multiplier;
                if next_item.index == target.index {
                    break;
                }
                
                search_index = match dynamic_type.index > target.index {
                    true => search_index - 1,
                    false => search_index + 1
                };
            }

            let new_number = match dynamic_type.index > target.index {
                true => number * multiplier,
                false => number / multiplier
            };

            return Ok(TokenType::DynamicType(new_number, target.clone()))
        }
    }

    Err("Dynamic type not valid".to_string())
}
