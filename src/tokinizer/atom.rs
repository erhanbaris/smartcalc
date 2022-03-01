/*
 * smartcalc v1.0.2
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::borrow::ToOwned;
use chrono::Local;
use chrono::NaiveDateTime;

use crate::config::SmartCalcConfig;
use crate::types::*;
use crate::tokinizer::Tokinizer;
use chrono::NaiveTime;
use regex::Regex;

pub fn get_atom(config: &SmartCalcConfig, data: &str, group_item: &[Regex]) -> Vec<(usize, usize, Option<TokenType>, String)> {
    let mut atoms = Vec::new();

    for re in group_item.iter() {
        for capture in re.captures_iter(&data) {
            let atom_type = capture.name("ATOM").unwrap().as_str();
            let data      = capture.name("DATA").unwrap().as_str();

            let token_type = match atom_type {
                "TIME" => {
                    let seconds = data.parse::<u32>().unwrap();
                    let date = Local::now().naive_local().date();
                    let time = NaiveTime::from_num_seconds_from_midnight(seconds, 0);
                    let date_time = NaiveDateTime::new(date, time);
                    
                    TokenType::Time(date_time, config.get_time_offset())
                },
                "MONEY" => {
                    let splited_data: Vec<&str> = data.split(';').collect();
                    match config.get_currency(splited_data[1].to_string()) {
                        Some(currency_info) => TokenType::Money(splited_data[0].parse::<f64>().unwrap(), currency_info.clone()),
                        None => {
                            log::info!("Currency information not found, {}", splited_data[1]);
                            continue
                        }
                    }
                },
                "NUMBER" => {
                    let number = data.parse::<f64>().unwrap();
                    TokenType::Number(number)
                },
                "PERCENT" => {
                    let number = data.parse::<f64>().unwrap();
                    TokenType::Percent(number)
                },
                "OPERATOR" => TokenType::Operator(data.chars().next().unwrap()),
                _ => {
                    log::info!("Atom type not found, {}", atom_type);
                    continue
                }
            };

            atoms.push((capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), Some(token_type), capture.get(0).unwrap().as_str().to_string()))
        }
    }
    atoms
}


pub fn atom_regex_parser(config: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]) {
    let atoms =  get_atom(config, &tokinizer.data.to_owned(), group_item);
    for (start, end, token_type, text) in atoms {
        tokinizer.add_token_location(start, end, token_type, text);
    }
}

#[cfg(test)]
#[test]
fn operator_test() {
    use core::ops::Deref;
    use crate::tokinizer::test::setup_tokinizer;
    use core::cell::RefCell;
    use crate::config::SmartCalcConfig;
    use crate::app::Session;
    let session = RefCell::new(Session::new());
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("[OPERATOR:+] [PERCENT:-29.1] [TIME:44100]  [NUMBER:-222.333] [MONEY:200;try]".to_string(), &session, &config);

    tokinizer_mut.tokinize_with_regex();
    let tokens = &tokinizer_mut.session.borrow().token_infos;

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 12);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Operator('+')));

    assert_eq!(tokens[1].start, 13);
    assert_eq!(tokens[1].end, 28);
    assert_eq!(tokens[1].token_type.borrow().deref(), &Some(TokenType::Percent(-29.1)));

    assert_eq!(tokens[2].start, 29);
    assert_eq!(tokens[2].end, 41);
    assert_eq!(tokens[2].token_type.borrow().deref(), &Some(TokenType::Time(chrono::Utc::today().and_hms(12, 15, 0).naive_utc(), config.get_time_offset())));

    assert_eq!(tokens[3].start, 43);
    assert_eq!(tokens[3].end, 60);
    assert_eq!(tokens[3].token_type.borrow().deref(), &Some(TokenType::Number(-222.333)));

    assert_eq!(tokens[4].start, 61);
    assert_eq!(tokens[4].end, 76);
    assert_eq!(tokens[4].token_type.borrow().deref(), &Some(TokenType::Money(200.0, config.get_currency("try".to_string()).unwrap())));
}
