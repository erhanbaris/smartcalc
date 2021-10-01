use alloc::string::ToString;

use crate::config::SmartCalcConfig;
use crate::tokinizer::Tokinizer;
use crate::token::ui_token::UiTokenType;
use crate::types::TokenType;

pub fn long_text_parser(config: &SmartCalcConfig, tokinizer: &mut Tokinizer, data: &str) {
    if let Some(long_texts) = config.long_texts.get(&tokinizer.language) {
        for re in long_texts {
            for capture in re.captures_iter(data) {
                log::error!("{:?}", capture.get(0).unwrap().as_str().to_string());
                if tokinizer.add_token(&capture.get(0), Some(TokenType::Text(capture.get(0).unwrap().as_str().to_string()))) {
                    tokinizer.ui_tokens.add_from_regex_match(capture.get(0), UiTokenType::Text);
                }
            }
        }
    }
}
