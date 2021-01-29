use std::collections::HashMap;
use std::fs;

use serde_json::{from_str, Result, Value};

use chrono::{Utc, Duration};
use chrono_tz::Tz;

use crate::types::{Token, TokenType, BramaAstType};

pub fn hour_add(fields: &HashMap<String, &Token>) -> std::result::Result<TokenType, String> {
    if fields.contains_key("time") && fields.contains_key("hour") {
        let time_info = match &fields.get("time").unwrap().token {
            TokenType::Time(time) => time,
            TokenType::Variable(variable) => {
                match &*variable.data {
                    BramaAstType::Time(time) => time,
                    _ => return Err("Time not valid".to_string())
                }
            },
            _ => return Err("Time not valid".to_string())
        };

        if let TokenType::Number(hours) = fields.get("hour").unwrap().token {
            let time = *time_info + Duration::seconds(hours as i64 * 60 * 60);
            return Ok(TokenType::Time(time));
        }
    }

    Err("Time format not valid".to_string())
}

pub fn time_for_location(atoms: &HashMap<String, &Token>) -> std::result::Result<TokenType, String> {

    if let TokenType::Text(location) = &atoms.get("location").unwrap().token {
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
    }

    Err("Location not found".to_string())
}

#[cfg(test)]
#[test]
fn hour_add_test_1() {
    let mut map: HashMap<String, &Token> = HashMap::new();
    let current_time = Utc::now().naive_local().time();
    let time_token   = TokenType::Time(current_time);
    let hours_token  = TokenType::Number(1.0);

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
    let mut map: HashMap<String, &Token> = HashMap::new();
    let current_time = Utc::now().naive_local().time();
    let time_token   = TokenType::Time(current_time);
    let hours_token  = TokenType::Number(-1.0);

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
    let mut map: HashMap<String, &Token> = HashMap::new();
    let current_time = Utc::now().naive_local().time();
    let time_token   = TokenType::Time(current_time);
    let hours_token  = TokenType::Number(0.0);

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
