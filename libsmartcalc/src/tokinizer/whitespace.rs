use regex::Regex;
use crate::{executer::initialize, types::*};
use crate::tokinizer::Tokinizer;

pub fn is_whitespace(ch: char) -> bool {
    ch == ' '
}

pub fn whitespace_parser(tokinizer: &mut Tokinizer) -> TokenParserResult {
    if !is_whitespace(tokinizer.get_char()) {
        return Ok(false);
    }

    let mut ch = tokinizer.get_char();
    while !tokinizer.is_end() && is_whitespace(ch) {
        tokinizer.increase_index();
        ch = tokinizer.get_char();
    }
    Ok(true)
}

pub fn whitespace_regex_parser(tokinizer: &mut Tokinizer, group_item: &Vec<Regex>) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), None);
        }
    }
}

#[cfg(test)]
#[test]
fn whitespace_test_1() {
    use crate::tokinizer::test::setup;
    let mut tokinizer = setup("                                          ".to_string());

    tokinizer.borrow_mut().tokinize_with_regex();
    assert_eq!(tokinizer.borrow().token_locations.len(), 0);
}
