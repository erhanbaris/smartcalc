use regex::Regex;
use crate::constants::CURRENCIES;
use crate::tokinizer::Tokinizer;
use crate::types::TokenType;

pub fn money_regex_parser(tokinizer: &mut Tokinizer, data: &mut String, group_item: &Vec<Regex>) -> String {
    let mut data_str = data.to_string();

    for re in group_item.iter() {
        for capture in re.captures_iter(data) {
            /* Check price value */
            let price = match capture.name("PRICE").unwrap().as_str().replace(".", "").replace(",", ".").parse::<f64>() {
                Ok(price) => price.to_string(),
                _ => return data_str
            };

            /* Check currency value */
            let currency = match capture.name("CURRENCY") {
                Some(data) => data.as_str(),
                _ => return data_str
            };

            let currency = match CURRENCIES.lock().unwrap().get(&currency.to_lowercase()) {
                Some(symbol) => symbol.to_lowercase(),
                _ => return data_str
            };

            if tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), TokenType::Money(capture.name("PRICE").unwrap().as_str().replace(".", "").replace(",", ".").parse::<f64>().unwrap(), currency.to_string())) {
                data_str = data_str.replace(capture.get(0).unwrap().as_str(), &format!("[MONEY:{};{}]", price, currency)[..]);
            }
        }
    }

    data_str
}