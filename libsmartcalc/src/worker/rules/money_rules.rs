use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::collections::btree_map::BTreeMap;

use serde_json::{from_str, Result, Value};

use chrono::{Utc, Duration};

use crate::{executer::{token_cleaner, token_generator}, types::{TokenType, BramaAstType}};
use crate::tokinizer::{TokenLocation, TokenLocationStatus};
use crate::constants::{CURRENCY_RATES};

pub fn convert_money(fields: &BTreeMap<String, &TokenLocation>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("money") && fields.contains_key("curency") {
        let (price, currency) = match &fields.get("money").unwrap().token_type {
            Some(token) => match &token {
                TokenType::Money(price, currency) => (price, currency),
                TokenType::Variable(variable) => {
                    match &*variable.data {
                        BramaAstType::Money(price, currency) => (price, currency),
                        _ => return Err("Money information not valid".to_string())
                    }
                },
                _ => return Err("Money information not valid".to_string())
            },
            _ => return Err("Money information not valid".to_string())
        };


        let to_currency = match &fields.get("curency").unwrap().token_type {
            Some(token) => match &token {
                TokenType::Text(currency) => currency,
                /*TokenType::Variable(variable) => {
                    match &*variable.data {
                        BramaAstType::Text(currency) => currency,
                        _ => return Err("Currency information not valid".to_string())
                    }
                },*/
                _ => return Err("Currency information not valid".to_string())
            },
            _ => return Err("Currency information not valid".to_string())
        };
        

        let as_usd = match CURRENCY_RATES.read().unwrap().get(currency) {
            Some(l_rate) => price / l_rate,
            _ => 0.0
        };

        let calculated_price = match CURRENCY_RATES.read().unwrap().get(to_currency) {
            Some(r_rate) => as_usd * r_rate,
            _ => 0.0
        };

        return Ok(TokenType::Money(calculated_price, to_currency.to_string()));
    }

    Err("Time format not valid".to_string())
}

#[cfg(test)]
#[test]
fn convert_money_1() {
    use crate::tokinizer::test::setup;
    let tokinizer_mut = setup("10 usd as try".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_locations;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 13);
    assert_eq!(tokens[0].token, TokenType::Money(70.727697572, "try".to_string()));

}


#[cfg(test)]
#[test]
fn convert_money_2() {
    use crate::tokinizer::test::setup;
    let tokinizer_mut = setup("10 usd try".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_locations;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 10);
    assert_eq!(tokens[0].token, TokenType::Money(70.727697572, "try".to_string()));

}

#[cfg(test)]
#[test]
fn convert_money_3() {
    use crate::tokinizer::test::setup;
    let tokinizer_mut = setup("10 usd into try".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_locations;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 15);
    assert_eq!(tokens[0].token, TokenType::Money(70.727697572, "try".to_string()));

}


#[cfg(test)]
#[test]
fn convert_money_4() {
    use crate::tokinizer::test::setup;
    let tokinizer_mut = setup("salary = 1000 dkk eur".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_locations;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 6);
    assert_eq!(tokens[0].token, TokenType::Text("salary".to_string()));

    assert_eq!(tokens[2].start, 9);
    assert_eq!(tokens[2].end, 21);
    assert_eq!(tokens[2].token, TokenType::Money(134.4772867837901, "eur".to_string()));

}
