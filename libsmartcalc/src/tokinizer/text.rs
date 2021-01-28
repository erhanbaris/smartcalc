use std::rc::Rc;
use crate::types::*;
use crate::tokinizer::Tokinizer;

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

    tokinizer.add_token(start_column as u16, Token::Text(Rc::new(tokinizer.data[start..end].to_string())));
    Ok(true)
}