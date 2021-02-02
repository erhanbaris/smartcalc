use regex::Regex;
use crate::constants::CURRENCIES;
use crate::tokinizer::Tokinizer;
use crate::types::TokenType;

pub fn money_regex_parser(tokinizer: &mut Tokinizer, group_item: &Vec<Regex>) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            /* Check price value */
            let price = match capture.name("PRICE").unwrap().as_str().replace(".", "").replace(",", ".").parse::<f64>() {
                Ok(price) => price,
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

            tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), Some(TokenType::Money(price, currency.to_string())));
        }
    }
}

#[cfg(test)]
#[test]
fn money_test() {
    use crate::tokinizer::test::setup;
    let tokinizer_mut = setup("1000TRY 1000try 1000 try 1000 tl 1000 ₺ ₺1000".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    let tokens = &tokinizer_mut.borrow().token_locations;

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 7);
    assert_eq!(tokens[0].token_type, Some(TokenType::Money(1000.0, "try".to_string())));
    
    assert_eq!(tokens[1].start, 8);
    assert_eq!(tokens[1].end, 15);
    assert_eq!(tokens[1].token_type, Some(TokenType::Money(1000.0, "try".to_string())));
    
    assert_eq!(tokens[2].start, 16);
    assert_eq!(tokens[2].end, 24);
    assert_eq!(tokens[2].token_type, Some(TokenType::Money(1000.0, "try".to_string())));
    
    assert_eq!(tokens[3].start, 25);
    assert_eq!(tokens[3].end, 32);
    assert_eq!(tokens[3].token_type, Some(TokenType::Money(1000.0, "try".to_string())));
    
    assert_eq!(tokens[4].start, 33);
    assert_eq!(tokens[4].end, 41);
    assert_eq!(tokens[4].token_type, Some(TokenType::Money(1000.0, "try".to_string())));
    
    assert_eq!(tokens[5].start, 42);
    assert_eq!(tokens[5].end, 49);
    assert_eq!(tokens[5].token_type, Some(TokenType::Money(1000.0, "try".to_string())));
}
