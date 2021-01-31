use crate::types::*;
use crate::tokinizer::number::{get_number_token};
use crate::tokinizer::Tokinizer;
use regex::Regex;

pub fn percent_regex_parser(tokinizer: &mut Tokinizer, data: &mut String, group_item: &Vec<Regex>) -> String {
    let mut data_str = data.to_string();

    for re in group_item.iter() {
        for capture in re.captures_iter(data) {
            /* Check price value */
            let number = match capture.name("NUMBER").unwrap().as_str().replace(",", ".").parse::<f64>() {
                Ok(price) => price.to_string(),
                _ => return data_str
            };
            /*
                "(?P<NUMBER>[-+]?[0-9]+[0-9,]{0,}) (?P<TEXT>[\\\\p\\{L\\}-]+)",
                "(?P<TEXT>[\\\\p\\{L\\}-]+) (?P<NUMBER>[-+]?[0-9]+[0-9,]{0,})"
            */
            tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), TokenType::Percent(capture.name("NUMBER").unwrap().as_str().replace(",", ".").parse::<f64>().unwrap()));
            data_str = data_str.replace(capture.get(0).unwrap().as_str(), &format!("[PERCENT:{}]", number)[..]);
        }
    }

    data_str
}

pub fn percent_parser(mut tokinizer: &mut Tokinizer) -> TokenParserResult {
    let indexer      = tokinizer.get_indexer();
    let start_column = tokinizer.column;
    let number;

    if tokinizer.get_char() == '%' {
        tokinizer.increase_index();

        let number_token = get_number_token(&mut tokinizer);
        number = match number_token {
            Some(TokenType::Number(number)) => number,
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