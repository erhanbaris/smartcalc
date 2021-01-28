use crate::types::*;
use crate::tokinizer::Tokinizer;

#[allow(dead_code)]
pub fn is_whitespace_token(token: Token, _token_index: usize, _tokens: Vec<Token>) -> bool {
    match &token {
        Token::Text(text) => **text == " ",
        _ => false
    }
}

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