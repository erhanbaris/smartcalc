/*
 * smartcalc v1.0.4
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::string::String;
use alloc::string::ToString;
use alloc::sync::Arc;
use chrono::FixedOffset;
use chrono::TimeZone;
use core::ops::Deref;

use alloc::collections::btree_map::BTreeMap;

use crate::config::SmartCalcConfig;
use crate::types::TimeOffset;
use crate::worker::tools::get_timezone;
use crate::worker::tools::get_time;
use crate::{tokinizer::Tokinizer, types::{TokenType}};
use crate::tokinizer::{TokenInfo};

pub fn time_for_location(_: &SmartCalcConfig, _: &Tokinizer, atoms: &BTreeMap<String, Arc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if let Some(TokenType::Text(_location)) = &atoms.get("location").unwrap().token_type.borrow().deref()  {
        /*{
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
                    //println!("{}", error);
                    Err("Internal error".to_string())
                }
            };
        }*/
    }

    Err("Location not found".to_string())
}

pub fn time_with_timezone(_: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Arc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("time") && fields.contains_key("timezone") {
        
        let (time, current_offset) = get_time("time", &fields).unwrap();
        let (target_timezone, target_offset) = get_timezone("timezone", &fields).unwrap();

        // To source timezone
        let timezone_offset = FixedOffset::east(current_offset.offset * 60);
        let date_with_timezone = timezone_offset.from_utc_datetime(&time);
        let new_time = chrono::Local.from_local_datetime(&date_with_timezone.naive_local()).unwrap().naive_local();

        // To target timezone
        let timezone_offset = FixedOffset::east(target_offset * 60);
        let date_with_timezone = timezone_offset.from_local_datetime(&new_time).unwrap();
        let new_time = chrono::Utc.from_utc_datetime(&date_with_timezone.naive_utc()).naive_utc();

        return Ok(TokenType::Time(new_time, TimeOffset { 
            name: target_timezone.to_uppercase(),
            offset: target_offset
        }));
    }
    Err("Timezone or time informations not found".to_string())
}

pub fn convert_timezone(_: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Arc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("time") && fields.contains_key("timezone") {

        let (time, _) = get_time("time", &fields).unwrap();
        let (target_timezone, target_offset) = get_timezone("timezone", &fields).unwrap();

        return Ok(TokenType::Time(time, TimeOffset { 
            name: target_timezone.to_uppercase(),
            offset: target_offset
        }));
    }
    Err("Timezone or time informations not found".to_string())
}
