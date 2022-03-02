/*
 * smartcalc v1.0.3
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;

use alloc::sync::Arc;
use chrono::Timelike;
use chrono::{Local, NaiveDate, Datelike};

use crate::config::SmartCalcConfig;
use crate::worker::tools::get_date;
use crate::worker::tools::get_number_or_time;
use crate::{tokinizer::Tokinizer, types::{TokenType}, worker::tools::{get_number, get_number_or_month}};
use crate::tokinizer::{TokenInfo};

pub fn small_date(config: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Arc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if (fields.contains_key("day")) && fields.contains_key("month") {
        let day = match get_number("day", fields) {
            Some(number) => number,
            _ => return Err("Number information not valid".to_string())
        };

        let month = match get_number_or_month("month", fields) {
            Some(number) => number,
            _ => return Err("Month information not valid".to_string())
        };

        let year = match get_number("year", fields) {
            Some(number) => number as i32,
            _ => Local::now().date().year() as i32
        };

        return match NaiveDate::from_ymd_opt(year, month, day as u32) {
            Some(date) => {
                Ok(TokenType::Date(date, config.get_time_offset()))
            },
            None => Err("Date is not valid".to_string())
        };
    }
    Err("Date type not valid".to_string())
}

pub fn at_date(config: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Arc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if (fields.contains_key("source")) && fields.contains_key("time") {
        let (date, date_tz) = match get_date("source", fields) {
            Some(number) => number,
            _ => return Err("Date information not valid".to_string())
        };
        
        //todo: convert timezone informations
        let (time, _) = match get_number_or_time(config, "time", fields) {
            Some(number) => number,
            _ => return Err("Date information not valid".to_string())
        };
        return Ok(TokenType::DateTime(date.and_hms(time.hour(), time.minute(), time.second()), date_tz));
    }
    Err("Date type not valid".to_string())
}


#[cfg(test)]
#[test]
fn small_date_test_1() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("12 january".to_string());
    let config = SmartCalcConfig::default();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Date(NaiveDate::from_ymd(Local::now().date().year(), 1, 12), config.get_time_offset())));
}

#[cfg(test)]
#[test]
fn small_date_test_2() {
    use crate::{tokinizer::test::get_executed_raw_tokens, types::NumberType};
    
    let tokens = get_executed_raw_tokens("32 january".to_string());
    assert_eq!(tokens.len(), 3);
    
    assert_eq!(*tokens[0], TokenType::Number(32.0, NumberType::Decimal));
    assert_eq!(*tokens[1], TokenType::Operator('+'));
    assert_eq!(*tokens[2], TokenType::Month(1));
}

#[cfg(test)]
#[test]
fn small_date_test_3() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let config = SmartCalcConfig::default();
    let tokens = execute("22 december 1985".to_string());

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Date(NaiveDate::from_ymd(1985, 12, 22), config.get_time_offset())));
}

#[cfg(test)]
#[test]
fn small_date_test_4() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let config = SmartCalcConfig::default();
    let tokens = execute("22/12/1985".to_string());
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Date(NaiveDate::from_ymd(1985, 12, 22), config.get_time_offset())));
}
