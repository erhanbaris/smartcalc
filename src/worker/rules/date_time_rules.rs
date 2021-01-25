use std::vec::Vec;
use std::collections::HashMap;
use std::rc::Rc;
use std::fs;

use serde_json::{from_str, Result, Value};

use chrono::{Utc, NaiveTime, Duration, NaiveDateTime};
use chrono_tz::Tz;

use crate::types::{Token};

pub fn date_sum(atoms: &HashMap<String, &Token>) -> std::result::Result<Token, String> {
    if let Token::Time(time) = atoms.get("time").unwrap() {
        if let Token::Number(hours) = atoms.get("hours").unwrap() {
            let time = *time + Duration::seconds(*hours as i64 * 60 * 60);
            return Ok(Token::Time(time));
        }
    }

    Err("Time format not valid".to_string())
}

pub fn time_for_location(atoms: &HashMap<String, &Token>) -> std::result::Result<Token, String> {

    if let Token::Text(location) = atoms.get("location").unwrap() {
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
                                Err(e) => return Err("Time not found".to_string())
                            };
                            return Ok(Token::Time(Utc::now().with_timezone(&tz).naive_local()));
                        }
                    }
                }

                Err("Time not found".to_string())
            },
            Err(error) => Err("Internal error".to_string())
        };
    }

    Err("Location not found".to_string())
}