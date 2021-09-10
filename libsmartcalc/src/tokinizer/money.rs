use alloc::string::ToString;
use regex::Regex;
use alloc::borrow::ToOwned;
use crate::config::SmartCalcConfig;
use crate::tokinizer::Tokinizer;
use crate::types::{TokenType};
use crate::token::ui_token::{UiTokenType};
use crate::worker::tools::{read_currency};
use core::ops::Deref;

pub fn money_regex_parser(config: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            /* Check price value */
            let price = match capture.name("PRICE").unwrap().as_str().replace(".", "").replace(",", ".").parse::<f64>() {
                Ok(price) => match capture.name("NOTATION") {
                    Some(notation) => price * match notation.as_str() {
                        "k" | "K" => 1_000.0,
                        "M" => 1_000_000.0,
                        "G" => 1_000_000_000.0,
                        "T" => 1_000_000_000_000.0,
                        "P" => 1_000_000_000_000_000.0,
                        "Z" => 1_000_000_000_000_000_000.0,
                        "Y" => 1_000_000_000_000_000_000_000.0,
                        _ => 1.0
                    },
                    _ => price
                },
                _ => continue
            };

            /* Check currency value */
            let currency = match capture.name("CURRENCY") {
                Some(data) => data.as_str(),
                _ => continue
            };

            let currency = match read_currency(config, currency) {
                Some(real_currency) => real_currency,
                _ => continue
            };
            
            let end = match capture.name("NOTATION") {
                Some(notation) => notation.end(),
                _ => capture.name("CURRENCY").unwrap().end()
            };

            if tokinizer.add_token_location(capture.get(0).unwrap().start(), end, Some(TokenType::Money(price, currency.clone())), capture.name("PRICE").unwrap().as_str().to_string()) {
                tokinizer.ui_tokens.add_from_regex_match(capture.name("PRICE"), UiTokenType::Money);
                tokinizer.ui_tokens.add_from_regex_match(capture.name("CURRENCY"), UiTokenType::MoneySymbol);
                tokinizer.ui_tokens.add_from_regex_match(capture.name("NOTATION"), UiTokenType::MoneySymbol);
            }
        }
    }
}

#[cfg(test)]
#[test]
fn money_test_1() {
    use crate::tokinizer::test::setup_tokinizer;
    use core::cell::RefCell;
    use crate::config::SmartCalcConfig;
    use crate::app::Session;
    let session = RefCell::new(Session::new());
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("1000TRY 1000try 1000 try 1000 tl 1000 ₺ ₺1000".to_string(), &session, &config);

    tokinizer_mut.tokinize_with_regex();
    let tokens = &tokinizer_mut.session.borrow().token_infos;

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 7);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Money(1000.0, config.get_currency("try".to_string()).unwrap())));
    
    assert_eq!(tokens[1].start, 8);
    assert_eq!(tokens[1].end, 15);
    assert_eq!(tokens[1].token_type.borrow().deref(), &Some(TokenType::Money(1000.0, config.get_currency("try".to_string()).unwrap())));
    
    assert_eq!(tokens[2].start, 16);
    assert_eq!(tokens[2].end, 24);
    assert_eq!(tokens[2].token_type.borrow().deref(), &Some(TokenType::Money(1000.0, config.get_currency("try".to_string()).unwrap())));
    
    assert_eq!(tokens[3].start, 25);
    assert_eq!(tokens[3].end, 32);
    assert_eq!(tokens[3].token_type.borrow().deref(), &Some(TokenType::Money(1000.0, config.get_currency("try".to_string()).unwrap())));
    
    assert_eq!(tokens[4].start, 33);
    assert_eq!(tokens[4].end, 41);
    assert_eq!(tokens[4].token_type.borrow().deref(), &Some(TokenType::Money(1000.0, config.get_currency("try".to_string()).unwrap())));
    
    assert_eq!(tokens[5].start, 42);
    assert_eq!(tokens[5].end, 49);
    assert_eq!(tokens[5].token_type.borrow().deref(), &Some(TokenType::Money(1000.0, config.get_currency("try".to_string()).unwrap())));
}

#[cfg(test)]
#[test]
fn money_test_2() {
    use crate::tokinizer::test::setup_tokinizer;
    use core::ops::Deref;
    use core::cell::RefCell;
    use crate::config::SmartCalcConfig;
    use crate::app::Session;
    let session = RefCell::new(Session::new());
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("$2k".to_string(), &session, &config);

    tokinizer_mut.tokinize_with_regex();
    let tokens = &tokinizer_mut.session.borrow().token_infos;

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 3);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Money(2000.0, config.get_currency("usd".to_string()).unwrap())));
}
