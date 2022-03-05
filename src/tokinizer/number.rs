/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::string::ToString;
use alloc::borrow::ToOwned;
use crate::config::SmartCalcConfig;
use crate::types::*;
use crate::tokinizer::{Tokinizer};
use regex::Regex;
use crate::token::ui_token::{UiTokenType};

pub fn number_regex_parser(config: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            /* Check price value */
            let mut number = 0.0;
            let mut number_type = NumberType::Decimal;

            if let Some(binary) = capture.name("BINARY") {
                number = i64::from_str_radix(binary.as_str(), 2).unwrap() as f64;
                number_type = NumberType::Binary;
            }
            else if let Some(hex) = capture.name("HEX") { 
                number = i64::from_str_radix(hex.as_str(), 16).unwrap() as f64;
                number_type = NumberType::Hexadecimal;
            }
            else if let Some(octal) = capture.name("OCTAL") { 
                number = i64::from_str_radix(octal.as_str(), 8).unwrap() as f64;
                number_type = NumberType::Octal;
            }
            else if let Some(decimal) = capture.name("DECIMAL") {
                number = match decimal.as_str().replace(&config.thousand_separator[..], "").replace(&config.decimal_seperator[..], ".").parse::<f64>() {
                    Ok(num) => {
                        match capture.name("NOTATION") {
                            Some(notation) => num * match notation.as_str() {
                                "k" | "K" => 1_000.0,
                                "M" => 1_000_000.0,
                                "G" => 1_000_000_000.0,
                                "T" => 1_000_000_000_000.0,
                                "P" => 1_000_000_000_000_000.0,
                                "Z" => 1_000_000_000_000_000_000.0,
                                "Y" => 1_000_000_000_000_000_000_000.0,
                                _ => 1.0
                            },
                            _ => num
                        }
                    },
                    _ => continue
                };
            }

            if tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), Some(TokenType::Number(number, number_type)), capture.get(0).unwrap().as_str().to_string()) {
                tokinizer.ui_tokens.add_from_regex_match(capture.get(0), UiTokenType::Number);
            }
        }
    }
}


#[cfg(test)]
#[test]
fn number_test_1() {
    use core::ops::Deref;
    use crate::tokinizer::test::setup_tokinizer;
    use core::cell::RefCell;
    use crate::config::SmartCalcConfig;
    use crate::app::Session;
    let session = RefCell::new(Session::new());
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("1024 -1024 1024,1 -1024,1".to_string(), &session, &config);

    tokinizer_mut.tokinize_with_regex();
    let tokens = &tokinizer_mut.session.borrow().token_infos;

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 4);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(1024.0, NumberType::Decimal)));
    
    assert_eq!(tokens[1].start, 5);
    assert_eq!(tokens[1].end, 10);
    assert_eq!(tokens[1].token_type.borrow().deref(), &Some(TokenType::Number(-1024.0, NumberType::Decimal)));
    
    assert_eq!(tokens[2].start, 11);
    assert_eq!(tokens[2].end, 17);
    assert_eq!(tokens[2].token_type.borrow().deref(), &Some(TokenType::Number(1024.1, NumberType::Decimal)));
    
    assert_eq!(tokens[3].start, 18);
    assert_eq!(tokens[3].end, 25);
    assert_eq!(tokens[3].token_type.borrow().deref(), &Some(TokenType::Number(-1024.1, NumberType::Decimal)));
}

#[cfg(test)]
#[test]
fn number_test_2() {
    use core::ops::Deref;
    use crate::tokinizer::test::setup_tokinizer;
    use core::cell::RefCell;
    use crate::config::SmartCalcConfig;
    use crate::app::Session;
    let session = RefCell::new(Session::new());
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("0x100 0X100 0x1 0X1 0x0 0X0".to_string(), &session, &config);

    tokinizer_mut.tokinize_with_regex();
    let tokens = &tokinizer_mut.session.borrow().token_infos;

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 5);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(256.0, NumberType::Hexadecimal)));
    
    assert_eq!(tokens[1].start, 6);
    assert_eq!(tokens[1].end, 11);
    assert_eq!(tokens[1].token_type.borrow().deref(), &Some(TokenType::Number(256.0, NumberType::Hexadecimal)));
    
    assert_eq!(tokens[2].start, 12);
    assert_eq!(tokens[2].end, 15);
    assert_eq!(tokens[2].token_type.borrow().deref(), &Some(TokenType::Number(1.0, NumberType::Hexadecimal)));
    
    assert_eq!(tokens[3].start, 16);
    assert_eq!(tokens[3].end, 19);
    assert_eq!(tokens[3].token_type.borrow().deref(), &Some(TokenType::Number(1.0, NumberType::Hexadecimal)));
    
    assert_eq!(tokens[4].start, 20);
    assert_eq!(tokens[4].end, 23);
    assert_eq!(tokens[4].token_type.borrow().deref(), &Some(TokenType::Number(0.0, NumberType::Hexadecimal)));
    
    assert_eq!(tokens[5].start, 24);
    assert_eq!(tokens[5].end, 27);
    assert_eq!(tokens[5].token_type.borrow().deref(), &Some(TokenType::Number(0.0, NumberType::Hexadecimal)));
}

