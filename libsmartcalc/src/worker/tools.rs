
use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;
use chrono::{Duration, NaiveTime, NaiveDate};

use crate::config::SmartCalcConfig;
use crate::{types::{TokenType, BramaAstType}};
use crate::tokinizer::{TokenInfo};

pub fn read_currency<'a>(config: &SmartCalcConfig, currency: &'a str) -> Option<String> {
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

pub fn get_number<'a>(config: &SmartCalcConfig, field_name: &'a str, fields: &BTreeMap<String, &TokenInfo>) -> Option<f64> {
    return match fields.get(field_name) {
        Some(data) => match &data.token_type {
            Some(token) => match &token {
                TokenType::Number(number) => Some(*number),
                TokenType::Variable(variable) => {
                    match &*variable.data {
                        BramaAstType::Number(number) => Some(*number),
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
                    match &*variable.data {
                        BramaAstType::Duration(duration) => Some(*duration),
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
                    match &*variable.data {
                        BramaAstType::Time(time) => Some(*time),
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
                    match &*variable.data {
                        BramaAstType::Date(date) => Some(*date),
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
            Some(token) => match &token {
                TokenType::Text(text) => Some(text.to_string()),
                _ => None
            },
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
                    match &*variable.data {
                        BramaAstType::Month(number) => Some(*number),
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
    match get_number(config, field_name, fields) {
        Some(number) => Some(number),
        None => match get_money(config, field_name, fields) {
            Some((price, _)) => Some(price),
            None => None
        }
    }
}

pub fn get_number_or_month<'a>(config: &SmartCalcConfig, field_name: &'a str, fields: &BTreeMap<String, &TokenInfo>) -> Option<u32> {
    match get_number(config, field_name, fields) {
        Some(number) => Some(number as u32),
        None => match get_month(field_name, fields) {
            Some(month) => Some(month),
            None => None
        }
    }
}


pub fn get_money<'a>(config: &SmartCalcConfig, field_name: &'a str, fields: &BTreeMap<String, &TokenInfo>) -> Option<(f64, String)> {
    return match &fields.get(field_name) {
        Some(data) => match &data.token_type {
            Some(token) => match &token {
                TokenType::Money(price, currency) => {
                    match read_currency(config, currency) {
                        Some(real_currency) => Some((*price, real_currency.to_string())),
                        _ => None
                    }
                },
                TokenType::Variable(variable) => {
                    match &*variable.data {
                        BramaAstType::Money(price, currency) => {
                            match read_currency(config, currency) {
                                Some(real_currency) => Some((*price, real_currency.to_string())),
                                _ => None
                            }
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
                    match &*variable.data {
                        BramaAstType::Percent(percent) => Some(*percent),
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