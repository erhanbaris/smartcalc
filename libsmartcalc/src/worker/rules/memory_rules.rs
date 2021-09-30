use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;
use alloc::sync::Arc;

use crate::config::SmartCalcConfig;
use crate::{tokinizer::Tokinizer, types::{TokenType}};
use crate::tokinizer::{TokenInfo};

pub fn memory_convert(_: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Arc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("memory") && fields.contains_key("memory_type_1") {

        let re = regex::Regex::new("\\b\\b").unwrap();
        return Err("Memory information not valid".to_string())
    }

    Err("Memory type not valid".to_string())
}

#[cfg(test)]
#[test]
fn number_on_1() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("6% on 40".to_string());
    
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(42.4)));
}


#[cfg(test)]
#[test]
fn number_of_1() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("6% of 40".to_string());

    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(2.4)));
}


#[cfg(test)]
#[test]
fn number_off_1() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("6% off 40".to_string());

    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(37.6)));
}
