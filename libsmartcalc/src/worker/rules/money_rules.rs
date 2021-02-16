use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;

use crate::{types::{TokenType, CurrencyToken}};
use crate::tokinizer::{TokenLocation};
use crate::constants::{CURRENCY_RATES};

use crate::worker::tools::{get_money, get_currency, get_percent};


pub fn convert_money(fields: &BTreeMap<String, &TokenLocation>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("money") && fields.contains_key("currency") {
        let (price, currency) = match get_money("money".to_string(), fields) {
            Some((price, currency)) => (price, currency),
            _ => return Err("Money information not valid".to_string())
        };

        let to_currency = match get_currency("currency".to_string(), fields) {
            Some(to_currency) => to_currency,
            _ => return Err("Currency information not valid".to_string())
        };

        let as_usd = match CURRENCY_RATES.read().unwrap().get(&currency) {
            Some(l_rate) => price / l_rate,
            _ => return Err("Currency information not valid".to_string())
        };

        let calculated_price = match CURRENCY_RATES.read().unwrap().get(&to_currency) {
            Some(r_rate) => as_usd * r_rate,
            _ => return Err("Currency information not valid".to_string())
        };

        return Ok(TokenType::Money(calculated_price, CurrencyToken {
            currency:  to_currency.to_string(),
            start: 0,
            end: 0
        }));
    }

    Err("Money type not valid".to_string())
}

pub fn money_on(fields: &BTreeMap<String, &TokenLocation>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("money") && fields.contains_key("p") {
        let (price, currency) = match get_money("money".to_string(), fields) {
            Some((price, currency)) => (price, currency),
            _ => return Err("Money information not valid".to_string())
        };

        let percent = match get_percent("p".to_string(), fields) {
            Some(percent) => percent,
            _ => return Err("Percent information not valid".to_string())
        };

        let calculated_price = price + ((price * percent) / 100.0);

        return Ok(TokenType::Money(calculated_price, CurrencyToken {
            currency:  currency.to_string(),
            start: 0,
            end: 0
        }));
    }

    Err("Money type not valid".to_string())
}


pub fn money_of(fields: &BTreeMap<String, &TokenLocation>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("money") && fields.contains_key("p") {
        let (price, currency) = match get_money("money".to_string(), fields) {
            Some((price, currency)) => (price, currency),
            _ => return Err("Money information not valid".to_string())
        };

        let percent = match get_percent("p".to_string(), fields) {
            Some(percent) => percent,
            _ => return Err("Percent information not valid".to_string())
        };

        let calculated_price = (price * percent) / 100.0;

        return Ok(TokenType::Money(calculated_price, CurrencyToken {
            currency:  currency.to_string(),
            start: 0,
            end: 0
        }));
    }

    Err("Money type not valid".to_string())
}


pub fn money_off(fields: &BTreeMap<String, &TokenLocation>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("money") && fields.contains_key("p") {
        let (price, currency) = match get_money("money".to_string(), fields) {
            Some((price, currency)) => (price, currency),
            _ => return Err("Money information not valid".to_string())
        };

        let percent = match get_percent("p".to_string(), fields) {
            Some(percent) => percent,
            _ => return Err("Percent information not valid".to_string())
        };

        let calculated_price = price - ((price * percent) / 100.0);

        return Ok(TokenType::Money(calculated_price, CurrencyToken {
            currency:  currency.to_string(),
            start: 0,
            end: 0
        }));
    }

    Err("Money type not valid".to_string())
}

#[cfg(test)]
#[test]
fn convert_money_1() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
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
    assert_eq!(tokens[0].token, TokenType::Money(70.727697572, CurrencyToken { currency: "try".to_string(), start: 10, end: 13 }));

}


#[cfg(test)]
#[test]
fn convert_money_2() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
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
    assert_eq!(tokens[0].token, TokenType::Money(70.727697572, CurrencyToken { currency: "try".to_string(), start: 10, end: 13 }));

}

#[cfg(test)]
#[test]
fn convert_money_3() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
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
    assert_eq!(tokens[0].token, TokenType::Money(70.727697572, CurrencyToken { currency: "try".to_string(), start: 10, end: 13 }));

}


#[cfg(test)]
#[test]
fn convert_money_4() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
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
    assert_eq!(tokens[2].token, TokenType::Money(134.4772867837901, CurrencyToken { currency: "eur".to_string(), start: 10, end: 13 }));

}



#[cfg(test)]
#[test]
fn convert_money_5() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    let tokinizer_mut = setup("$9 in Euro".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_locations;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);

    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 10);
    assert_eq!(tokens[0].token, TokenType::Money(7.5106400733, CurrencyToken { currency: "eur".to_string(), start: 10, end: 13 }));

}


#[cfg(test)]
#[test]
fn convert_money_6() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    use log;
    
    let tokinizer_mut = setup("2M eur".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_locations;

    let tokens = token_generator(&tokens);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 2);
    assert_eq!(tokens[0].token, TokenType::Money(2_000_000.0, CurrencyToken { currency: "eur".to_string(), start: 3, end: 6 }));

    assert_eq!(tokens[1].start, 0);
    assert_eq!(tokens[1].end, 4);
    assert_eq!(tokens[1].token, TokenType::Money(2_000_000.0, CurrencyToken { currency: "eur".to_string(), start: 4, end: 7 }));
}


#[cfg(test)]
#[test]
fn money_on_1() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    let tokinizer_mut = setup("6% on 40 EUR".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_locations;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);
    
    assert_eq!(tokens[0].token, TokenType::Money(42.4, CurrencyToken { currency: "eur".to_string(), start: 10, end: 13 }));
}


#[cfg(test)]
#[test]
fn money_of_1() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    let tokinizer_mut = setup("6% of 40 EUR".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_locations;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);
    
    assert_eq!(tokens[0].token, TokenType::Money(2.4, CurrencyToken { currency: "eur".to_string(), start: 10, end: 13 }));
}


#[cfg(test)]
#[test]
fn money_off_1() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    let tokinizer_mut = setup("6% off 40 EUR".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_locations;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);
    
    assert_eq!(tokens[0].token, TokenType::Money(37.6, CurrencyToken { currency: "eur".to_string(), start: 10, end: 13 }));
}
