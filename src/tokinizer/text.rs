/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::string::ToString;
use alloc::borrow::ToOwned;
use chrono::{Duration, Local};
use crate::config::SmartCalcConfig;
use crate::types::{TokenType};
use crate::tokinizer::Tokinizer;
use crate::token::ui_token::{UiTokenType};
use regex::{Regex};
use crate::worker::tools::{read_currency, get_timezone};
use crate::constants::ConstantType;

pub fn text_regex_parser(config: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            let text = capture.name("TEXT").unwrap().as_str();
            if !text.trim().is_empty() {

                if let Some(constant) = config.constant_pair.get(&tokinizer.language).unwrap().get(&text.to_string()) {

                    let token = match constant {
                        ConstantType::Today     => Some(TokenType::Date(Local::today().naive_local(), get_timezone())),
                        ConstantType::Tomorrow  => Some(TokenType::Date(Local::today().naive_local() + Duration::days(1), get_timezone())),
                        ConstantType::Yesterday => Some(TokenType::Date(Local::today().naive_local() + Duration::days(-1), get_timezone())),
                        ConstantType::Now       => Some(TokenType::Time(Local::now().naive_local(), get_timezone())),
                        _ => None
                    };

                    if token.is_some() && tokinizer.add_token(&capture.get(0), token) {
                        tokinizer.ui_tokens.add_from_regex_match(capture.get(0), UiTokenType::Time);
                    }
                }

                if tokinizer.add_token(&capture.get(0), Some(TokenType::Text(text.to_string()))) {
                    match read_currency(config, text) {
                        Some(_) => tokinizer.ui_tokens.add_from_regex_match(capture.get(0), UiTokenType::MoneySymbol),
                        _ => tokinizer.ui_tokens.add_from_regex_match(capture.get(0), UiTokenType::Text)
                    };
                }
            }
        }
    }
}



#[cfg(test)]
#[test]
fn text_test() {
    use core::ops::Deref;
    use crate::tokinizer::test::setup_tokinizer;
    use core::cell::RefCell;
    use crate::config::SmartCalcConfig;
    use crate::app::Session;
    let session = RefCell::new(Session::new());
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("erhan barış aysel barış test".to_string(), &session, &config);

    tokinizer_mut.tokinize_with_regex();
    let tokens = &tokinizer_mut.session.borrow().token_infos;

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 5);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Text("erhan".to_string())));

    assert_eq!(tokens[1].start, 6);
    assert_eq!(tokens[1].end, 13);
    assert_eq!(tokens[1].token_type.borrow().deref(), &Some(TokenType::Text("barış".to_string())));

    assert_eq!(tokens[2].start, 14);
    assert_eq!(tokens[2].end, 19);
    assert_eq!(tokens[2].token_type.borrow().deref(), &Some(TokenType::Text("aysel".to_string())));

    assert_eq!(tokens[3].start, 20);
    assert_eq!(tokens[3].end, 27);
    assert_eq!(tokens[3].token_type.borrow().deref(), &Some(TokenType::Text("barış".to_string())));

    assert_eq!(tokens[4].start, 28);
    assert_eq!(tokens[4].end, 32);
    assert_eq!(tokens[4].token_type.borrow().deref(), &Some(TokenType::Text("test".to_string())));
}

