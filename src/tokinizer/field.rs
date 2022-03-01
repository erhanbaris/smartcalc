/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::rc::Rc;
use alloc::string::ToString;
use alloc::borrow::ToOwned;
use crate::config::SmartCalcConfig;
use crate::types::*;
use crate::tokinizer::Tokinizer;
use regex::{Regex, Captures};

fn get_field_type<'t>(config: &SmartCalcConfig, type_name: &str, value: &str, language: &str, capture: &Captures<'t>) -> Option<FieldType> {
    match type_name {
        "DATE" => Some(FieldType::Date(value.to_string())),
        "TIME" => Some(FieldType::Time(value.to_string())),
        "NUMBER" => Some(FieldType::Number(value.to_string())),
        "TEXT" => Some(FieldType::Text(value.to_string())),
        "MONEY" => Some(FieldType::Money(value.to_string())),
        "PERCENT" => Some(FieldType::Percent(value.to_string())),
        "MONTH" => Some(FieldType::Month(value.to_string())),
        "MEMORY" => Some(FieldType::Memory(value.to_string())),
        "TIMEZONE" => Some(FieldType::Timezone(value.to_string())),
        "DURATION" => Some(FieldType::Duration(value.to_string())),
        "GROUP" => {
            let group  = match capture.name("GROUP") {
                Some(data) => data.as_str().to_string(),
                None => "".to_string()
            };
            
            config.word_group.get(language).unwrap().get(&group).map(|group_items| FieldType::Group(value.to_string(), group_items.to_vec()))
        },
        _ => match config.json_data.type_group.get(type_name) {
            Some(group) => Some(FieldType::TypeGroup(group.to_vec(), value.to_string())),
            _ => {
                log::info!("Field type not found, {}", type_name);
                None
            }
        }
    }
}

pub fn field_regex_parser(config: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            let field_type = capture.name("FIELD").unwrap().as_str();
            let name  = capture.name("NAME").unwrap().as_str();
            let language = tokinizer.session.borrow().get_language();

            if let Some(field) = get_field_type(config, field_type, name, &language, &capture) {
                tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), Some(TokenType::Field(Rc::new(field))), capture.get(0).unwrap().as_str().to_string());
            }
        }
    }
}

#[cfg(test)]
#[test]
fn field_test() {
    use core::ops::Deref;
    use crate::tokinizer::test::setup_tokinizer;
    use core::cell::RefCell;
    use crate::config::SmartCalcConfig;
    use crate::app::Session;
    let session = RefCell::new(Session::new());
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("{TEXT:merhaba} {PERCENT:percent}".to_string(), &session, &config);

    tokinizer_mut.tokinize_with_regex();
    let tokens = &tokinizer_mut.session.borrow().token_infos;

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 14);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Field(Rc::new(FieldType::Text("merhaba".to_string())))));

    assert_ne!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Field(Rc::new(FieldType::Text("test".to_string())))));

    assert_eq!(tokens[1].start, 15);
    assert_eq!(tokens[1].end, 32);
    assert_eq!(tokens[1].token_type.borrow().deref(), &Some(TokenType::Field(Rc::new(FieldType::Percent("percent".to_string())))));
}
