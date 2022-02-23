/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;
use alloc::sync::Arc;
use crate::compiler::date::DateItem;
use crate::compiler::duration::DurationItem;
use crate::compiler::memory::MemoryItem;
use crate::compiler::number::NumberItem;
use crate::compiler::percent::PercentItem;
use crate::compiler::DataItem;
use crate::compiler::time::TimeItem;
use crate::types::MemoryType;
use core::ops::Deref;
use chrono::{Duration, NaiveTime, NaiveDate};

use crate::config::SmartCalcConfig;
use crate::types::CurrencyInfo;
use crate::types::Money;
use crate::types::{TokenType, BramaAstType};
use crate::tokinizer::TokenInfo;
use crate::compiler::money::MoneyItem;

pub fn read_currency(config: &SmartCalcConfig, currency: &'_ str) -> Option<Arc<CurrencyInfo>> {
    match config.currency_alias.get(&currency.to_lowercase()[..]) {
        Some(symbol) => Some(symbol.clone()),
        _ => {
            match config.currency.get(&currency.to_lowercase()[..]) {
                Some(symbol) => Some(symbol.clone()),
                _ => None
            }
        }
    }
}

pub fn get_number<'a>(field_name: &'a str, fields: &BTreeMap<String, Arc<TokenInfo>>) -> Option<f64> {
    return match fields.get(field_name) {
        Some(data) => match data.token_type.borrow().deref() {
            Some(token) => match &token {
                TokenType::Number(number) => Some(*number),
                TokenType::Variable(variable) => {
                    match variable.data.borrow().deref().deref() {
                        BramaAstType::Item(item) => match item.as_any().downcast_ref::<NumberItem>() {
                            Some(number) => Some(number.get_underlying_number()),
                            _ => None
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

pub fn get_duration<'a>(field_name: &'a str, fields: &BTreeMap<String, Arc<TokenInfo>>) -> Option<Duration> {
    return match fields.get(field_name) {
        Some(data) => match &data.token_type.borrow().deref() {
            Some(token) => match &token {
                TokenType::Duration(duration) => Some(*duration),
                TokenType::Variable(variable) => {
                    match variable.data.borrow().deref().deref() {
                        BramaAstType::Item(item) => match item.as_any().downcast_ref::<DurationItem>() {
                            Some(number) => Some(number.get_duration()),
                            _ => None
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

pub fn get_time<'a>(field_name: &'a str, fields: &BTreeMap<String, Arc<TokenInfo>>) -> Option<NaiveTime> {
    return match fields.get(field_name) {
        Some(data) => match &data.token_type.borrow().deref() {
            Some(token) => match &token {
                TokenType::Time(time) => Some(*time),
                TokenType::Variable(variable) => {
                    match variable.data.borrow().deref().deref() {
                        BramaAstType::Item(item) => match item.as_any().downcast_ref::<TimeItem>() {
                            Some(time_item) => Some(time_item.get_time()),
                            _ => None
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

pub fn get_date<'a>(field_name: &'a str, fields: &BTreeMap<String, Arc<TokenInfo>>) -> Option<NaiveDate> {
    return match fields.get(field_name) {
        Some(data) => match &data.token_type.borrow().deref() {
            Some(token) => match &token {
                TokenType::Date(date) => Some(*date),
                TokenType::Variable(variable) => {
                    match variable.data.borrow().deref().deref() {
                        BramaAstType::Item(item) => match item.as_any().downcast_ref::<DateItem>() {
                            Some(date_item) => Some(date_item.get_date()),
                            _ => None
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

pub fn get_text<'a>(field_name: &'a str, fields: &BTreeMap<String, Arc<TokenInfo>>) -> Option<String> {
    return match fields.get(field_name) {
        Some(data) => match &data.token_type.borrow().deref() {
            Some(TokenType::Text(text)) =>  Some(text.to_string()),
            _ => None
        },
        _ => None
    }
}

pub fn get_month<'a>(field_name: &'a str, fields: &BTreeMap<String, Arc<TokenInfo>>) -> Option<u32> {
    return match &fields.get(field_name) {
        Some(data) =>match &data.token_type.borrow().deref() {
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

pub fn get_memory<'a>(field_name: &'a str, fields: &BTreeMap<String, Arc<TokenInfo>>) -> Option<(f64, MemoryType)> {
    return match &fields.get(field_name) {
        Some(data) =>match &data.token_type.borrow().deref() {
            Some(token) => match &token {
                TokenType::Memory(memory, memory_type) => Some((*memory, memory_type.clone())),
                TokenType::Variable(variable) => {
                    match variable.data.borrow().deref().deref() {
                        BramaAstType::Item(item) => match item.as_any().downcast_ref::<MemoryItem>() {
                            Some(memory_item) => Some((memory_item.get_memory(), memory_item.get_memory_type())),
                            _ => None
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

pub fn get_number_or_time<'a>(field_name: &'a str, fields: &BTreeMap<String, Arc<TokenInfo>>) -> Option<NaiveTime> {
    match get_number(field_name, fields) {
        Some(number) => Some(NaiveTime::from_hms(number as u32, 0, 0)),
        None => get_time(field_name, fields)
    }
}

pub fn get_number_or_price<'a>(config: &SmartCalcConfig, field_name: &'a str, fields: &BTreeMap<String, Arc<TokenInfo>>) -> Option<f64> {
    match get_number(field_name, fields) {
        Some(number) => Some(number),
        None => get_money(config, field_name, fields).map(|money| money.get_price())
    }
}

pub fn get_number_or_month<'a>(field_name: &'a str, fields: &BTreeMap<String, Arc<TokenInfo>>) -> Option<u32> {
    match get_number(field_name, fields) {
        Some(number) => Some(number as u32),
        None => get_month(field_name, fields)
    }
}


pub fn get_money<'a>(_: &SmartCalcConfig, field_name: &'a str, fields: &BTreeMap<String, Arc<TokenInfo>>) -> Option<Money> {
    return match &fields.get(field_name) {
        Some(data) => match &data.token_type.borrow().deref() {
            Some(token) => match &token {
                TokenType::Money(price, currency) => Some(Money(*price, currency.clone())),
                TokenType::Variable(variable) => {
                    match variable.data.borrow().deref().deref() {
                        BramaAstType::Item(item) => match item.as_any().downcast_ref::<MoneyItem>() {
                            Some(money_item) => Some(Money(money_item.get_price(), money_item.get_currency())),
                            _ => None
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

pub fn get_currency<'a>(config: &SmartCalcConfig, field_name: &'a str, fields: &BTreeMap<String, Arc<TokenInfo>>) -> Option<Arc<CurrencyInfo>> {
    match &fields.get(field_name) {
        Some(data) => match &data.token_type.borrow().deref() {
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

pub fn get_percent<'a>(field_name: &'a str, fields: &BTreeMap<String, Arc<TokenInfo>>) -> Option<f64> {
    match &fields.get(field_name) {
        Some(data) => match &data.token_type.borrow().deref() {
            Some(token) => match &token {
                TokenType::Percent(percent) => Some(*percent),
                TokenType::Variable(variable) => {
                    match variable.data.borrow().deref().deref() {
                        BramaAstType::Item(item) => match item.as_any().downcast_ref::<PercentItem>() {
                            Some(percent_item) => Some(percent_item.get_underlying_number()),
                            _ => None
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