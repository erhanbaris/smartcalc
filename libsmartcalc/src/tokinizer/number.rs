use alloc::string::ToString;
use alloc::borrow::ToOwned;
use crate::config::SmartCalcConfig;
use crate::types::*;
use crate::tokinizer::{Tokinizer};
use regex::Regex;
use crate::token::ui_token::{UiTokenType};

pub fn number_regex_parser(_: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            /* Check price value */
            let mut number = 0.0;

            if let Some(binary) = capture.name("BINARY") {
                number = i64::from_str_radix(binary.as_str(), 2).unwrap() as f64;
            }
            else if let Some(hex) = capture.name("HEX") { 
                number = i64::from_str_radix(hex.as_str(), 16).unwrap() as f64;
            }
            else if let Some(hex) = capture.name("OCTAL") { 
                number = i64::from_str_radix(hex.as_str(), 8).unwrap() as f64;
            }
            else if let Some(decimal) = capture.name("DECIMAL") {
                number = match decimal.as_str().replace(".", "").replace(",", ".").parse::<f64>() {
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

            if tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), Some(TokenType::Number(number)), capture.get(0).unwrap().as_str().to_string()) {
                tokinizer.ui_tokens.add_from_regex_match(capture.get(0), UiTokenType::Number);
            }
        }
    }
}


#[cfg(test)]
#[test]
fn number_test_1() {
    use crate::tokinizer::test::setup;
    let mut tokinizer_mut = setup("1024 -1024 1024,1 -1024,1".to_string());

    tokinizer_mut.tokinize_with_regex();
    let tokens = &tokinizer_mut.token_infos;

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 4);
    assert_eq!(tokens[0].token_type, Some(TokenType::Number(1024.0)));
    
    assert_eq!(tokens[1].start, 5);
    assert_eq!(tokens[1].end, 10);
    assert_eq!(tokens[1].token_type, Some(TokenType::Number(-1024.0)));
    
    assert_eq!(tokens[2].start, 11);
    assert_eq!(tokens[2].end, 17);
    assert_eq!(tokens[2].token_type, Some(TokenType::Number(1024.1)));
    
    assert_eq!(tokens[3].start, 18);
    assert_eq!(tokens[3].end, 25);
    assert_eq!(tokens[3].token_type, Some(TokenType::Number(-1024.1)));
}

#[cfg(test)]
#[test]
fn number_test_2() {
    use crate::tokinizer::test::setup;
    let mut tokinizer_mut = setup("0x100 0X100 0x1 0X1 0x0 0X0".to_string());

    tokinizer_mut.tokinize_with_regex();
    let tokens = &tokinizer_mut.token_infos;

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 5);
    assert_eq!(tokens[0].token_type, Some(TokenType::Number(256.0)));
    
    assert_eq!(tokens[1].start, 6);
    assert_eq!(tokens[1].end, 11);
    assert_eq!(tokens[1].token_type, Some(TokenType::Number(256.0)));
    
    assert_eq!(tokens[2].start, 12);
    assert_eq!(tokens[2].end, 15);
    assert_eq!(tokens[2].token_type, Some(TokenType::Number(1.0)));
    
    assert_eq!(tokens[3].start, 16);
    assert_eq!(tokens[3].end, 19);
    assert_eq!(tokens[3].token_type, Some(TokenType::Number(1.0)));
    
    assert_eq!(tokens[4].start, 20);
    assert_eq!(tokens[4].end, 23);
    assert_eq!(tokens[4].token_type, Some(TokenType::Number(0.0)));
    
    assert_eq!(tokens[5].start, 24);
    assert_eq!(tokens[5].end, 27);
    assert_eq!(tokens[5].token_type, Some(TokenType::Number(0.0)));
}

#[cfg(test)]
#[test]
fn number_test_3() {
    use crate::tokinizer::test::setup;
    let mut tokinizer_mut = setup("0b100 0B100 0b1 0B1 0b0 0B0".to_string());

    tokinizer_mut.tokinize_with_regex();
    let tokens = &tokinizer_mut.token_infos;

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 5);
    assert_eq!(tokens[0].token_type, Some(TokenType::Number(4.0)));
    
    assert_eq!(tokens[1].start, 6);
    assert_eq!(tokens[1].end, 11);
    assert_eq!(tokens[1].token_type, Some(TokenType::Number(4.0)));
    
    assert_eq!(tokens[2].start, 12);
    assert_eq!(tokens[2].end, 15);
    assert_eq!(tokens[2].token_type, Some(TokenType::Number(1.0)));
    
    assert_eq!(tokens[3].start, 16);
    assert_eq!(tokens[3].end, 19);
    assert_eq!(tokens[3].token_type, Some(TokenType::Number(1.0)));
    
    assert_eq!(tokens[4].start, 20);
    assert_eq!(tokens[4].end, 23);
    assert_eq!(tokens[4].token_type, Some(TokenType::Number(0.0)));
    
    assert_eq!(tokens[5].start, 24);
    assert_eq!(tokens[5].end, 27);
    assert_eq!(tokens[5].token_type, Some(TokenType::Number(0.0)));
}


#[cfg(test)]
#[test]
fn number_test_4() {
    use crate::tokinizer::test::setup;
    let mut tokinizer_mut = setup("0o100 0O100 0o1 0O1 0o0 0O0".to_string());

    tokinizer_mut.tokinize_with_regex();
    let tokens = &tokinizer_mut.token_infos;

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 5);
    assert_eq!(tokens[0].token_type, Some(TokenType::Number(64.0)));
    
    assert_eq!(tokens[1].start, 6);
    assert_eq!(tokens[1].end, 11);
    assert_eq!(tokens[1].token_type, Some(TokenType::Number(64.0)));
    
    assert_eq!(tokens[2].start, 12);
    assert_eq!(tokens[2].end, 15);
    assert_eq!(tokens[2].token_type, Some(TokenType::Number(1.0)));
    
    assert_eq!(tokens[3].start, 16);
    assert_eq!(tokens[3].end, 19);
    assert_eq!(tokens[3].token_type, Some(TokenType::Number(1.0)));
    
    assert_eq!(tokens[4].start, 20);
    assert_eq!(tokens[4].end, 23);
    assert_eq!(tokens[4].token_type, Some(TokenType::Number(0.0)));
    
    assert_eq!(tokens[5].start, 24);
    assert_eq!(tokens[5].end, 27);
    assert_eq!(tokens[5].token_type, Some(TokenType::Number(0.0)));
}
