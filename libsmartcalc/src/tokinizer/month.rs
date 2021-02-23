use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::borrow::ToOwned;
use regex::Regex;
use crate::tokinizer::Tokinizer;
use crate::token::ui_token::{UiTokenType};
use crate::constants::MONTHS_REGEXES;

use log;

pub fn month_parser(tokinizer: &mut Tokinizer) {

    match MONTHS_REGEXES.read().unwrap().get("en") {
        Some(months) => {
            for (re, month_number) in months {
                for capture in re.captures_iter(&tokinizer.data) {
                    log::warn!("{:?}", capture);
                }
            }
        },
        None => ()
    };
}
