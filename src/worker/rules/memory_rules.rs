/*
 * smartcalc v1.0.3
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;
use alloc::sync::Arc;

use crate::config::SmartCalcConfig;
use crate::types::MemoryType;
use crate::worker::tools::get_memory;
use crate::worker::tools::get_text;
use crate::{tokinizer::Tokinizer, types::{TokenType}};
use crate::tokinizer::{TokenInfo};

pub fn memory_convert(_: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Arc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("memory") && fields.contains_key("memory_type") {
        let target_memory_type = match get_text("memory_type", &fields) {
            Some(text) => match MemoryType::from(&text.to_lowercase()[..]) {
                Some(memory_type) => memory_type,
                None => return Err("Target memory type not valid".to_string())
            },
            None => return Err("Target memory type not valid".to_string())
        };
        let (memory, memory_type) = get_memory("memory", &fields).unwrap();

        let difference = (target_memory_type.clone() as isize - memory_type.clone() as isize).abs();
        let div_info = 1024.0_f64.powf(difference as f64);
        let new_size = match target_memory_type.clone() as isize > memory_type as isize {
            true => memory / div_info,
            false => memory * div_info
        };

        return Ok(TokenType::Memory(new_size, target_memory_type.clone()));
    }

    Err("Memory type not valid".to_string())
}

#[cfg(test)]
#[test]
fn number_on_1() {
    use core::ops::Deref;
    use crate::{tokinizer::test::execute, types::NumberType};
    
    let tokens = execute("6% on 40".to_string());
    
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(42.4, NumberType::Decimal)));
}


#[cfg(test)]
#[test]
fn number_of_1() {
    use core::ops::Deref;
    use crate::{tokinizer::test::execute, types::NumberType};
    
    let tokens = execute("6% of 40".to_string());

    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(2.4, NumberType::Decimal)));
}


#[cfg(test)]
#[test]
fn number_off_1() {
    use core::ops::Deref;
    use crate::{tokinizer::test::execute, types::NumberType};
    
    let tokens = execute("6% off 40".to_string());

    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(37.6, NumberType::Decimal)));
}
