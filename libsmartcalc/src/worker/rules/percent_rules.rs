use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;
use crate::config::SmartCalcConfig;
use crate::{tokinizer::{TokenInfo, Tokinizer}, types::{TokenType}, worker::tools::get_currency};

use crate::worker::tools::{get_number, get_percent, get_number_or_price};

pub fn percent_calculator(config: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, &TokenInfo>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("p") && fields.contains_key("number") {
        let number = match get_number(config, "number", fields) {
            Some(number) => number,
            _ => return Err("Number information not valid".to_string())
        };

        let percent = match get_percent("p", fields) {
            Some(number) => number,
            _ => return Err("Percent information not valid".to_string())
        };
        return Ok(TokenType::Number((percent * number) / 100.0));
    }

    Err("Percent not valid".to_string())
}

pub fn find_numbers_percent(config: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, &TokenInfo>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("part") && fields.contains_key("total") {
        let total = match get_number_or_price(config, "total", fields) {
            Some(number) => number,
            _ => return Err("Total number information not valid".to_string())
        };

        let part = match get_number_or_price(config, "part", fields) {
            Some(number) => number,
            _ => return Err("Part number information not valid".to_string())
        };
        
        return Ok(TokenType::Percent((part * 100.0) / total));
    }

    Err("Find percent not valid".to_string())
}

pub fn find_total_from_percent(config: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, &TokenInfo>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("number_part") && fields.contains_key("percent_part") {
        let number_part = match get_number_or_price(config, "number_part", fields) {
            Some(number) => number,
            _ => return Err("Number part information not valid".to_string())
        };

        let percent_part = match get_percent("percent_part", fields) {
            Some(percent) => percent,
            _ => return Err("Percent part information not valid".to_string())
        };

        return Ok(match get_currency(config, "number_part", fields) {
            Some(currency) => TokenType::Money((number_part * 100.0) / percent_part, currency.to_string()),
            None => TokenType::Number((number_part * 100.0) / percent_part)
        });
    }

    Err("Find percent not valid".to_string())
}


#[cfg(test)]
#[test]
fn find_percent_to_number_1() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    let tokinizer_mut = setup("20 is 10% of what".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_infos;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], TokenType::Number(200.0));

}

#[cfg(test)]
#[test]
fn find_percent_to_number_2() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    let tokinizer_mut = setup("180 is 10% of what".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_infos;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], TokenType::Number(1800.0));

}

#[cfg(test)]
#[test]
fn find_numbers_percent_1() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    let tokinizer_mut = setup("15 is what % of 100".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_infos;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], TokenType::Percent(15.00));
}
