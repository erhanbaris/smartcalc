use alloc::string::ToString;
use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use chrono::{Duration, Local};
use crate::types::{TokenType};
use crate::tokinizer::Tokinizer;
use crate::token::ui_token::{UiTokenType};
use regex::{Regex};
use crate::worker::tools::{read_currency};
use crate::constants::{CONSTANT_PAIRS, ConstantType};

pub fn text_regex_parser(tokinizer: &mut Tokinizer, group_item: &Vec<Regex>) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            let text = capture.name("TEXT").unwrap().as_str();
            if text.trim().len() != 0 {

                if let Some(constant) = CONSTANT_PAIRS.read().unwrap().get(&tokinizer.language).unwrap().get(&text.to_string()) {

                    let token = match constant {
                        ConstantType::Today     => Some(TokenType::Date(Local::today().naive_local())),
                        ConstantType::Tomorrow  => Some(TokenType::Date(Local::today().naive_local() + Duration::days(-1))),
                        ConstantType::Yesterday => Some(TokenType::Date(Local::today().naive_local() + Duration::days(1))),
                        ConstantType::Now       => Some(TokenType::Time(Local::now().time())),
                        _ => None
                    };

                    if token.is_some() && tokinizer.add_token(&capture.get(0), token) {
                        match read_currency(text) {
                            Some(_) => tokinizer.ui_tokens.add_from_regex_match(capture.get(0), UiTokenType::MoneySymbol),
                            _ => tokinizer.ui_tokens.add_from_regex_match(capture.get(0), UiTokenType::Text)
                        };
                        return;
                    }
                }

                if tokinizer.add_token(&capture.get(0), Some(TokenType::Text(text.to_string()))) {
                    match read_currency(text) {
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
    use crate::tokinizer::test::setup;
    let tokinizer_mut = setup("erhan barış aysel barış test".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    let tokens = &tokinizer_mut.borrow().token_infos;

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 5);
    assert_eq!(tokens[0].token_type, Some(TokenType::Text("erhan".to_string())));

    assert_eq!(tokens[1].start, 6);
    assert_eq!(tokens[1].end, 13);
    assert_eq!(tokens[1].token_type, Some(TokenType::Text("barış".to_string())));

    assert_eq!(tokens[2].start, 14);
    assert_eq!(tokens[2].end, 19);
    assert_eq!(tokens[2].token_type, Some(TokenType::Text("aysel".to_string())));

    assert_eq!(tokens[3].start, 20);
    assert_eq!(tokens[3].end, 27);
    assert_eq!(tokens[3].token_type, Some(TokenType::Text("barış".to_string())));

    assert_eq!(tokens[4].start, 28);
    assert_eq!(tokens[4].end, 32);
    assert_eq!(tokens[4].token_type, Some(TokenType::Text("test".to_string())));
}

