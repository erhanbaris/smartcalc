use regex::Regex;
use crate::constants::CURRENCIES;
use crate::tokinizer::Tokinizer;
use crate::types::TokenType;

pub fn money_regex_parser(tokinizer: &mut Tokinizer, group_item: &Vec<Regex>) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            /* Check price value */
            let price = match capture.name("PRICE").unwrap().as_str().replace(".", "").replace(",", ".").parse::<f64>() {
                Ok(price) => price.to_string(),
                _ => return
            };

            /* Check currency value */
            let currency = match capture.name("CURRENCY") {
                Some(data) => data.as_str(),
                _ => return
            };

            let currency = match CURRENCIES.lock().unwrap().get(&currency.to_lowercase()) {
                Some(symbol) => symbol.to_lowercase(),
                _ => return
            };

            tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), Some(TokenType::Money(capture.name("PRICE").unwrap().as_str().replace(".", "").replace(",", ".").parse::<f64>().unwrap(), currency.to_string())));
        }
    }
}