/*
 * smartcalc v1.0.8
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::borrow::ToOwned;
use crate::config::SmartCalcConfig;
use crate::types::*;
use crate::tokinizer::Tokinizer;
use regex::Regex;
use crate::token::ui_token::{UiTokenType};
use crate::tools::parse_timezone;

pub fn timezone_regex_parser(config: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned().to_uppercase()) {
            if let Some((timezone, offset)) = parse_timezone(config, &capture) {
                if tokinizer.add_token_from_match(&capture.get(0), Some(TokenType::Timezone(timezone, offset))) {
                    tokinizer.add_uitoken_from_match(capture.name("timezone"), UiTokenType::Symbol1);
                }
            };
        }
    }
}

#[cfg(test)]
mod test {
    use core::ops::Deref;
    use alloc::string::ToString;
    use crate::tokinizer::test::setup_tokinizer;
    use crate::tokinizer::{TokenType, regex_tokinizer};
    use crate::config::SmartCalcConfig;
    use crate::session::Session;
    
    #[test]
    fn timezone_test_1() {
        let mut session = Session::new();
        let config = SmartCalcConfig::default();
        let mut tokinizer_mut = setup_tokinizer("GMT EST GMT+10:00".to_string(), &mut session, &config);

        regex_tokinizer(&mut tokinizer_mut);
        let tokens = &tokinizer_mut.token_infos;

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].start, 0);
        assert_eq!(tokens[0].end, 3);
        assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Timezone("GMT".to_string(), 0)));

        assert_eq!(tokens[1].start, 4);
        assert_eq!(tokens[1].end, 7);
        assert_eq!(tokens[1].token_type.borrow().deref(), &Some(TokenType::Timezone("EST".to_string(), -300)));

        assert_eq!(tokens[2].start, 8);
        assert_eq!(tokens[2].end, 17);
        assert_eq!(tokens[2].token_type.borrow().deref(), &Some(TokenType::Timezone("GMT+10:00".to_string(), 600)));
    }
    
    #[test]
    fn timezone_test_2() {
        let mut session = Session::new();
        let config = SmartCalcConfig::default();
        let mut tokinizer_mut = setup_tokinizer("GMT EST GMT+10:00 GMT-10:00 GMT11:00 GMT+10 GMT-10 GMT11 GMT1".to_string(), &mut session, &config);
        
        regex_tokinizer(&mut tokinizer_mut);
        let tokens = &tokinizer_mut.token_infos;

        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0].start, 0);
        assert_eq!(tokens[0].end, 3);
        assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Timezone("GMT".to_string(), 0)));

        assert_eq!(tokens[1].start, 4);
        assert_eq!(tokens[1].end, 7);
        assert_eq!(tokens[1].token_type.borrow().deref(), &Some(TokenType::Timezone("EST".to_string(), -300)));

        assert_eq!(tokens[2].start, 8);
        assert_eq!(tokens[2].end, 17);
        assert_eq!(tokens[2].token_type.borrow().deref(), &Some(TokenType::Timezone("GMT+10:00".to_string(), 600)));

        assert_eq!(tokens[3].start, 18);
        assert_eq!(tokens[3].end, 27);
        assert_eq!(tokens[3].token_type.borrow().deref(), &Some(TokenType::Timezone("GMT-10:00".to_string(), -600)));

        assert_eq!(tokens[4].start, 28);
        assert_eq!(tokens[4].end, 36);
        assert_eq!(tokens[4].token_type.borrow().deref(), &Some(TokenType::Timezone("GMT11:00".to_string(), 660)));

        assert_eq!(tokens[5].start, 37);
        assert_eq!(tokens[5].end, 43);
        assert_eq!(tokens[5].token_type.borrow().deref(), &Some(TokenType::Timezone("GMT+10".to_string(), 600)));

        assert_eq!(tokens[6].start, 44);
        assert_eq!(tokens[6].end, 50);
        assert_eq!(tokens[6].token_type.borrow().deref(), &Some(TokenType::Timezone("GMT-10".to_string(), -600)));

        assert_eq!(tokens[7].start, 51);
        assert_eq!(tokens[7].end, 56);
        assert_eq!(tokens[7].token_type.borrow().deref(), &Some(TokenType::Timezone("GMT11".to_string(), 660)));

        assert_eq!(tokens[8].start, 57);
        assert_eq!(tokens[8].end, 61);
        assert_eq!(tokens[8].token_type.borrow().deref(), &Some(TokenType::Timezone("GMT1".to_string(), 60)));
    }
}
