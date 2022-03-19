/*
 * smartcalc v1.0.7
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::borrow::ToOwned;
use regex::Regex;
use crate::config::SmartCalcConfig;
use crate::{types::*};
use crate::tokinizer::Tokinizer;
use crate::token::ui_token::UiTokenType;

pub fn operator_regex_parser(_: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            if tokinizer.add_token_from_match(&capture.get(0), Some(TokenType::Operator(capture.get(0).unwrap().as_str().chars().next().unwrap())))  {
                tokinizer.add_uitoken_from_match(capture.get(0), UiTokenType::Operator);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::*;
    use alloc::string::ToString;
    
    #[cfg(test)]
    #[test]
    fn operator_test_1() {
        use core::ops::Deref;
        use crate::tokinizer::regex_tokinizer;
        use crate::tokinizer::test::setup_tokinizer;
        use crate::config::SmartCalcConfig;
        use crate::session::Session;
        let mut session = Session::new();
        let config = SmartCalcConfig::default();
        let mut tokinizer = setup_tokinizer(" - merhaba".to_string(), &mut session, &config);
        
        regex_tokinizer(&mut tokinizer);

        assert_eq!(tokinizer.token_infos.len(), 2);

        assert_eq!(tokinizer.token_infos[0].start, 1);
        assert_eq!(tokinizer.token_infos[0].end, 2);
        assert_eq!(tokinizer.token_infos[0].token_type.borrow().deref(), &Some(TokenType::Operator('-')));
        
        assert_eq!(tokinizer.token_infos[1].start, 3);
        assert_eq!(tokinizer.token_infos[1].end, 10);
        assert_eq!(tokinizer.token_infos[1].token_type.borrow().deref(), &Some(TokenType::Text("merhaba".to_string())));
    }

    #[cfg(test)]
    #[test]
    fn operator_test_2() {
        use core::ops::Deref;
        use crate::tokinizer::regex_tokinizer;
        use crate::tokinizer::test::setup_tokinizer;
        use alloc::string::ToString;
        use crate::config::SmartCalcConfig;
        use crate::session::Session;
        let mut session = Session::new();
        let config = SmartCalcConfig::default();
        let mut tokinizer = setup_tokinizer("- ' * ` /,".to_string(), &mut session, &config);

        regex_tokinizer(&mut tokinizer);

        assert_eq!(tokinizer.token_infos.len(), 6);
        assert_eq!(tokinizer.token_infos[0].start, 0);
        assert_eq!(tokinizer.token_infos[0].end, 1);
        assert_eq!(tokinizer.token_infos[0].token_type.borrow().deref(), &Some(TokenType::Operator('-')));
        
        assert_eq!(tokinizer.token_infos[1].start, 2);
        assert_eq!(tokinizer.token_infos[1].end, 3);
        assert_eq!(tokinizer.token_infos[1].token_type.borrow().deref(), &Some(TokenType::Operator('\'')));

        assert_eq!(tokinizer.token_infos[2].start, 4);
        assert_eq!(tokinizer.token_infos[2].end, 5);
        assert_eq!(tokinizer.token_infos[2].token_type.borrow().deref(), &Some(TokenType::Operator('*')));

        assert_eq!(tokinizer.token_infos[3].start, 6);
        assert_eq!(tokinizer.token_infos[3].end, 7);
        assert_eq!(tokinizer.token_infos[3].token_type.borrow().deref(), &Some(TokenType::Operator('`')));

        assert_eq!(tokinizer.token_infos[4].start, 8);
        assert_eq!(tokinizer.token_infos[4].end, 9);
        assert_eq!(tokinizer.token_infos[4].token_type.borrow().deref(), &Some(TokenType::Operator('/')));

        assert_eq!(tokinizer.token_infos[5].start, 9);
        assert_eq!(tokinizer.token_infos[5].end, 10);
        assert_eq!(tokinizer.token_infos[5].token_type.borrow().deref(), &Some(TokenType::Operator(',')));
    }
}