use crate::types::*;

pub fn whitespace_parser(tokinizer: &mut Tokinizer) -> TokenParserResult {
    if tokinizer.get_char() != ' ' {
        return Ok(false);
    }

    let mut ch = tokinizer.get_char();
    while !tokinizer.is_end() && ch == ' ' {
        tokinizer.increase_index();
        ch = tokinizer.get_char();
    }
    Ok(true)
}