#[cfg(test)]
#[test]
fn number_test_3() {
    use core::ops::Deref;
    use crate::tokinizer::test::setup_tokinizer;
    use core::cell::RefCell;
    use crate::config::SmartCalcConfig;
    use crate::app::Session;
    let session = RefCell::new(Session::new());
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("0b100 0B100 0b1 0B1 0b0 0B0".to_string(), &session, &config);

    tokinizer_mut.tokinize_with_regex();
    let tokens = &tokinizer_mut.session.borrow().token_infos;

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 5);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(4.0, NumberType::Binary)));
    
    assert_eq!(tokens[1].start, 6);
    assert_eq!(tokens[1].end, 11);
    assert_eq!(tokens[1].token_type.borrow().deref(), &Some(TokenType::Number(4.0, NumberType::Binary)));
    
    assert_eq!(tokens[2].start, 12);
    assert_eq!(tokens[2].end, 15);
    assert_eq!(tokens[2].token_type.borrow().deref(), &Some(TokenType::Number(1.0, NumberType::Binary)));
    
    assert_eq!(tokens[3].start, 16);
    assert_eq!(tokens[3].end, 19);
    assert_eq!(tokens[3].token_type.borrow().deref(), &Some(TokenType::Number(1.0, NumberType::Binary)));
    
    assert_eq!(tokens[4].start, 20);
    assert_eq!(tokens[4].end, 23);
    assert_eq!(tokens[4].token_type.borrow().deref(), &Some(TokenType::Number(0.0, NumberType::Binary)));
    
    assert_eq!(tokens[5].start, 24);
    assert_eq!(tokens[5].end, 27);
    assert_eq!(tokens[5].token_type.borrow().deref(), &Some(TokenType::Number(0.0, NumberType::Binary)));
}


#[cfg(test)]
#[test]
fn number_test_4() {
    use core::ops::Deref;
    use crate::tokinizer::test::setup_tokinizer;
    use core::cell::RefCell;
    use crate::config::SmartCalcConfig;
    use crate::app::Session;
    let session = RefCell::new(Session::new());
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("0o100 0O100 0o1 0O1 0o0 0O0".to_string(), &session, &config);

    tokinizer_mut.tokinize_with_regex();
    let tokens = &tokinizer_mut.session.borrow().token_infos;

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 5);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Number(64.0, NumberType::Octal)));
    
    assert_eq!(tokens[1].start, 6);
    assert_eq!(tokens[1].end, 11);
    assert_eq!(tokens[1].token_type.borrow().deref(), &Some(TokenType::Number(64.0, NumberType::Octal)));
    
    assert_eq!(tokens[2].start, 12);
    assert_eq!(tokens[2].end, 15);
    assert_eq!(tokens[2].token_type.borrow().deref(), &Some(TokenType::Number(1.0, NumberType::Octal)));
    
    assert_eq!(tokens[3].start, 16);
    assert_eq!(tokens[3].end, 19);
    assert_eq!(tokens[3].token_type.borrow().deref(), &Some(TokenType::Number(1.0, NumberType::Octal)));
    
    assert_eq!(tokens[4].start, 20);
    assert_eq!(tokens[4].end, 23);
    assert_eq!(tokens[4].token_type.borrow().deref(), &Some(TokenType::Number(0.0, NumberType::Octal)));
    
    assert_eq!(tokens[5].start, 24);
    assert_eq!(tokens[5].end, 27);
    assert_eq!(tokens[5].token_type.borrow().deref(), &Some(TokenType::Number(0.0, NumberType::Octal)));
}
