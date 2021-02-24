use alloc::string::String;
use crate::tokinizer::Tokinizer;
use crate::token::ui_token::UiTokenType;
use crate::constants::MONTHS_REGEXES;
use crate::types::TokenType;

use log;

pub fn month_parser(tokinizer: &mut Tokinizer, data: &String) {

    match MONTHS_REGEXES.read().unwrap().get("en") {
        Some(months) => {
            for (re, month) in months {
                for capture in re.captures_iter(data) {
                    if tokinizer.add_token(&capture.get(0), Some(TokenType::Month(*month as u32))) {
                        tokinizer.ui_tokens.add_from_regex_match(capture.get(0), UiTokenType::Month);
                    }

                    log::warn!("{:?}", capture);
                }
            }
        },
        None => ()
    };
}
