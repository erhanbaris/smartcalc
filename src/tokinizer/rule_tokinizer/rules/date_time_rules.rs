/*
 * smartcalc v1.0.7
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use chrono::FixedOffset;
use chrono::NaiveDateTime;
use chrono::TimeZone;

use alloc::collections::btree_map::BTreeMap;

use crate::config::SmartCalcConfig;
use crate::tokinizer::get_date;
use crate::tokinizer::get_date_time;
use crate::tokinizer::get_number;
use crate::tokinizer::get_time;
use crate::tokinizer::get_timezone;
use crate::types::NumberType;
use crate::types::TimeOffset;
use crate::{tokinizer::Tokinizer, types::{TokenType}};
use crate::tokinizer::{TokenInfo};

pub fn time_with_timezone(_: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Rc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("time") && fields.contains_key("timezone") {
        
        let (time, current_offset) = get_time("time", fields).unwrap();
        let (target_timezone, target_offset) = get_timezone("timezone", fields).unwrap();

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

pub fn to_unixtime(_: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Rc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("data") {
        let timestamp = match get_time("data", fields) {
            Some((time, _)) => time.timestamp(),
            None => match get_date("data", fields) {
                Some((date, _)) => date.and_hms(0,0,0).timestamp(),
                None => match get_date_time("data", fields) {
                    Some((date_time, _)) => date_time.timestamp(),
                    None => 0
                }
            }
        };

        return Ok(TokenType::Number(timestamp as f64, NumberType::Raw));
    }
    Err("Date with time/date/time information not found".to_string())
}

pub fn from_unixtime(config: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Rc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("number") {
        let timestamp = get_number("number", fields).unwrap();
        let date = NaiveDateTime::from_timestamp(timestamp as i64, 0);
        
        return match get_timezone("timezone", fields) {
            Some((target_timezone, target_offset)) => Ok(TokenType::DateTime(date, TimeOffset { 
                name: target_timezone.to_uppercase(),
                offset: target_offset
            })),
            None => Ok(TokenType::DateTime(date, config.get_time_offset()))
        };
    }
    Err("Date with time/date/time information not found".to_string())
}

pub fn convert_timezone(_: &SmartCalcConfig, _: &Tokinizer, fields: &BTreeMap<String, Rc<TokenInfo>>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("time") && fields.contains_key("timezone") {
        let (target_timezone, target_offset) = get_timezone("timezone", fields).unwrap();
        let offset = TimeOffset { 
            name: target_timezone.to_uppercase(),
            offset: target_offset
        };
        
        return match get_time("time", fields) {
            Some((time, _)) => Ok(TokenType::Time(time, offset)),
            None => match get_date("time", fields) {
                Some((date, _)) => Ok(TokenType::Date(date, offset)),
                None => match get_date_time("time", fields) {
                    Some((date_time, _)) => Ok(TokenType::DateTime(date_time, offset)),
                    None => Err("Timezone or time informations not found".to_string())
                }
            }
        };
    }
    Err("Timezone or time informations not found".to_string())
}
