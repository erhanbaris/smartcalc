use std::collections::HashMap;
use std::fs;

use serde_json::{from_str, Result, Value};

use chrono::{Utc, Duration};
use chrono_tz::Tz;

use crate::types::{Token, TokenType, BramaAstType};
use crate::tokinizer::TokenLocation;

pub fn hour_add(fields: &HashMap<String, &TokenLocation>) -> std::result::Result<TokenType, String> {
    if fields.contains_key("time") && fields.contains_key("hour") {
        let time_info = match &fields.get("time").unwrap().token_type {
            Some(token) => match &token {
                TokenType::Time(time) => time,
                TokenType::Variable(variable) => {
                    match &*variable.data {
                        BramaAstType::Time(time) => time,
                        _ => return Err("Time not valid".to_string())
                    }
                },
                _ => return Err("Time not valid".to_string())
            },
            _ => return Err("Time not valid".to_string())
        };

        match &fields.get("hour").unwrap().token_type {
            Some(token) => match &token {
                TokenType::Number(hours) => {
                    let time = *time_info + Duration::seconds(*hours as i64 * 60 * 60);
                    return Ok(TokenType::Time(time));
                },
                _ => ()
            },
            _ => ()
        };
    }

    Err("Time format not valid".to_string())
}

pub fn time_for_location(atoms: &HashMap<String, &TokenLocation>) -> std::result::Result<TokenType, String> {
    match &atoms.get("location").unwrap().token_type {
        Some(TokenType::Text(location)) => {
            let json_data = fs::read_to_string("/Users/erhanbaris/ClionProjects/smartcalculator/smartcalc/src/json/city_informations.json").expect("{}");
            let json_value: Result<Value> = from_str(&json_data);

            return match json_value {
                Ok(data) => {
                    for item in data.as_array().unwrap() {
                        if let Value::String(city) = item.get("city_ascii").unwrap() {

                            if city.to_lowercase() == location.to_lowercase() {
                                let timezone = item.get("timezone").unwrap().as_str().unwrap();
                                let tz: Tz = match timezone.parse() {
                                    Ok(v) => v,
                                    Err(_) => return Err("Time not found".to_string())
                                };
                                return Ok(TokenType::Time(Utc::now().with_timezone(&tz).naive_local().time()));
                            }
                        }
                    }

                    Err("Time not found".to_string())
                },
                Err(error) => {
                    println!("{}", error);
                    Err("Internal error".to_string())
                }
            };
        },
        _ => ()
    };

    Err("Location not found".to_string())
}

#[cfg(test)]
#[test]
fn hour_add_test_1() {
    let mut map: HashMap<String, &TokenLocation> = HashMap::new();
    let current_time = Utc::now().naive_local().time();
    let time_token   = TokenLocation {
        start: 0,
        end: 5,
        token_type: Some(TokenType::Time(current_time)),
        original_text: "".to_string()
    };
    let hours_token  = TokenLocation {
        start: 0,
        end: 5,
        token_type: Some(TokenType::Number(1.0)),
        original_text: "".to_string()
    };

    map.insert("time".to_string(),  &time_token);
    map.insert("hour".to_string(), &hours_token);

    let result = hour_add(&map);
    match result {
        Ok(token) => {
            if let TokenType::Time(time) = token {
                assert!(time - current_time == Duration::hours(1));
            }
            else {
                assert!(false)
            }
        },
        _ => assert!(false)
    }
}

#[cfg(test)]
#[test]
fn hour_add_test_2() {
    let mut map: HashMap<String, &TokenLocation> = HashMap::new();
    let current_time = Utc::now().naive_local().time();
    let time_token   = TokenLocation {
        start: 0,
        end: 5,
        token_type: Some(TokenType::Time(current_time)),
        original_text: "".to_string()
    };
    let hours_token  = TokenLocation {
        start: 0,
        end: 5,
        token_type: Some(TokenType::Number(-1.0)),
        original_text: "".to_string()
    };

    map.insert("time".to_string(),  &time_token);
    map.insert("hour".to_string(), &hours_token);

    let result = hour_add(&map);
    match result {
        Ok(token) => {
            if let TokenType::Time(time) = token {
                assert!(time - current_time == Duration::hours(-1));
            }
            else {
                assert!(false)
            }
        },
        _ => assert!(false)
    }
}

#[cfg(test)]
#[test]
fn hour_add_test_3() {
    let mut map: HashMap<String, &TokenLocation> = HashMap::new();
    let current_time = Utc::now().naive_local().time();
    let time_token   = TokenLocation {
        start: 0,
        end: 5,
        token_type: Some(TokenType::Time(current_time)),
        original_text: "".to_string()
    };
    let hours_token  = TokenLocation {
        start: 0,
        end: 5,
        token_type: Some(TokenType::Number(0.0)),
        original_text: "".to_string()
    };

    map.insert("time".to_string(),  &time_token);
    map.insert("hour".to_string(), &hours_token);

    let result = hour_add(&map);
    match result {
        Ok(token) => {
            if let TokenType::Time(time) = token {
                assert!(time - current_time == Duration::hours(0));
            }
            else {
                assert!(false)
            }
        },
        _ => assert!(false)
    }
}
