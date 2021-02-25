use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;

use chrono::{Duration};

use crate::{constants::{CONSTANT_DEF, ConstantType}, types::{TokenType}, worker::tools::{get_number, get_text}};
use crate::tokinizer::{TokenInfo};

pub fn duration_parse(fields: &BTreeMap<String, &TokenInfo>) -> core::result::Result<TokenType, String> {
    if (fields.contains_key("duration")) && fields.contains_key("type") {
        let duration = match get_number("duration", fields) {
            Some(number) => number as i64,
            _ => return Err("Duration information not valid".to_string())
        };

        let duration_type = match get_text("type", fields) {
            Some(number) => number,
            _ => return Err("Duration type information not valid".to_string())
        };

        let constant_type = match CONSTANT_DEF.read().unwrap().get_constant("en", &duration_type) {
            Some(constant) => constant,
            None => return Err("Duration type not valid".to_string())
        };

        let calculated_duration = match constant_type {
            ConstantType::Day => Duration::days(duration),
            ConstantType::Month => Duration::days(duration * 30),
            ConstantType::Year => Duration::days(duration * 365),
            ConstantType::Second => Duration::seconds(duration),
            ConstantType::Minute => Duration::minutes(duration),
            ConstantType::Hour => Duration::hours(duration),
            _ => return Err("Duration type not valid".to_string()) 
        };

        return Ok(TokenType::Duration(calculated_duration, duration, constant_type));
    }
    Err("Date type not valid".to_string())
}

#[cfg(test)]
#[test]
fn duration_parse_test_1() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    let tokinizer_mut = setup("10 days".to_string());

    tokinizer_mut.borrow_mut().language_based_tokinize();
    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_infos;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);

    assert_eq!(tokens.len(), 1);
    
    assert_eq!(tokens[0], TokenType::Duration(Duration::days(10), 10, ConstantType::Day));
}

#[cfg(test)]
#[test]
fn duration_parse_test_2() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    let tokinizer_mut = setup("10 years".to_string());

    tokinizer_mut.borrow_mut().language_based_tokinize();
    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_infos;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);

    assert_eq!(tokens.len(), 1);
    
    assert_eq!(tokens[0], TokenType::Duration(Duration::days(10 * 365), 10, ConstantType::Year));
}

#[cfg(test)]
#[test]
fn duration_parse_test_3() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    let tokinizer_mut = setup("60 minutes".to_string());

    tokinizer_mut.borrow_mut().language_based_tokinize();
    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_infos;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);

    assert_eq!(tokens.len(), 1);
    
    assert_eq!(tokens[0], TokenType::Duration(Duration::minutes(60), 60, ConstantType::Minute));
}