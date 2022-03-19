/*
 * smartcalc v1.0.7
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;

use crate::config::SmartCalcConfig;
use crate::compiler::dynamic_type::DynamicTypeItem;
use crate::tokinizer::get_dynamic_type;
use crate::tokinizer::get_text;
use crate::{tokinizer::Tokinizer, types::{TokenType}};
use crate::tokinizer::{TokenInfo};

pub fn dynamic_type_convert(config: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Rc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("source") && fields.contains_key("type") {
        let target_type = get_text("type", fields).unwrap();
        let (number, source_type) = get_dynamic_type("source", fields).unwrap();
        
        if let Some((new_number, new_type)) = DynamicTypeItem::convert(config, number, source_type, target_type) {
            return Ok(TokenType::DynamicType(new_number, new_type))
        };
    }

    Err("Dynamic type not valid".to_string())
}
