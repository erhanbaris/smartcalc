use alloc::string::ToString;
use alloc::borrow::ToOwned;
use crate::config::SmartCalcConfig;
use crate::types::*;
use crate::tokinizer::Tokinizer;
use regex::Regex;
use crate::token::ui_token::{UiTokenType};

pub fn memory_regex_parser(_: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned().to_lowercase()) {
            let memory_type = match MemoryType::from(&capture.name("TYPE").unwrap().as_str().to_lowercase()[..]) {
                Some(memory_type) => memory_type,
                None =>MemoryType::Byte
            };
            
            if tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), Some(TokenType::Memory(capture.name("NUMBER").unwrap().as_str().replace(",", ".").parse::<f64>().unwrap(), memory_type)), capture.get(0).unwrap().as_str().to_string()) {
                tokinizer.ui_tokens.add_from_regex_match(capture.name("NUMBER"), UiTokenType::Number);
                tokinizer.ui_tokens.add_from_regex_match(capture.name("FULL_TYPE"), UiTokenType::Text);
            }
        }
    }
}