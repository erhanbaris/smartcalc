/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::string::ToString;
use alloc::borrow::ToOwned;
use crate::config::SmartCalcConfig;
use crate::types::*;
use crate::tokinizer::Tokinizer;
use regex::Regex;
use crate::token::ui_token::{UiTokenType};

pub fn percent_regex_parser(config: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            /* Check price value */
            if tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), Some(TokenType::Percent(capture.name("NUMBER").unwrap().as_str().replace(&config.thousand_separator[..], "").replace(&config.decimal_seperator[..], ".").parse::<f64>().unwrap())), capture.get(0).unwrap().as_str().to_string()) {
                tokinizer.ui_tokens.add_from_regex_match(capture.name("NUMBER"), UiTokenType::Number);
                tokinizer.ui_tokens.add_from_regex_match(capture.name("PERCENT"), UiTokenType::Symbol2);
            }
        }
    }
}

#[cfg(test)]
#[test]
fn percent_test() {
    use core::ops::Deref;
    use crate::tokinizer::regex_tokinizer;
    use crate::tokinizer::test::setup_tokinizer;
    use core::cell::RefCell;
    use crate::config::SmartCalcConfig;
    use crate::session::Session;
    let session = RefCell::new(Session::new());
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("%10 %-1 50% -55% %10,1 %-1,3 50,5% -55,9%".to_string(), &session, &config);

    regex_tokinizer(&mut tokinizer_mut);
    let tokens = &tokinizer_mut.session.borrow().token_infos;

    assert_eq!(tokens.len(), 8);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 3);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Percent(10.0)));

    assert_eq!(tokens[1].start, 4);
    assert_eq!(tokens[1].end, 7);
    assert_eq!(tokens[1].token_type.borrow().deref(), &Some(TokenType::Percent(-1.0)));

    assert_eq!(tokens[2].start, 8);
    assert_eq!(tokens[2].end, 11);
    assert_eq!(tokens[2].token_type.borrow().deref(), &Some(TokenType::Percent(50.0)));

    assert_eq!(tokens[3].start, 12);
    assert_eq!(tokens[3].end, 16);
    assert_eq!(tokens[3].token_type.borrow().deref(), &Some(TokenType::Percent(-55.0)));

    assert_eq!(tokens[4].start, 17);
    assert_eq!(tokens[4].end, 22);
    assert_eq!(tokens[4].token_type.borrow().deref(), &Some(TokenType::Percent(10.1)));

    assert_eq!(tokens[5].start, 23);
    assert_eq!(tokens[5].end, 28);
    assert_eq!(tokens[5].token_type.borrow().deref(), &Some(TokenType::Percent(-1.3)));

    assert_eq!(tokens[6].start, 29);
    assert_eq!(tokens[6].end, 34);
    assert_eq!(tokens[6].token_type.borrow().deref(), &Some(TokenType::Percent(50.5)));

    assert_eq!(tokens[7].start, 35);
    assert_eq!(tokens[7].end, 41);
    assert_eq!(tokens[7].token_type.borrow().deref(), &Some(TokenType::Percent(-55.9)));
}
