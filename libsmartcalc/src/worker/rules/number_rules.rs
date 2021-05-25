use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;

use crate::config::SmartCalcConfig;
use crate::{tokinizer::Tokinizer, types::{TokenType}};
use crate::tokinizer::{TokenInfo};

use crate::worker::tools::{get_number_or_price, get_percent, get_currency};

pub fn number_on(config: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, &TokenInfo>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("number") && fields.contains_key("p") {
        let number = match get_number_or_price(config, "number", fields) {
            Some(number) => number,
            _ => return Err("Number information not valid".to_string())
        };

        let percent = match get_percent("p", fields) {
            Some(percent) => percent,
            _ => return Err("Percent information not valid".to_string())
        };

        let calculated_number = number + ((number * percent) / 100.0);
        return Ok(match get_currency(config, "number", fields) {
            Some(currency) => TokenType::Money(calculated_number, currency.to_string()),
            None => TokenType::Number(calculated_number)
        });
    }

    Err("Number type not valid".to_string())
}


pub fn number_of(config: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, &TokenInfo>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("number") && fields.contains_key("p") {
        let number = match get_number_or_price(config, "number", fields) {
            Some(number) => number,
            _ => return Err("Number information not valid".to_string())
        };

        let percent = match get_percent("p", fields) {
            Some(percent) => percent,
            _ => return Err("Percent information not valid".to_string())
        };

        let calculated_number = (number * percent) / 100.0;
        return Ok(match get_currency(config, "number", fields) {
            Some(currency) => TokenType::Money(calculated_number, currency.to_string()),
            None => TokenType::Number(calculated_number)
        });
    }

    Err("Number type not valid".to_string())
}


pub fn number_off(config: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, &TokenInfo>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("number") && fields.contains_key("p") {
        let number = match get_number_or_price(config, "number", fields) {
            Some(number) => number,
            _ => return Err("Number information not valid".to_string())
        };

        let percent = match get_percent("p", fields) {
            Some(percent) => percent,
            _ => return Err("Percent information not valid".to_string())
        };

        let calculated_number = number - ((number * percent) / 100.0);
        return Ok(match get_currency(config, "number", fields) {
            Some(currency) => TokenType::Money(calculated_number, currency.to_string()),
            None => TokenType::Number(calculated_number)
        });
    }

    Err("Number type not valid".to_string())
}

#[cfg(test)]
#[test]
fn number_on_1() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    let tokinizer_mut = setup("6% on 40".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_infos;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);
    
    assert_eq!(tokens[0], TokenType::Number(42.4));
}


#[cfg(test)]
#[test]
fn number_of_1() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    let tokinizer_mut = setup("6% of 40".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_infos;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);
    
    assert_eq!(tokens[0], TokenType::Number(2.4));
}


#[cfg(test)]
#[test]
fn number_off_1() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    let tokinizer_mut = setup("6% off 40".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_infos;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);
    
    assert_eq!(tokens[0], TokenType::Number(37.6));
}
