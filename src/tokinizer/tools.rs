/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;
use chrono::NaiveDateTime;
use chrono::Utc;
use crate::compiler::date::DateItem;
use crate::compiler::date_time::DateTimeItem;
use crate::compiler::duration::DurationItem;
use crate::compiler::number::NumberItem;
use crate::compiler::percent::PercentItem;
use crate::compiler::dynamic_type::DynamicTypeItem;
use crate::compiler::DataItem;
use crate::compiler::time::TimeItem;
use crate::types::TimeOffset;
use core::ops::Deref;
use chrono::{Duration, NaiveDate};

use crate::config::SmartCalcConfig;
use crate::config::DynamicType;
use crate::types::CurrencyInfo;
use crate::types::Money;
use crate::types::{TokenType, SmartCalcAstType};
use crate::tokinizer::TokenInfo;
use crate::compiler::money::MoneyItem;

pub fn read_currency(config: &SmartCalcConfig, currency: &'_ str) -> Option<Rc<CurrencyInfo>> {
    match config.currency_alias.get(&currency.to_lowercase()) {
        Some(symbol) => Some(symbol.clone()),
        _ => config.currency.get(&currency.to_lowercase()).cloned()
    }
}

pub fn get_number(field_name: &str, fields: &BTreeMap<String, Rc<TokenInfo>>) -> Option<f64> {
    return match fields.get(field_name) {
        Some(data) => match data.token_type.borrow().deref() {
            Some(token) => match &token {
                TokenType::Number(number, _) => Some(*number),
                TokenType::Variable(variable) => {
                    match variable.data.borrow().deref().deref() {
                        SmartCalcAstType::Item(item) => item.as_any().downcast_ref::<NumberItem>().map(|number| number.get_underlying_number()),
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

pub fn get_duration(field_name: &str, fields: &BTreeMap<String, Rc<TokenInfo>>) -> Option<Duration> {
    return match fields.get(field_name) {
        Some(data) => match &data.token_type.borrow().deref() {
            Some(token) => match &token {
                TokenType::Duration(duration) => Some(*duration),
                TokenType::Variable(variable) => {
                    match variable.data.borrow().deref().deref() {
                        SmartCalcAstType::Item(item) => item.as_any().downcast_ref::<DurationItem>().map(|number| number.get_duration()),
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

pub fn get_time(field_name: &str, fields: &BTreeMap<String, Rc<TokenInfo>>) -> Option<(NaiveDateTime, TimeOffset)> {
    return match fields.get(field_name) {
        Some(data) => match &data.token_type.borrow().deref() {
            Some(token) => match &token {
                TokenType::Time(time, tz) => Some((*time, tz.clone())),
                TokenType::Variable(variable) => {
                    match variable.data.borrow().deref().deref() {
                        SmartCalcAstType::Item(item) => item.as_any().downcast_ref::<TimeItem>().map(|time_item| (time_item.get_time(), time_item.get_tz())),
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

pub fn get_date(field_name: &str, fields: &BTreeMap<String, Rc<TokenInfo>>) -> Option<(NaiveDate, TimeOffset)> {
    return match fields.get(field_name) {
        Some(data) => match &data.token_type.borrow().deref() {
            Some(token) => match &token {
                TokenType::Date(date, tz) => Some((*date, tz.clone())),
                TokenType::Variable(variable) => {
                    match variable.data.borrow().deref().deref() {
                        SmartCalcAstType::Item(item) => item.as_any().downcast_ref::<DateItem>().map(|date_item| (date_item.get_date(), date_item.get_tz())),
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


pub fn get_date_time(field_name: &str, fields: &BTreeMap<String, Rc<TokenInfo>>) -> Option<(NaiveDateTime, TimeOffset)> {
    return match fields.get(field_name) {
        Some(data) => match &data.token_type.borrow().deref() {
            Some(token) => match &token {
                TokenType::DateTime(date, tz) => Some((*date, tz.clone())),
                TokenType::Variable(variable) => {
                    match variable.data.borrow().deref().deref() {
                        SmartCalcAstType::Item(item) => item.as_any().downcast_ref::<DateTimeItem>().map(|date_item| (date_item.get_date_time(), date_item.get_tz())),
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

pub fn get_text(field_name: &str, fields: &BTreeMap<String, Rc<TokenInfo>>) -> Option<String> {
    return match fields.get(field_name) {
        Some(data) => match &data.token_type.borrow().deref() {
            Some(TokenType::Text(text)) =>  Some(text.to_string()),
            _ => None
        },
        _ => None
    }
}

pub fn get_dynamic_type(field_name: &str, fields: &BTreeMap<String, Rc<TokenInfo>>) -> Option<(f64, Rc<DynamicType>)> {
    return match &fields.get(field_name) {
        Some(data) =>match &data.token_type.borrow().deref() {
            Some(token) => match &token {
                TokenType::DynamicType(number, dynamic_type) => Some((*number, dynamic_type.clone())),
                TokenType::Variable(variable) => {
                    match variable.data.borrow().deref().deref() {
                        SmartCalcAstType::Item(item) => item.as_any().downcast_ref::<DynamicTypeItem>().map(|dynamic_type| (dynamic_type.get_number(), dynamic_type.get_type())),
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

pub fn get_timezone(field_name: &str, fields: &BTreeMap<String, Rc<TokenInfo>>) -> Option<(String, i32)> {
    return match fields.get(field_name) {
        Some(data) => match &data.token_type.borrow().deref() {
            Some(TokenType::Timezone(timezone, offset)) =>  Some((timezone.to_string(), *offset)),
            _ => None
        },
        _ => None
    }
}

pub fn get_month(field_name: &str, fields: &BTreeMap<String, Rc<TokenInfo>>) -> Option<u32> {
    return match &fields.get(field_name) {
        Some(data) =>match &data.token_type.borrow().deref() {
            Some(token) => match &token {
                TokenType::Month(number) => Some(*number),
                TokenType::Variable(variable) => {
                    match **variable.data.borrow() {
                        SmartCalcAstType::Month(number) => Some(number),
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

pub fn get_number_or_time(config: &SmartCalcConfig, field_name: &str, fields: &BTreeMap<String, Rc<TokenInfo>>) -> Option<(NaiveDateTime, TimeOffset)> {
    match get_number(field_name, fields) {
        Some(number) => {
            let date = Utc::now().naive_local().date();
            let time = chrono::NaiveTime::from_hms(number as u32, 0, 0);
            Some((NaiveDateTime::new(date, time), config.get_time_offset()))
        },
        None => get_time(field_name, fields)
    }
}

pub fn get_number_or_price(config: &SmartCalcConfig, field_name: &str, fields: &BTreeMap<String, Rc<TokenInfo>>) -> Option<f64> {
    match get_number(field_name, fields) {
        Some(number) => Some(number),
        None => get_money(config, field_name, fields).map(|money| money.get_price())
    }
}

pub fn get_number_or_month(field_name: &str, fields: &BTreeMap<String, Rc<TokenInfo>>) -> Option<u32> {
    match get_number(field_name, fields) {
        Some(number) => Some(number as u32),
        None => get_month(field_name, fields)
    }
}


pub fn get_money(_: &SmartCalcConfig, field_name: &str, fields: &BTreeMap<String, Rc<TokenInfo>>) -> Option<Money> {
    return match &fields.get(field_name) {
        Some(data) => match &data.token_type.borrow().deref() {
            Some(token) => match &token {
                TokenType::Money(price, currency) => Some(Money(*price, currency.clone())),
                TokenType::Variable(variable) => {
                    match variable.data.borrow().deref().deref() {
                        SmartCalcAstType::Item(item) => item.as_any().downcast_ref::<MoneyItem>().map(|money_item| Money(money_item.get_price(), money_item.get_currency())),
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

pub fn get_currency(config: &SmartCalcConfig, field_name: &str, fields: &BTreeMap<String, Rc<TokenInfo>>) -> Option<Rc<CurrencyInfo>> {
    match &fields.get(field_name) {
        Some(data) => match &data.token_type.borrow().deref() {
            Some(token) => match &token {
                TokenType::Text(currency) => read_currency(config, currency),
                TokenType::Money(_, currency) => Some(currency.clone()),
                TokenType::Variable(variable) => {
                    match variable.data.borrow().deref().deref() {
                        SmartCalcAstType::Item(item) => item.as_any().downcast_ref::<MoneyItem>().map(|money_item| money_item.get_currency()),
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

pub fn get_percent(field_name: &str, fields: &BTreeMap<String, Rc<TokenInfo>>) -> Option<f64> {
    match &fields.get(field_name) {
        Some(data) => match &data.token_type.borrow().deref() {
            Some(token) => match &token {
                TokenType::Percent(percent) => Some(*percent),
                TokenType::Variable(variable) => {
                    match variable.data.borrow().deref().deref() {
                        SmartCalcAstType::Item(item) => item.as_any().downcast_ref::<PercentItem>().map(|percent_item| percent_item.get_underlying_number()),
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
