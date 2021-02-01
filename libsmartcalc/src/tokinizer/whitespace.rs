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
    let data = "                                          ".to_string();
    let mut tokinizer = Tokinizer {
        column: 0,
        line: 0,
        tokens: Vec::new(),
        iter: data.chars().collect(),
        data: data.to_string(),
        index: 0,
        indexer: 0,
        total: data.chars().count(),
        token_locations: Vec::new()
    };
    initialize();

    tokinizer.tokinize_with_regex();

    assert_eq!(tokinizer.token_locations.len(), 1);
    assert_eq!(tokinizer.token_locations[0].start, 0);
    assert_eq!(tokinizer.token_locations[0].end, 42);
    assert_eq!(tokinizer.token_locations[0].token_type, None);
}
