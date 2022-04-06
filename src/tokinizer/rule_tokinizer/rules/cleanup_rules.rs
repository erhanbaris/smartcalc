/*
 * smartcalc v1.0.8
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;
use core::ops::Deref;

use crate::config::SmartCalcConfig;
use crate::{tokinizer::Tokinizer, types::{TokenType}};
use crate::tokinizer::{TokenInfo};
use crate::{types::{SmartCalcAstType}};

pub fn division_cleanup(_: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Rc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if (fields.contains_key("data")) && fields.contains_key("text") {
        return match &fields.get(&"data".to_string()).unwrap().token_type.borrow().deref()  {
            Some(token) => match &token {
                TokenType::Number(number, number_type) => Ok(TokenType::Number(*number, *number_type)),
                TokenType::Percent(percent) => Ok(TokenType::Percent(*percent)),
                TokenType::Money(price, currency) => Ok(TokenType::Money(*price, currency.clone())),
                TokenType::Variable(variable) => {
                    match variable.data.borrow().deref().deref() {
                        SmartCalcAstType::Item(item) => Ok(item.as_token_type()),
                        _ => Err("Data type not valid".to_string())
                    }
                },
                _ => Err("Data type not valid".to_string())
            },
            _ => Err("Data type not valid".to_string())
        }
    }
    Err("Data type not valid".to_string())
}


#[cfg(test)]
#[test]
fn cleanup_rules() {
    use chrono::Duration;
    use crate::types::{TokenType};
    use crate::config::SmartCalcConfig;
    use crate::tokinizer::test::get_executed_raw_tokens;
    
    let tokens = get_executed_raw_tokens("$25/hour * 14 hours of work".to_string());
    let conf = SmartCalcConfig::default();
    assert_eq!(tokens.len(), 3);
    
    assert_eq!(*tokens[0], TokenType::Money(25.0, conf.get_currency("usd".to_string()).unwrap()));
    assert_eq!(*tokens[1], TokenType::Operator('*'));
    assert_eq!(*tokens[2], TokenType::Duration(Duration::hours(14)));
}
