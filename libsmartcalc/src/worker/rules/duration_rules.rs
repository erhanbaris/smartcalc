use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;

use chrono::{Local, NaiveDate, Datelike};

use crate::{types::{TokenType}, worker::tools::{get_number, get_text}};
use crate::tokinizer::{TokenInfo};

pub fn duration_parse(fields: &BTreeMap<String, &TokenInfo>) -> core::result::Result<TokenType, String> {
    if (fields.contains_key("duration")) && fields.contains_key("type") {
        let duration = match get_number("duration", fields) {
            Some(number) => number,
            _ => return Err("Duration information not valid".to_string())
        };

        let duration_type = match get_text("type", fields) {
            Some(number) => number,
            _ => return Err("Duration type information not valid".to_string())
        };

        return Ok(TokenType::Number(1.2));
    }
    Err("Date type not valid".to_string())
}


#[cfg(test)]
#[test]
fn small_date_test_1() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    let tokinizer_mut = setup("12 january".to_string());

    tokinizer_mut.borrow_mut().language_based_tokinize();
    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_infos;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);

    assert_eq!(tokens.len(), 1);
    
    assert_eq!(tokens[0], TokenType::Date(NaiveDate::from_ymd(Local::now().date().year(), 1, 12)));
}

#[cfg(test)]
#[test]
fn small_date_test_2() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    let tokinizer_mut = setup("32 january".to_string());

    tokinizer_mut.borrow_mut().language_based_tokinize();
    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_infos;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);

    assert_eq!(tokens.len(), 2);
    
    assert_eq!(tokens[0], TokenType::Number(32.0));
    assert_eq!(tokens[1], TokenType::Month(1));
}

#[cfg(test)]
#[test]
fn small_date_test_3() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    let tokinizer_mut = setup("22 december 1985".to_string());

    tokinizer_mut.borrow_mut().language_based_tokinize();
    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_infos;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);

    assert_eq!(tokens.len(), 1);
    
    assert_eq!(tokens[0], TokenType::Date(NaiveDate::from_ymd(1985, 12, 22)));
}

#[cfg(test)]
#[test]
fn small_date_test_4() {
    use crate::tokinizer::test::setup;
    use crate::executer::token_generator;
    use crate::executer::token_cleaner;
    let tokinizer_mut = setup("22/12/1985".to_string());

    tokinizer_mut.borrow_mut().language_based_tokinize();
    tokinizer_mut.borrow_mut().tokinize_with_regex();
    tokinizer_mut.borrow_mut().apply_aliases();
    tokinizer_mut.borrow_mut().apply_rules();

    let tokens = &tokinizer_mut.borrow().token_infos;

    let mut tokens = token_generator(&tokens);
    token_cleaner(&mut tokens);

    assert_eq!(tokens.len(), 1);
    
    assert_eq!(tokens[0], TokenType::Date(NaiveDate::from_ymd(1985, 12, 22)));
}
