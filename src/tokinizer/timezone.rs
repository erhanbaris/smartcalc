/*
 * smartcalc v1.0.1
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
use crate::tools::parse_timezone;

pub fn timezone_regex_parser(config: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned().to_uppercase()) {
            match parse_timezone(config, &capture) {
                Some((timezone, offset)) => {
                    if tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), Some(TokenType::Timezone(timezone, offset)), capture.get(0).unwrap().as_str().to_string()) {
                        tokinizer.ui_tokens.add_from_regex_match(capture.name("timezone"), UiTokenType::Text);
                    }
                },
                None => ()
            };
        }
    }
}
