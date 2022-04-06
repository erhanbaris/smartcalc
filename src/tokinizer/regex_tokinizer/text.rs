/*
 * smartcalc v1.0.8
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::string::ToString;
use alloc::borrow::ToOwned;
use chrono::{Duration, Utc};
use crate::config::SmartCalcConfig;
use crate::types::{TokenType};
use crate::tokinizer::{Tokinizer, read_currency};
use crate::token::ui_token::{UiTokenType};
use regex::{Regex};
use crate::constants::ConstantType;

pub fn text_regex_parser(config: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            let text = capture.name("TEXT").unwrap().as_str();
            if !text.trim().is_empty() {

                if let Some(constant) = config.constant_pair.get(&tokinizer.language).unwrap().get(&text.to_string()) {

                    let token = match constant {
                        ConstantType::Today     => Some(TokenType::Date(Utc::today().naive_utc(), config.get_time_offset())),
                        ConstantType::Tomorrow  => Some(TokenType::Date(Utc::today().naive_utc() + Duration::days(1), config.get_time_offset())),
                        ConstantType::Yesterday => Some(TokenType::Date(Utc::today().naive_utc() + Duration::days(-1), config.get_time_offset())),
                        ConstantType::Now       => Some(TokenType::Time(Utc::now().naive_utc(), config.get_time_offset())),
                        _ => None
                    };

                    if token.is_some() && tokinizer.add_token_from_match(&capture.get(0), token) {
                        tokinizer.add_uitoken_from_match(capture.get(0), UiTokenType::DateTime);
                    }
                }

                if tokinizer.add_token_from_match(&capture.get(0), Some(TokenType::Text(text.to_string()))) {
                    match read_currency(config, text) {
                        Some(_) => tokinizer.add_uitoken_from_match(capture.get(0), UiTokenType::Symbol1),
                        _ => tokinizer.add_uitoken_from_match(capture.get(0), UiTokenType::Text)
                    };
                }
            }
        }
    }
}

#[cfg(test)]
#[test]
fn text_test_1() {
    use core::ops::Deref;
    use crate::tokinizer::regex_tokinizer;
    use crate::tokinizer::test::setup_tokinizer;
    use crate::config::SmartCalcConfig;
    use crate::session::Session;
    let mut session = Session::new();
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("erhan barış aysel barış test".to_string(), &mut session, &config);

    regex_tokinizer(&mut tokinizer_mut);
    let tokens = &tokinizer_mut.token_infos;

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

#[cfg(test)]
#[test]
fn text_test_2() {
    use core::ops::Deref;
    use crate::tokinizer::regex_tokinizer;
    use crate::tokinizer::test::setup_tokinizer;
    use crate::config::SmartCalcConfig;
    use crate::session::Session;
    let mut session = Session::new();
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("today now yesterday tomorrow".to_string(), &mut session, &config);

    regex_tokinizer(&mut tokinizer_mut);
    let tokens = &tokinizer_mut.token_infos;

    let today = Utc::today().naive_utc();
    let tomorrow = Utc::today().naive_utc() + Duration::days(1);
    let yesterday = Utc::today().naive_utc() + Duration::days(-1);
    let now = Utc::now().naive_utc();

    assert_eq!(tokens.len(), 4);

    /*Today */
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 5);
    let token = tokens[0].token_type.borrow().deref().clone();

    if let Some(TokenType::Date(calculated_today, offset)) = token {
        assert_eq!(offset, config.get_time_offset());
        assert!(today.signed_duration_since(calculated_today).num_seconds().abs() < 5);
    } else { assert!(false); }

    /*Now */
    assert_eq!(tokens[1].start, 6);
    assert_eq!(tokens[1].end, 9);
    let token = tokens[1].token_type.borrow().deref().clone();

    if let Some(TokenType::Time(calculated_now, offset)) = token {
        assert_eq!(offset, config.get_time_offset());
        assert!(now.signed_duration_since(calculated_now).num_seconds().abs() < 5);
    } else { assert!(false); }

    /*Yesterday */
    assert_eq!(tokens[2].start, 10);
    assert_eq!(tokens[2].end, 19);
    let token = tokens[2].token_type.borrow().deref().clone();

    if let Some(TokenType::Date(calculated_yesterday, offset)) = token {
        assert_eq!(offset, config.get_time_offset());
        assert!(yesterday.signed_duration_since(calculated_yesterday).num_seconds().abs() < 5);
    } else { assert!(false); }

    /*Tomorrow */
    assert_eq!(tokens[3].start, 20);
    assert_eq!(tokens[3].end, 28);
    let token = tokens[3].token_type.borrow().deref().clone();

    if let Some(TokenType::Date(calculated_tomorrow, offset)) = token {
        assert_eq!(offset, config.get_time_offset());
        assert!(tomorrow.signed_duration_since(calculated_tomorrow).num_seconds().abs() < 5);
    } else { assert!(false); }
}

