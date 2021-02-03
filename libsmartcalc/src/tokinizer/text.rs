use std::rc::Rc;
use crate::types::*;
use crate::tokinizer::Tokinizer;
use regex::Regex;

pub fn is_text(ch: char) -> bool {
    ch.is_alphabetic()
}

pub fn text_parser(tokinizer: &mut Tokinizer) -> TokenParserResult {
    let mut ch            = tokinizer.get_char();
    let start             = tokinizer.index as usize;
    let mut end           = start;
    let start_column      = tokinizer.column;

    if !is_text(ch) {
        return Ok(false);
    }

    while !tokinizer.is_end() {
        ch = tokinizer.get_char();

        if !ch.is_alphabetic() {
            break;
        }

        if ch.is_whitespace() || ch == '\'' || ch == '"' {
            break;
        }
        end += ch.len_utf8();
        tokinizer.increase_index();
    }

    tokinizer.add_token(start_column as u16, TokenType::Text(tokinizer.data[start..end].to_string()));
    Ok(true)
}

pub fn text_regex_parser(tokinizer: &mut Tokinizer, group_item: &Vec<Regex>) {

    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            let text = capture.name("TEXT").unwrap().as_str();
            if text.trim().len() != 0 {
                tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), Some(TokenType::Text(text.to_string())), capture.get(0).unwrap().as_str().to_string());
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
    let tokens = &tokinizer_mut.borrow().token_locations;

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

