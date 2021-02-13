
use alloc::string::String;
use alloc::string::ToString;
use alloc::collections::btree_map::BTreeMap;

use crate::{types::{TokenType, BramaAstType}};
use crate::tokinizer::{TokenLocation};
use crate::constants::{CURRENCIES};

pub fn read_currency(currency: String) -> Option<String> {
    match CURRENCIES.read().unwrap().get(&currency.to_lowercase()) {
        Some(symbol) => Some(symbol.to_lowercase()),
        _ => None
    }
}

pub fn get_number(field_name: String, fields: &BTreeMap<String, &TokenLocation>) -> Option<f64> {
    return match &fields.get(&field_name).unwrap().token_type {
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
    }
}


pub fn get_money(field_name: String, fields: &BTreeMap<String, &TokenLocation>) -> Option<(f64, String)> {
    return match &fields.get(&field_name).unwrap().token_type {
        Some(token) => match &token {
            TokenType::Money(price, currency) => {
                match read_currency(currency.to_string()) {
                    Some(real_currency) => Some((*price, real_currency.to_string())),
                    _ => None
                }
            },
            TokenType::Variable(variable) => {
                match &*variable.data {
                    BramaAstType::Money(price, currency) => {
                        match read_currency(currency.to_string()) {
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
    }
}

pub fn get_currency(field_name: String, fields: &BTreeMap<String, &TokenLocation>) -> Option<String> {
    return match &fields.get(&field_name).unwrap().token_type {
        Some(token) => match &token {
            TokenType::Text(currency) => read_currency(currency.to_string()),
            _ => None
        },
        _ => None
    }
}

pub fn get_percent(field_name: String, fields: &BTreeMap<String, &TokenLocation>) -> Option<f64> {
    return match &fields.get(&field_name).unwrap().token_type {
        Some(token) => match &token {
            TokenType::Percent(percent) => Some(*percent),
            _ => None
        },
        _ => None
    }
}
