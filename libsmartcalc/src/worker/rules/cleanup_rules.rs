use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;

use crate::{types::{TokenType}};
use crate::tokinizer::{TokenLocation};
use crate::{types::{BramaAstType}};

pub fn division_cleanup(fields: &BTreeMap<String, &TokenLocation>) -> core::result::Result<TokenType, String> {
    if (fields.contains_key("data")) && fields.contains_key("text") {
        return match &fields.get(&"data".to_string()).unwrap().token_type {
            Some(token) => match &token {
                TokenType::Number(number) => Ok(TokenType::Number(*number)),
                TokenType::Percent(percent) => Ok(TokenType::Percent(*percent)),
                TokenType::Money(price, currency) => Ok(TokenType::Money(*price, currency.to_string())),
                TokenType::Variable(variable) => {
                    match &*variable.data {
                        BramaAstType::Number(number) => Ok(TokenType::Number(*number)),
                        BramaAstType::Percent(percent) => Ok(TokenType::Percent(*percent)),
                        BramaAstType::Money(price, currency) => Ok(TokenType::Money(*price, currency.to_string())),
                        _ => Err("Data type not valid".to_string())
                    }
                },
                _ => Err("Data type not valid".to_string())
            },
            _ => Err("Data type not valid".to_string())
        }
    }
    Err("Data type not valid".to_string())
}


#[cfg(test)]
#[test]
fn number_of_1() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    let tokinizer_mut = setup("$25/hour * 14 hours of work".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_locations;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);

    assert_eq!(tokens.len(), 3);
    
    assert_eq!(tokens[0].token, TokenType::Money(25.0, "usd".to_string()));
    assert_eq!(tokens[1].token, TokenType::Operator('*'));
    assert_eq!(tokens[2].token, TokenType::Number(14.0));
}
