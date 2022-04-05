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
use crate::tokinizer::get_currency;
use crate::tokinizer::get_number;
use crate::tokinizer::get_number_or_price;
use crate::tokinizer::get_percent;
use crate::tokinizer::get_text;
use crate::types::NumberType;
use crate::{tokinizer::Tokinizer, types::{TokenType}};
use crate::tokinizer::{TokenInfo};
use crate::tools::do_divition;

pub fn number_on(config: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Rc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("number") && fields.contains_key("p") {
        let number = match get_number_or_price(config, "number", fields) {
            Some(number) => number,
            _ => return Err("Number information not valid".to_string())
        };

        let percent = match get_percent("p", fields) {
            Some(percent) => percent,
            _ => return Err("Percent information not valid".to_string())
        };

        let calculated_number = number + do_divition(number * percent, 100.0);
        return Ok(match get_currency(config, "number", fields) {
            Some(currency) => TokenType::Money(calculated_number, currency),
            None => TokenType::Number(calculated_number, NumberType::Decimal)
        });
    }

    Err("Number type not valid".to_string())
}


pub fn number_of(config: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Rc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("number") && fields.contains_key("p") {
        let number = match get_number_or_price(config, "number", fields) {
            Some(number) => number,
            _ => return Err("Number information not valid".to_string())
        };

        let percent = match get_percent("p", fields) {
            Some(percent) => percent,
            _ => return Err("Percent information not valid".to_string())
        };

        let calculated_number = do_divition(number * percent, 100.0);
        return Ok(match get_currency(config, "number", fields) {
            Some(currency) => TokenType::Money(calculated_number, currency),
            None => TokenType::Number(calculated_number, NumberType::Decimal)
        });
    }

    Err("Number type not valid".to_string())
}


pub fn number_off(config: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Rc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("number") && fields.contains_key("p") {
        let number = match get_number_or_price(config, "number", fields) {
            Some(number) => number,
            _ => return Err("Number information not valid".to_string())
        };

        let percent = match get_percent("p", fields) {
            Some(percent) => percent,
            _ => return Err("Percent information not valid".to_string())
        };

        let calculated_number = number - do_divition(number * percent, 100.0);
        return Ok(match get_currency(config, "number", fields) {
            Some(currency) => TokenType::Money(calculated_number, currency),
            None => TokenType::Number(calculated_number, NumberType::Decimal)
        });
    }

    Err("Number type not valid".to_string())
}

pub fn number_type_convert(_: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Rc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("number") && fields.contains_key("type") {
        let number = match get_number("number", fields) {
            Some(number) => number.round(),
            None => return Err("Number type not valid".to_string())
        };
        let number_type = match get_text("type", fields) {
            Some(text) => text,
            None => return Err("Number type not valid".to_string())
        };
        
        let number_type = match &number_type[..] {
            "hex" | "hexadecimal" => NumberType::Hexadecimal,
            "octal"               => NumberType::Octal,
            "binary"              => NumberType::Binary,
            "decimal"             => NumberType::Decimal,
            _ => return Err("Target number type not valid".to_string())
        };

        return Ok(TokenType::Number(number, number_type));
    }

    Err("Number type not valid".to_string())
}

#[cfg(test)]
#[test]
fn number_on_1() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("6% on 40".to_string());
    
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(42.4, NumberType::Decimal)));
}


#[cfg(test)]
#[test]
fn number_of_1() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("6% of 40".to_string());

    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(2.4, NumberType::Decimal)));
}


#[cfg(test)]
#[test]
fn number_off_1() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("6% off 40".to_string());

    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(37.6, NumberType::Decimal)));
}

#[cfg(test)]
#[test]
fn number_type_convert_1() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;

    let tokens = execute("100 to hex".to_string());
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(100.0, NumberType::Hexadecimal)));
}

#[cfg(test)]
#[test]
fn number_type_convert_2() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("100,0 to hex".to_string());
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(100.0, NumberType::Hexadecimal)));
}

#[cfg(test)]
#[test]
fn number_type_convert_3() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("100,0 to hexadecimal".to_string());
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(100.0, NumberType::Hexadecimal)));
}

#[cfg(test)]
#[test]
fn number_type_convert_4() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("100,0 to octal".to_string());
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(100.0, NumberType::Octal)));
}

#[cfg(test)]
#[test]
fn number_type_convert_5() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("100,0 to oct".to_string());
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(100.0, NumberType::Octal)));
}

#[cfg(test)]
#[test]
fn number_type_convert_6() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("100,0 to bin".to_string());
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(100.0, NumberType::Binary)));
}

#[cfg(test)]
#[test]
fn number_type_convert_7() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("100,0 to binary".to_string());
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(100.0, NumberType::Binary)));
}
