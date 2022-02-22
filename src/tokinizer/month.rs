/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use crate::config::SmartCalcConfig;
use crate::tokinizer::Tokinizer;
use crate::token::ui_token::UiTokenType;
use crate::types::TokenType;

pub fn month_parser(config: &SmartCalcConfig, tokinizer: &mut Tokinizer, data: &str) {
    if let Some(months) = config.month_regex.get(&tokinizer.language) {
        for (re, month) in months {
            for capture in re.captures_iter(data) {
                if tokinizer.add_token(&capture.get(0), Some(TokenType::Month(month.month as u32))) {
                    tokinizer.ui_tokens.add_from_regex_match(capture.get(0), UiTokenType::Month);
                }
            }
        }
    }
}
