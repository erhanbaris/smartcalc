/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;
use alloc::sync::Arc;

use crate::config::SmartCalcConfig;
use crate::compiler::dynamic_type::DynamicTypeItem;
use crate::worker::tools::get_text;
use crate::worker::tools::get_dynamic_type;
use crate::{tokinizer::Tokinizer, types::{TokenType}};
use crate::tokinizer::{TokenInfo};

pub fn dynamic_type_convert(config: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Arc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("type") && fields.contains_key("target") {
        let target_type = get_text("target", &fields).unwrap();
        let (number, source_type) = get_dynamic_type("type", &fields).unwrap();
        
        match DynamicTypeItem::convert(config, number, source_type.clone(), target_type) {
            Some((new_number, new_type)) => return Ok(TokenType::DynamicType(new_number, new_type.clone())),
            None => ()
        };
    }

    Err("Dynamic type not valid".to_string())
}
