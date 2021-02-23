use alloc::vec::Vec;
use alloc::rc::Rc;
use alloc::string::ToString;
use alloc::borrow::ToOwned;
use crate::types::*;
use crate::tokinizer::Tokinizer;
use regex::Regex;
use crate::constants::WORD_GROUPS;

pub fn field_regex_parser(tokinizer: &mut Tokinizer, group_item: &Vec<Regex>) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            let field_type = capture.name("FIELD").unwrap().as_str();
            let name  = capture.name("NAME").unwrap().as_str();

            let field = match field_type {
                "DATE" => FieldType::Date(name.to_string()),
                "TIME" => FieldType::Time(name.to_string()),
                "NUMBER" => FieldType::Number(name.to_string()),
                "TEXT" => FieldType::Text(name.to_string()),
                "MONEY" => FieldType::Money(name.to_string()),
                "PERCENT" => FieldType::Percent(name.to_string()),
                "NUMBER_OR_MONEY" => FieldType::NumberOrMoney(name.to_string()),
                "GROUP" => {
                    match WORD_GROUPS.read().unwrap().get(name) {
                        Some(group_items) => FieldType::Group(group_items.to_vec()),
                        _ => continue
                    }
                },
                _ => {
                    log::info!("Type not found, {}", field_type);
                    continue
                }
            };
            tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), Some(TokenType::Field(Rc::new(field))), capture.get(0).unwrap().as_str().to_string());
        }
    }
}

#[cfg(test)]
#[test]
fn field_test() {
    use crate::tokinizer::test::setup;
    let tokinizer_mut = setup("{TEXT:merhaba} {PERCENT:percent}".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    let tokens = &tokinizer_mut.borrow().token_locations;

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 14);
    assert_eq!(tokens[0].token_type, Some(TokenType::Field(Rc::new(FieldType::Text("merhaba".to_string())))));

    assert_ne!(tokens[0].token_type, Some(TokenType::Field(Rc::new(FieldType::Text("test".to_string())))));

    assert_eq!(tokens[1].start, 15);
    assert_eq!(tokens[1].end, 32);
    assert_eq!(tokens[1].token_type, Some(TokenType::Field(Rc::new(FieldType::Percent("percent".to_string())))));
}
