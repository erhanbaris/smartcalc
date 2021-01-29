use crate::types::*;
use crate::parser::number::{get_number_token};
use crate::tokinizer::Tokinizer;

pub fn money_parser(mut tokinizer: &mut Tokinizer) -> TokenParserResult {
    let indexer      = tokinizer.get_indexer();
    let start_column = tokinizer.column;
    let mut number   = 0.0;

    if tokinizer.get_char() == '%' {
        tokinizer.increase_index();

        number = match get_number_token(&mut tokinizer) {
            Some(token_type) => {
                match token_type {
                    TokenType::Number(num) => num,
                    _ => 0.0
                }
            },
            None => return Err(("Percent not parsed", tokinizer.column))
        };
    }
    else {
        number = match get_number_token(&mut tokinizer) {
            Some(token_type) => {
                match token_type {
                    TokenType::Number(num) => num,
                    _ => 0.0
                }
            },
            None => {
                tokinizer.set_indexer(indexer);
                return Ok(false);
            }
        };

        if tokinizer.get_char() != '%' {
            tokinizer.set_indexer(indexer);
            return Ok(false);
        }
        tokinizer.increase_index();
    }

    tokinizer.add_token(start_column, TokenType::Percent(number));
    return Ok(true);
}