use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::borrow::ToOwned;
use regex::Regex;
use crate::tokinizer::Tokinizer;

pub fn whitespace_regex_parser(tokinizer: &mut Tokinizer, group_item: &Vec<Regex>) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), None, capture.get(0).unwrap().as_str().to_string());
        }
    }
}

#[cfg(test)]
#[test]
fn whitespace_test_1() {
    use crate::tokinizer::test::setup;
    let tokinizer = setup("                                          ".to_string());

    tokinizer.borrow_mut().tokinize_with_regex();
    assert_eq!(tokinizer.borrow().token_infos.len(), 0);
}
