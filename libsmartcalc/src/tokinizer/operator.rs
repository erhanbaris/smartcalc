use crate::types::*;
use crate::tokinizer::Tokinizer;

pub fn operator_parser(tokinizer: &mut Tokinizer) -> TokenParserResult {
    let ch       = tokinizer.get_char();
    let start= tokinizer.column;

    tokinizer.increase_index();
    tokinizer.add_token(start, TokenType::Operator(ch));
    return Ok(true);
}