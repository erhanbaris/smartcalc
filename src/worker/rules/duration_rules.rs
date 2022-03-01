/*
 * smartcalc v1.0.2
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;
use alloc::sync::Arc;
use core::ops::Deref;

use chrono::{Duration, Timelike};

use crate::config::SmartCalcConfig;
use crate::{constants::ConstantType, tokinizer::Tokinizer, types::{TokenType}, worker::tools::{get_duration, get_number, get_text, get_time, get_date}};
use crate::tokinizer::TokenInfo;
use crate::formatter::{MINUTE, HOUR, DAY, WEEK, MONTH, YEAR};

pub fn duration_parse(config: &SmartCalcConfig, tokinizer: &Tokinizer, fields: &BTreeMap<String, Arc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if (fields.contains_key("duration")) && fields.contains_key("type") {
        let duration = match get_number("duration", fields) {
            Some(number) => number as i64,
            _ => return Err("Duration information not valid".to_string())
        };

        let duration_type = match get_text("type", fields) {
            Some(number) => number,
            _ => return Err("Duration type information not valid".to_string())
        };

        let constant_type = match config.constant_pair.get(&tokinizer.language).unwrap().get(&duration_type) {
            Some(constant) => constant.clone(),
            None => return Err("Duration type not valid".to_string())
        };

        let calculated_duration = match constant_type {
            ConstantType::Year => Duration::days(365 * duration),
            ConstantType::Month => {
                let years = duration / 12;
                let month = duration % 12;

                Duration::days((365 * years) + (30 * month))
            },
            ConstantType::Day => {
                let years = duration / 365;
                let month = (duration % 365) / 30;
                let day = (duration % 365) % 30;

                Duration::days((365 * years) + (30 * month) + day)
            },
            ConstantType::Week => Duration::weeks(duration),
            ConstantType::Hour => Duration::hours(duration),
            ConstantType::Minute => Duration::minutes(duration),
            ConstantType::Second => Duration::seconds(duration),            
            _ => return Err("Duration type not valid".to_string()) 
        };

        return Ok(TokenType::Duration(calculated_duration));
    }
    Err("Date type not valid".to_string())
}

pub fn combine_durations(_: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Arc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if (fields.contains_key("1")) && fields.contains_key("2") {
        let mut sum_duration = Duration::zero();

        for key in fields.keys() {
            let duration = match get_duration(key, fields) {
                Some(duration) => duration,
                _ => return Err("Duration information not valid".to_string())
            };

            sum_duration = sum_duration + duration;
        }

        return Ok(TokenType::Duration(sum_duration));
    }
    Err("Date type not valid".to_string())
}

pub fn as_duration(config: &SmartCalcConfig, tokinizer: &Tokinizer, fields: &BTreeMap<String, Arc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if (fields.contains_key("source")) && fields.contains_key("type") {
        let duration_type = match get_text("type", fields) {
            Some(number) => number,
            _ => return Err("Duration type information not valid".to_string())
        };

        let constant_type = match config.constant_pair.get(&tokinizer.language).unwrap().get(&duration_type) {
            Some(constant) => constant.clone(),
            None => return Err("Duration type not valid".to_string())
        };

        match fields.get("source") {
            Some(token_info) => match token_info.token_type.borrow().deref()  {
                Some(TokenType::Duration(duration)) => {
                    let seconds = duration.num_seconds().abs() as i64;
                    
                    return match constant_type {
                        ConstantType::Day => Ok(TokenType::Duration(Duration::days(seconds / DAY))),
                        ConstantType::Second => Ok(TokenType::Duration(Duration::seconds(seconds))),
                        ConstantType::Minute => Ok(TokenType::Duration(Duration::minutes(seconds / MINUTE as i64))),
                        ConstantType::Hour => Ok(TokenType::Duration(Duration::hours(seconds / HOUR as i64))),
                        ConstantType::Week => Ok(TokenType::Duration(Duration::weeks(seconds / WEEK as i64))),
                        _ => return Err("Duration type not valid".to_string()) 
                    };
                },
                Some(TokenType::Time(time, _)) => {
                    let seconds = time.num_seconds_from_midnight() as i64;
                    
                    return match constant_type {
                        ConstantType::Month => Ok(TokenType::Duration(Duration::days(seconds / MONTH))),
                        ConstantType::Year => Ok(TokenType::Duration(Duration::days(seconds / YEAR))),
                        ConstantType::Day => Ok(TokenType::Duration(Duration::days(seconds / DAY))),
                        ConstantType::Second => Ok(TokenType::Duration(Duration::seconds(seconds))),
                        ConstantType::Minute => Ok(TokenType::Duration(Duration::minutes(seconds / MINUTE as i64))),
                        ConstantType::Hour => Ok(TokenType::Duration(Duration::hours(seconds / HOUR as i64))),
                        ConstantType::Week => Ok(TokenType::Duration(Duration::weeks(seconds / WEEK as i64))),

                        _ => return Err("Duration type not valid".to_string()) 
                    };
                }
                _ => ()
            },
            None => return Err("Source information not valid".to_string())
        };
        
        
        let duration = match get_number("duration", fields) {
            Some(number) => number as i64,
            _ => return Err("Duration information not valid".to_string())
        };

        let calculated_duration = match constant_type {
            ConstantType::Day => Duration::days(duration),
            ConstantType::Month => Duration::days(duration * 30),
            ConstantType::Year => Duration::days(duration * 365),
            ConstantType::Second => Duration::seconds(duration),
            ConstantType::Minute => Duration::minutes(duration),
            ConstantType::Hour => Duration::hours(duration),
            _ => return Err("Duration type not valid".to_string()) 
        };

        return Ok(TokenType::Duration(calculated_duration));
    }
    Err("Date type not valid".to_string())
}

pub fn to_duration(_: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Arc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if (fields.contains_key("source")) && fields.contains_key("target") {
        //todo: calculate with timezone
        if let (Some((source, _)), Some((target, _))) = (get_time("source", fields), get_time("target", fields)) {
            let diff = if target > source { target - source } else { source - target};
            return Ok(TokenType::Duration(diff));
        }
        
        //todo: calculate with timezone
        return match (get_date("source", fields), get_date("target", fields)) {
            (Some((source, _)), Some((target, _))) => {
                let diff = if target > source { target - source } else { source - target};
                return Ok(TokenType::Duration(diff));
            },
            _ => Err("Time information not valid".to_string())
        }
    }

    Err("Time diff not valid".to_string())
}

#[cfg(test)]
#[test]
fn duration_parse_test_1() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("10 days".to_string());

    assert_eq!(tokens.len(), 3);
    
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Duration(Duration::days(10))));
}

#[cfg(test)]
#[test]
fn duration_parse_test_2() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("10 weeks".to_string());

    assert_eq!(tokens.len(), 3);
    
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Duration(Duration::weeks(10))));
}

#[cfg(test)]
#[test]
fn duration_parse_test_3() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("60 minutes".to_string());

    assert_eq!(tokens.len(), 3);
    
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Duration(Duration::minutes(60))));
}

#[cfg(test)]
#[test]
fn duration_parse_test_4() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("5 weeks as seconds".to_string());
    assert_eq!(tokens.len(), 6);
    
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Duration(Duration::seconds(3024000))));
}

#[cfg(test)]
#[test]
fn duration_parse_test_5() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("48 weeks as hours".to_string());

    assert_eq!(tokens.len(), 6);
    
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Duration(Duration::hours(8064))));
}

#[cfg(test)]
#[test]
fn duration_parse_test_6() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("11:50 as hour".to_string());

    assert_eq!(tokens.len(), 4);
    
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Duration(Duration::hours(11))));
}

#[cfg(test)]
#[test]
fn duration_parse_test_7() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("2 week 5 hours as hours".to_string());

    assert_eq!(tokens.len(), 10);
    
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Duration(Duration::hours(341))));
}

#[cfg(test)]
#[test]
fn to_duration_1() {
    use core::ops::Deref;
    use crate::tokinizer::test::execute;
    
    let tokens = execute("17:30 to 20:45".to_string());

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Duration(Duration::seconds(11700))));
}

#[cfg(test)]
#[test]
fn to_duration_2() {
    use crate::tokinizer::test::execute;
    
    let tokens = execute("20:45 to 17:30".to_string());

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Duration(Duration::seconds(11700))));
}