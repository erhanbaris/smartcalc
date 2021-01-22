use crate::types::*;

pub fn operator_parser(tokinizer: &mut Tokinizer) -> TokenParserResult {
    let ch       = tokinizer.get_char();
    let start= tokinizer.column;

    tokinizer.increase_index();
    tokinizer.add_token(start, BramaTokenType::Operator(ch));
    return Ok(true);
}