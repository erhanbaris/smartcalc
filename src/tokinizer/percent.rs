use crate::types::*;
use crate::tokinizer::number::{get_number_token};

#[allow(dead_code)]
pub fn is_percent_token(token: Token, _token_index: usize, _tokens: Vec<Token>) -> bool {
    match token {
        Token::Text(_text) => {
            /*match tokens.get(token_index + 1) {
                Some(token) =>
            }*/
            true
        },
        _ => false
    }
}

pub fn percent_parser(mut tokinizer: &mut Tokinizer) -> TokenParserResult {
    let indexer      = tokinizer.get_indexer();
    let start_column = tokinizer.column;
    let number;

    if tokinizer.get_char() == '%' {
        tokinizer.increase_index();

        let number_token = get_number_token(&mut tokinizer);
        number = match number_token {
            Some(Token::Number(number)) => number,
            Some(_) => {
                println!("{:?}", number_token);
                return Err(("Percent not parsed", tokinizer.column));
            },
            None => {
                println!("{:?}", number_token);
                return Err(("Percent not parsed", tokinizer.column));
            }
        };
    }
    else {
        number = match get_number_token(&mut tokinizer) {
            Some(token_type) => {
                match token_type {
                    Token::Number(num) => num,
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

    tokinizer.add_token(start_column, Token::Percent(number));
    return Ok(true);
}