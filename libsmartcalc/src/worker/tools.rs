
use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;
use chrono::{Duration, NaiveTime, NaiveDate};

use crate::config::SmartCalcConfig;
use crate::{types::{TokenType, BramaAstType}};
use crate::tokinizer::{TokenInfo};

pub fn read_currency(config: &SmartCalcConfig, currency: &'_ str) -> Option<String> {
    match config.currency_alias.get(&currency.to_lowercase()) {
        Some(symbol) => Some(symbol.to_lowercase()),
        _ => {
            match config.currency.get(&currency.to_lowercase()) {
                Some(_) => Some(currency.to_lowercase()),
                _ => None
            }
        }
    }
}

pub fn get_number<'a>(field_name: &'a str, fields: &BTreeMap<String, &TokenInfo>) -> Option<f64> {
    return match fields.get(field_name) {
        Some(data) => match &data.token_type {
            Some(token) => match &token {
                TokenType::Number(number) => Some(*number),
                TokenType::Variable(variable) => {
                    match **variable.data.borrow() {
                        BramaAstType::Number(number) => Some(number),
                        _ => None
                    }
                },
                _ => None
            },
            _ => None
        },
        _ => None
    }
}

pub fn get_duration<'a>(field_name: &'a str, fields: &BTreeMap<String, &TokenInfo>) -> Option<Duration> {
    return match fields.get(field_name) {
        Some(data) => match &data.token_type {
            Some(token) => match &token {
                TokenType::Duration(duration) => Some(*duration),
                TokenType::Variable(variable) => {
                    match **variable.data.borrow() {
                        BramaAstType::Duration(duration) => Some(duration),
                        _ => None
                    }
                },
                _ => None
            },
            _ => None
        },
        _ => None
    }
}

pub fn get_time<'a>(field_name: &'a str, fields: &BTreeMap<String, &TokenInfo>) -> Option<NaiveTime> {
    return match fields.get(field_name) {
        Some(data) => match &data.token_type {
            Some(token) => match &token {
                TokenType::Time(time) => Some(*time),
                TokenType::Variable(variable) => {
                    match **variable.data.borrow() {
                        BramaAstType::Time(time) => Some(time),
                        _ => None
                    }
                },
                _ => None
            },
            _ => None
        },
        _ => None
    }
}

pub fn get_date<'a>(field_name: &'a str, fields: &BTreeMap<String, &TokenInfo>) -> Option<NaiveDate> {
    return match fields.get(field_name) {
        Some(data) => match &data.token_type {
            Some(token) => match &token {
                TokenType::Date(date) => Some(*date),
                TokenType::Variable(variable) => {
                    match **variable.data.borrow() {
                        BramaAstType::Date(date) => Some(date),
                        _ => None
                    }
                },
                _ => None
            },
            _ => None
        },
        _ => None
    }
}

pub fn get_text<'a>(field_name: &'a str, fields: &BTreeMap<String, &TokenInfo>) -> Option<String> {
    return match fields.get(field_name) {
        Some(data) => match &data.token_type {
            Some(TokenType::Text(text)) =>  Some(text.to_string()),
            _ => None
        },
        _ => None
    }
}

pub fn get_month<'a>(field_name: &'a str, fields: &BTreeMap<String, &TokenInfo>) -> Option<u32> {
    return match &fields.get(field_name) {
        Some(data) =>match &data.token_type {
            Some(token) => match &token {
                TokenType::Month(number) => Some(*number),
                TokenType::Variable(variable) => {
                    match **variable.data.borrow() {
                        BramaAstType::Month(number) => Some(number),
                        _ => None
                    }
                },
                _ => None
            },
            _ => None
        },
        _ => None
    }
}


pub fn get_number_or_price<'a>(config: &SmartCalcConfig, field_name: &'a str, fields: &BTreeMap<String, &TokenInfo>) -> Option<f64> {
    match get_number(field_name, fields) {
        Some(number) => Some(number),
        None => get_money(config, field_name, fields).map(|(price, _)| price)
    }
}

pub fn get_number_or_month<'a>(field_name: &'a str, fields: &BTreeMap<String, &TokenInfo>) -> Option<u32> {
    match get_number(field_name, fields) {
        Some(number) => Some(number as u32),
        None => get_month(field_name, fields)
    }
}


pub fn get_money<'a>(config: &SmartCalcConfig, field_name: &'a str, fields: &BTreeMap<String, &TokenInfo>) -> Option<(f64, String)> {
    return match &fields.get(field_name) {
        Some(data) => match &data.token_type {
            Some(token) => match &token {
                TokenType::Money(price, currency) => {
                    read_currency(config, currency).map(|real_currency| (*price, real_currency))
                },
                TokenType::Variable(variable) => {
                    match &**variable.data.borrow() {
                        BramaAstType::Money(price, currency) => {
                            read_currency(config, &currency).map(|real_currency| (*price, real_currency))
                        },
                        _ => None
                    }
                },
                _ => None
            },
            _ => None
        },
        _ => None
    }
}

pub fn get_currency<'a>(config: &SmartCalcConfig, field_name: &'a str, fields: &BTreeMap<String, &TokenInfo>) -> Option<String> {
    match &fields.get(field_name) {
        Some(data) => match &data.token_type {
            Some(token) => match &token {
                TokenType::Text(currency) => read_currency(config, currency),
                TokenType::Money(_, currency) => read_currency(config, currency),
                _ => None
            },
            _ => None
        },
        _ => None
    }
}

pub fn get_percent<'a>(field_name: &'a str, fields: &BTreeMap<String, &TokenInfo>) -> Option<f64> {
    match &fields.get(field_name) {
        Some(data) => match &data.token_type {
            Some(token) => match &token {
                TokenType::Percent(percent) => Some(*percent),
                TokenType::Variable(variable) => {
                    match **variable.data.borrow() {
                        BramaAstType::Percent(percent) => Some(percent),
                        _ => None
                    }
                },
                _ => None
            },
            _ => None
        },
        _ => None
    }
}