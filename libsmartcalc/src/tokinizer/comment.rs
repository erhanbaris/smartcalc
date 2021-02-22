use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::borrow::ToOwned;
use regex::Regex;
use crate::tokinizer::Tokinizer;
use crate::token::ui_token::{UiTokenType};

pub fn comment_regex_parser(tokinizer: &mut Tokinizer, group_item: &Vec<Regex>) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            if tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), None, capture.get(0).unwrap().as_str().to_string()) {
                tokinizer.add_ui_token(capture.get(0), UiTokenType::Comment);
            }
        }
    }
}

#[cfg(test)]
#[test]
fn comment_test_1() {
    use crate::tokinizer::test::setup;
    let tokinizer = setup("#123".to_string());

    tokinizer.borrow_mut().tokinize_with_regex();
    assert_eq!(tokinizer.borrow().ui_tokens.len(), 1);
}

#[cfg(test)]
#[test]
fn comment_test_2() {
    use crate::tokinizer::test::setup;
    let tokinizer = setup("#
#123
# 111".to_string());

    tokinizer.borrow_mut().tokinize_with_regex();
    assert_eq!(tokinizer.borrow().ui_tokens.len(), 3);
}
