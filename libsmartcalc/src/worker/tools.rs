use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;
use alloc::sync::Arc;
use chrono::{Duration, NaiveTime, NaiveDate};

use crate::config::SmartCalcConfig;
use crate::types::CurrencyInfo;
use crate::types::Money;
use crate::{types::{TokenType, BramaAstType}};
use crate::tokinizer::{TokenInfo};

pub fn read_currency(config: &SmartCalcConfig, currency: &'_ str) -> Option<Arc<CurrencyInfo>> {
    match config.currency_alias.get(&currency.to_lowercase()) {
        Some(symbol) => Some(symbol.clone()),
        _ => {
            match config.currency.get(&currency.to_lowercase()) {
                Some(symbol) => Some(symbol.clone()),
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
        None => get_money(config, field_name, fields).map(|money| money.get_price())
    }
}

pub fn get_number_or_month<'a>(field_name: &'a str, fields: &BTreeMap<String, &TokenInfo>) -> Option<u32> {
    match get_number(field_name, fields) {
        Some(number) => Some(number as u32),
        None => get_month(field_name, fields)
    }
}


pub fn get_money<'a>(_: &SmartCalcConfig, field_name: &'a str, fields: &BTreeMap<String, &TokenInfo>) -> Option<Money> {
    return match &fields.get(field_name) {
        Some(data) => match &data.token_type {
            Some(token) => match &token {
                TokenType::Money(price, currency) => Some(Money(*price, currency.clone())),
                TokenType::Variable(variable) => {
                    match &**variable.data.borrow() {
                        BramaAstType::Money(price, currency) => Some(Money(*price, currency.clone())),
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

pub fn get_currency<'a>(config: &SmartCalcConfig, field_name: &'a str, fields: &BTreeMap<String, &TokenInfo>) -> Option<Arc<CurrencyInfo>> {
    match &fields.get(field_name) {
        Some(data) => match &data.token_type {
            Some(token) => match &token {
                TokenType::Text(currency) => read_currency(config, currency),
                TokenType::Money(_, currency) => Some(currency.clone()),
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