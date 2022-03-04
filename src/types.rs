/*
 * smartcalc v1.0.4
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::sync::Arc;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::result::Result;
use alloc::rc::Rc;
use alloc::string::ToString;
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::format;
use core::ops::Deref;
use chrono::{NaiveDateTime, TimeZone};

use serde_derive::{Deserialize, Serialize};
use alloc::collections::btree_map::BTreeMap;
use chrono::{Duration, NaiveDate};
use crate::compiler::DataItem;
use crate::config::SmartCalcConfig;
use crate::token::ui_token::{UiTokenType};

use crate::tokinizer::{TokenInfo, TokenInfoStatus, Tokinizer};

pub type ExpressionFunc     = fn(config: &SmartCalcConfig, tokinizer: &Tokinizer, fields: &BTreeMap<String, Arc<TokenInfo>>) -> core::result::Result<TokenType, String>;
pub type AstResult          = Result<SmartCalcAstType, (&'static str, u16, u16)>;

pub struct Money(pub f64, pub Arc<CurrencyInfo>);
impl Money {
    pub fn get_price(&self) -> f64 {
        self.0
    }
    
    pub fn get_currency(&self) -> Arc<CurrencyInfo> {
        self.1.clone()
    }
}

#[derive(Debug)]
pub struct VariableInfo {
    pub index: usize,
    pub name: String,
    pub tokens: Vec<Rc<TokenType>>,
    pub data: RefCell<Rc<SmartCalcAstType>>
}

impl PartialEq for VariableInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.index == other.index
    }
}

unsafe impl Send for VariableInfo {}
unsafe impl Sync for VariableInfo {}


impl ToString for VariableInfo {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
pub enum FieldType {
    Text(String),
    DateTime(String),
    Date(String),
    Time(String),
    Money(String),
    Percent(String),
    Number(String),
    Group(String, Vec<String>),
    TypeGroup(Vec<String>, String),
    Month(String),
    Duration(String),
    Memory(String),
    Timezone(String)
}

unsafe impl Send for FieldType {}
unsafe impl Sync for FieldType {}

impl FieldType {
    pub fn type_name(&self) -> String {
        match self {
            FieldType::Text(_) => "TEXT".to_string(),
            FieldType::Date(_) => "DATE".to_string(),
            FieldType::DateTime(_) => "DATE_TIME".to_string(),
            FieldType::Time(_) => "TIME".to_string(),
            FieldType::Money(_) => "MONEY".to_string(),
            FieldType::Percent(_) => "PERCENT".to_string(),
            FieldType::Number(_) => "NUMBER".to_string(),
            FieldType::Group(_, _) => "GROUP".to_string(),
            FieldType::TypeGroup(_, _) => "TYPE_GROUP".to_string(),
            FieldType::Month(_) => "MONTH".to_string(),
            FieldType::Duration(_) => "DURATION".to_string(),
            FieldType::Memory(_) => "MEMORY".to_string(),
            FieldType::Timezone(_) => "TIMEZONE".to_string()
        }
    }
}


#[derive(Clone)]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct CurrencyInfo {
    pub code: String,
    pub symbol: String,

    #[serde(alias = "thousandsSeparator")]
    pub thousands_separator: String,

    #[serde(alias = "decimalSeparator")]
    pub decimal_separator: String,

    #[serde(alias = "symbolOnLeft")]
    pub symbol_on_left: bool,

    #[serde(alias = "spaceBetweenAmountAndSymbol")]
    pub space_between_amount_and_symbol: bool,

    #[serde(alias = "decimalDigits")]
    pub decimal_digits: u8
}


use core::cmp::{
    PartialEq,
    Eq,
    Ord,
    Ordering,
};

impl PartialEq for CurrencyInfo {
    fn eq(&self, other: &Self) -> bool {
        return self.code.eq(&other.code);
    }
}
impl PartialOrd for CurrencyInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.code.partial_cmp(&other.code);
    }
}
impl Eq for CurrencyInfo {}
impl Ord for CurrencyInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.code.cmp(&other.code);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimeOffset {
    pub name: String,
    pub offset: i32
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NumberType {
    Decimal,
    Octal,
    Hexadecimal,
    Binary,
    RAW
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Number(f64, NumberType),
    Text(String),
    Time(NaiveDateTime, TimeOffset),
    Date(NaiveDate, TimeOffset),
    DateTime(NaiveDateTime, TimeOffset),
    Operator(char),
    Field(Rc<FieldType>),
    Percent(f64),
    Money(f64, Arc<CurrencyInfo>),
    Variable(Rc<VariableInfo>),
    Month(u32),
    Duration(Duration),
    Memory(f64, MemoryType),
    Timezone(String, i32)
}


impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        match (&self, &other) {
            (TokenType::Timezone(l_value, l_type),     TokenType::Timezone(r_value, r_type)) => *l_value == *r_value && *l_type == *r_type,
            (TokenType::Memory(l_value, l_type),     TokenType::Memory(r_value, r_type)) => *l_value == *r_value && *l_type == *r_type,
            (TokenType::Text(l_value),     TokenType::Text(r_value)) => *l_value == *r_value,
            (TokenType::Number(l_value, _),   TokenType::Number(r_value, _)) => l_value == r_value,
            (TokenType::Percent(l_value),  TokenType::Percent(r_value)) => l_value == r_value,
            (TokenType::Operator(l_value), TokenType::Operator(r_value)) => l_value == r_value,
            (TokenType::Variable(l_value), TokenType::Variable(r_value)) => l_value == r_value,
            (TokenType::Money(l_value, l_symbol), TokenType::Money(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
            (TokenType::Time(l_value, l_tz),     TokenType::Time(r_value, r_tz)) => l_value == r_value && l_tz == r_tz,
            (TokenType::Month(l_value),     TokenType::Month(r_value)) => l_value == r_value,
            (TokenType::Duration(l_value),     TokenType::Duration(r_value)) => l_value == r_value,
            (TokenType::Date(l_value, l_tz),     TokenType::Date(r_value, r_tz)) => l_value == r_value && l_tz == r_tz,
            (TokenType::Field(l_value),    TokenType::Field(r_value)) => {
                match (l_value.deref(), r_value.deref()) {
                    (FieldType::Timezone(l), FieldType::Timezone(r)) => r == l,
                    (FieldType::Memory(l), FieldType::Memory(r)) => r == l,
                    (FieldType::Percent(l), FieldType::Percent(r)) => r == l,
                    (FieldType::Number(l),  FieldType::Number(r)) => r == l,
                    (FieldType::Text(l),    FieldType::Text(r)) => r == l,
                    (FieldType::Date(l),    FieldType::Date(r)) => r == l,
                    (FieldType::DateTime(l),    FieldType::DateTime(r)) => r == l,
                    (FieldType::Time(l),    FieldType::Time(r)) => r == l,
                    (FieldType::Money(l),   FieldType::Money(r)) => r == l,
                    (FieldType::Month(l),   FieldType::Month(r)) => r == l,
                    (FieldType::Duration(l),   FieldType::Duration(r)) => r == l,
                    (FieldType::Group(_, l),   FieldType::Group(_, r)) => r == l,
                    (FieldType::TypeGroup(l1, l2),   FieldType::TypeGroup(r1, r2)) => r1 == l1 && r2 == l2,
                    (_, _) => false,
                }
            },
            (_, _)  => false
        }
    }
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match &self {
            TokenType::Number(number, _) => number.to_string(),
            TokenType::Text(text) => text.to_string(),
            TokenType::Time(time, tz) => {
                let tz_offset = chrono::FixedOffset::east(tz.offset * 60);
                let datetime = tz_offset.from_utc_datetime(&time);
                alloc::format!("{} {}", datetime.format("%H:%M:%S").to_string(), tz.name)
            },
            TokenType::Date(date, tz) => {
                let tz_offset = chrono::FixedOffset::east(tz.offset * 60);
                let datetime = tz_offset.from_utc_date(&date);
                alloc::format!("{} {}", datetime.format("%d/%m/%Y").to_string(), tz.name)
            },
            TokenType::DateTime(datetime, tz) => {
                let tz_offset = chrono::FixedOffset::east(tz.offset * 60);
                let datetime = tz_offset.from_utc_datetime(&datetime);
                alloc::format!("{} {}", datetime.format("%d/%m/%Y %H:%M:%S").to_string(), tz.name)
            },
            TokenType::Operator(ch) => ch.to_string(),
            TokenType::Field(_) => "field".to_string(),
            TokenType::Percent(number) => format!("%{}", number),
            TokenType::Money(price, currency) => format!("{} {}", price, currency.code.to_string()),
            TokenType::Variable(var) => var.to_string(),
            TokenType::Month(month) => month.to_string(),
            TokenType::Duration(duration) => duration.to_string(),
            TokenType::Memory(memory, memory_type) => format!("{} {:?}", memory.to_string(), memory_type),
            TokenType::Timezone(timezone, offset) => format!("{} {:?}", timezone, offset)
        }
    }
}


impl TokenType {
    pub fn type_name(&self) -> String {
        match self {
            TokenType::Number(_, _) => "NUMBER".to_string(),
            TokenType::Text(_) => "TEXT".to_string(),
            TokenType::Time(_, _) => "TIME".to_string(),
            TokenType::Date(_, _) => "DATE".to_string(),
            TokenType::DateTime(_, _) => "DATE_TIME".to_string(),
            TokenType::Operator(_) => "OPERATOR".to_string(),
            TokenType::Field(_) => "FIELD".to_string(),
            TokenType::Percent(_) => "PERCENT".to_string(),
            TokenType::Money(_, _) => "MONEY".to_string(),
            TokenType::Variable(_) => "VARIABLE".to_string(),
            TokenType::Month(_) => "MONTH".to_string(),
            TokenType::Duration(_) => "DURATION".to_string(),
            TokenType::Memory(_, _) => "MEMORY".to_string(),
            TokenType::Timezone(_, _) => "TIMEZONE".to_string()
        }
    }

    pub fn variable_compare(left: &TokenInfo, right: Rc<SmartCalcAstType>) -> bool {
        match &left.token_type.borrow().deref() {
            Some(token) => match (&token, right.deref()) {
                (TokenType::Text(l_value), SmartCalcAstType::Symbol(r_value)) => l_value.deref() == r_value,
                (TokenType::Memory(l_value, l_type), SmartCalcAstType::Item(r_value)) => r_value.is_same(&(l_value.clone(), l_type.clone())),
                (TokenType::Timezone(l_value, l_type), SmartCalcAstType::Item(r_value)) => r_value.is_same(&(l_value.clone(), l_type.clone())),
                (TokenType::Number(l_value, _), SmartCalcAstType::Item(r_value)) => r_value.is_same(l_value),
                (TokenType::Percent(l_value), SmartCalcAstType::Item(r_value)) => r_value.is_same(l_value),
                (TokenType::Duration(l_value), SmartCalcAstType::Item(r_value)) => r_value.is_same(l_value),
                (TokenType::Time(l_value, l_tz), SmartCalcAstType::Item(r_value)) => r_value.is_same(&(l_value.clone(), l_tz.clone())),
                (TokenType::Money(l_value, l_symbol), SmartCalcAstType::Item(r_value)) => r_value.is_same(&(l_value.clone(), l_symbol.clone())),
                (TokenType::Date(l_value, l_tz), SmartCalcAstType::Item(r_value)) => r_value.is_same(&(l_value.clone(), l_tz.clone())),
                (TokenType::Field(l_value), _) => {
                    match (l_value.deref(), right.deref()) {
                        (FieldType::Percent(_), SmartCalcAstType::Item(item)) => item.type_name() == "PERCENT",
                        (FieldType::Number(_), SmartCalcAstType::Item(item)) => item.type_name() == "NUMBER",
                        (FieldType::Text(_), SmartCalcAstType::Symbol(_)) => true,
                        (FieldType::Time(_), SmartCalcAstType::Item(item)) => item.type_name() == "TIME",
                        (FieldType::Money(_),   SmartCalcAstType::Item(item)) => item.type_name() == "MONEY",
                        (FieldType::Month(_),   SmartCalcAstType::Month(_)) => true,
                        (FieldType::Duration(_),   SmartCalcAstType::Item(item)) => item.type_name() == "DURATION",
                        (FieldType::Memory(_),   SmartCalcAstType::Item(item)) => item.type_name() == "MEMORY",
                        (FieldType::Timezone(_),   SmartCalcAstType::Item(item)) => item.type_name() == "TIMEZONE",
                        (FieldType::DateTime(_),   SmartCalcAstType::Item(item)) => item.type_name() == "DATE_TIME",
                        (FieldType::Date(_),   SmartCalcAstType::Item(item)) => item.type_name() == "DATE",
                        (FieldType::TypeGroup(types, _), right_ast) => types.contains(&right_ast.type_name()),
                        (_, _) => false,
                    }
                },
                (_, _) => false
            },
            _ => false
        }
    }

    pub fn get_field_name(token: &TokenInfo) -> Option<String> {
        match &token.token_type.borrow().deref() {
            Some(TokenType::Field(field)) =>  match field.deref() {
                FieldType::Text(field_name)    => Some(field_name.to_string()),
                FieldType::DateTime(field_name)    => Some(field_name.to_string()),
                FieldType::Date(field_name)    => Some(field_name.to_string()),
                FieldType::Time(field_name)    => Some(field_name.to_string()),
                FieldType::Money(field_name)   => Some(field_name.to_string()),
                FieldType::Percent(field_name) => Some(field_name.to_string()),
                FieldType::Number(field_name)  => Some(field_name.to_string()),
                FieldType::Month(field_name)  => Some(field_name.to_string()),
                FieldType::Duration(field_name)  => Some(field_name.to_string()),
                FieldType::Group(field_name, _)  => Some(field_name.to_string()),
                FieldType::TypeGroup(_, field_name) => Some(field_name.to_string()),
                FieldType::Memory(field_name) => Some(field_name.to_string()),
                FieldType::Timezone(field_name) => Some(field_name.to_string())
            },
            _ => None
        }
    }

    pub fn is_same(tokens: &[Rc<TokenType>], rule_tokens: &[Rc<TokenType>]) -> Option<usize> {
        let total_rule_token       = rule_tokens.len();
        let mut rule_token_index   = 0;
        let mut target_token_index = 0;
        let mut start_token_index  = 0;

        while let Some(token) = tokens.get(target_token_index) {
            if token == &rule_tokens[rule_token_index] {
                rule_token_index   += 1;
                target_token_index += 1;
            }
            else {
                rule_token_index    = 0;
                target_token_index += 1;
                start_token_index   = target_token_index;
            }

            if total_rule_token == rule_token_index { break; }
        }

        if total_rule_token == rule_token_index {
            return Some(start_token_index);
        }
        None
    }

    pub fn is_same_location(tokens: &[Arc<TokenInfo>], rule_tokens: &[Rc<TokenType>]) -> Option<usize> {
        let total_rule_token       = rule_tokens.len();
        let mut rule_token_index   = 0;
        let mut target_token_index = 0;
        let mut start_token_index  = 0;

        while let Some(token) = tokens.get(target_token_index) {
            if token.deref() == rule_tokens[rule_token_index].deref() {
                rule_token_index   += 1;
                target_token_index += 1;
            }
            else {
                rule_token_index    = 0;
                target_token_index += 1;
                start_token_index   = target_token_index;
            }

            if total_rule_token == rule_token_index { break; }
        }

        if total_rule_token == rule_token_index {
            return Some(start_token_index);
        }
        None
    }

    pub fn update_for_variable(tokenizer: &mut Tokinizer) {
        let mut session_mut = tokenizer.session.borrow_mut();
        let mut token_start_index = 0;
        for (index, token) in session_mut.token_infos.iter().enumerate() {
            if let Some(TokenType::Operator('=')) = &token.token_type.borrow().deref() {
                token_start_index = index as usize + 1;

                tokenizer.ui_tokens.update_tokens(0, session_mut.token_infos[index - 1].end, UiTokenType::VariableDefination);                        
                break;
            }
        }

       let mut update_tokens = true;

        while update_tokens {
            let mut found            = false;
            let mut closest_variable = usize::max_value();
            let mut variable_index   = 0;
            let mut variable_size    = 0;

            update_tokens            = false;

            for (index, variable) in session_mut.variables.iter().enumerate() {
                if let Some(start_index) = TokenType::is_same_location(&session_mut.token_infos[token_start_index..].to_vec(), &variable.tokens) {
                    if (start_index == closest_variable && variable_size < variable.tokens.len()) || (start_index < closest_variable) {
                        closest_variable = start_index;
                        variable_index   = index;
                        variable_size    = variable.tokens.len();
                        found = true;
                    }
                }
            }

            if found {
                let remove_start_index  = token_start_index + closest_variable;
                let remove_end_index    = remove_start_index + variable_size;
                let text_start_position = session_mut.token_infos[remove_start_index].start;
                let text_end_position   = session_mut.token_infos[remove_end_index - 1].end;

                tokenizer.ui_tokens.update_tokens(text_start_position, text_end_position, UiTokenType::VariableUse);

                let buffer_length: usize = session_mut.token_infos[remove_start_index..remove_end_index].iter().map(|s| s.original_text.len()).sum();
                let mut original_text = String::with_capacity(buffer_length);

                for token in session_mut.token_infos[remove_start_index..remove_end_index].iter() {
                    original_text.push_str(&token.original_text.to_owned());
                }

                session_mut.token_infos.drain(remove_start_index..remove_end_index);
                
                let token_type = RefCell::new(Some(TokenType::Variable(session_mut.variables[variable_index].clone())));
                
                session_mut.token_infos.insert(remove_start_index, Arc::new(TokenInfo {
                    start: text_start_position as usize,
                    end: text_end_position as usize,
                    token_type: token_type,
                    original_text: original_text.to_owned(),
                    status: TokenInfoStatus::Active
                }));
                update_tokens = true;
            }
        }
    }
}

impl core::cmp::PartialEq<TokenType> for TokenInfo {
    fn eq(&self, other: &TokenType) -> bool {
        if self.token_type.borrow().deref().is_none() {
            return false
        }

        match &self.token_type.borrow().deref() {
            Some(l_token) => match (&l_token, &other) {
                (TokenType::Text(l_value), TokenType::Text(r_value)) => l_value == r_value,
                (TokenType::Number(l_value, _),   TokenType::Number(r_value, _)) => l_value == r_value,
                (TokenType::Percent(l_value),  TokenType::Percent(r_value)) => l_value == r_value,
                (TokenType::Operator(l_value), TokenType::Operator(r_value)) => l_value == r_value,
                (TokenType::Date(l_value, l_tz), TokenType::Date(r_value, r_tz)) => l_value == r_value && l_tz == r_tz,
                (TokenType::Duration(l_value), TokenType::Duration(r_value)) => l_value == r_value,
                (TokenType::Month(l_value), TokenType::Month(r_value)) => l_value == r_value,
                (TokenType::Money(l_value, l_symbol), TokenType::Money(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
                (TokenType::Memory(l_value, l_symbol), TokenType::Memory(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
                (TokenType::Timezone(l_value, l_symbol), TokenType::Timezone(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
                (TokenType::Variable(l_value), TokenType::Variable(r_value)) => l_value == r_value,
                (TokenType::Field(l_value), _) => {
                    match (l_value.deref(), &other) {
                        (FieldType::Percent(_), TokenType::Percent(_)) => true,
                        (FieldType::Memory(_),  TokenType::Memory(_, _)) => true,
                        (FieldType::Timezone(_),  TokenType::Timezone(_, _)) => true,
                        (FieldType::Number(_),  TokenType::Number(_, _)) => true,
                        (FieldType::Text(_),    TokenType::Text(_) ) => true,
                        (FieldType::Time(_),    TokenType::Time(_, _)) => true,
                        (FieldType::DateTime(_),    TokenType::DateTime(_, _)) => true,
                        (FieldType::Date(_),    TokenType::Date(_, _)) => true,
                        (FieldType::Money(_),   TokenType::Money(_, _)) => true,
                        (FieldType::Month(_),   TokenType::Month(_)) => true,
                        (FieldType::Duration(_),   TokenType::Duration(_)) => true,
                        (FieldType::Group(_, items),   TokenType::Text(text)) => items.iter().any(|item| item == text),
                        (FieldType::TypeGroup(types, _), right_ast) => types.contains(&right_ast.type_name()),
                        (_, _) => false,
                    }
                },
                (_, TokenType::Field(r_value)) => {
                    match (r_value.deref(), &l_token) {
                        (FieldType::Percent(_), TokenType::Percent(_)) => true,
                        (FieldType::Memory(_), TokenType::Memory(_, _)) => true,
                        (FieldType::Timezone(_), TokenType::Timezone(_, _)) => true,
                        (FieldType::Number(_),  TokenType::Number(_, _)) => true,
                        (FieldType::Text(_),    TokenType::Text(_) ) => true,
                        (FieldType::Time(_),    TokenType::Time(_, _)) => true,
                        (FieldType::DateTime(_),    TokenType::DateTime(_, _)) => true,
                        (FieldType::Date(_),    TokenType::Date(_, _)) => true,
                        (FieldType::Money(_),   TokenType::Money(_, _)) => true,
                        (FieldType::Duration(_),   TokenType::Duration(_)) => true,
                        (FieldType::Month(_),   TokenType::Month(_)) => true,
                        (FieldType::Group(_, items),    TokenType::Text(text)) => items.iter().any(|item| item == text),
                        (FieldType::TypeGroup(types, _), right_ast) => types.contains(&right_ast.type_name()),
                        (_, _) => false
                    }
                },
                (_, _)  => false
            },
            _ => false
        }
    }
}

impl PartialEq for TokenInfo {
    fn eq(&self, other: &Self) -> bool {
        if self.token_type.borrow().deref().is_none() || other.token_type.borrow().deref().is_none() {
            return false
        }

        match (&self.token_type.borrow().deref(), &other.token_type.borrow().deref()) {
            (Some(l_token), Some(r_token)) => match (&l_token, &r_token) {
                (TokenType::Text(l_value), TokenType::Text(r_value)) => l_value == r_value,
                (TokenType::Number(l_value, _),   TokenType::Number(r_value, _)) => l_value == r_value,
                (TokenType::Percent(l_value),  TokenType::Percent(r_value)) => l_value == r_value,
                (TokenType::Operator(l_value), TokenType::Operator(r_value)) => l_value == r_value,
                (TokenType::Date(l_value, l_tz), TokenType::Date(r_value, r_tz)) => l_value == r_value && l_tz == r_tz,
                (TokenType::Duration(l_value), TokenType::Duration(r_value)) => l_value == r_value,
                (TokenType::Money(l_value, l_symbol), TokenType::Money(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
                (TokenType::Memory(l_value, l_symbol), TokenType::Memory(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
                (TokenType::Timezone(l_value, l_symbol), TokenType::Timezone(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
                (TokenType::Variable(l_value), TokenType::Variable(r_value)) => l_value == r_value,
                (TokenType::Field(l_value), _) => {
                    match (l_value.deref(), &r_token) {
                        (FieldType::Percent(_), TokenType::Percent(_)) => true,
                        (FieldType::Memory(_), TokenType::Memory(_, _)) => true,
                        (FieldType::Timezone(_), TokenType::Timezone(_, _)) => true,
                        (FieldType::Number(_),  TokenType::Number(_, _)) => true,
                        (FieldType::Text(_),    TokenType::Text(_) ) => true,
                        (FieldType::Time(_),    TokenType::Time(_, _)) => true,
                        (FieldType::Date(_),    TokenType::Date(_, _)) => true,
                        (FieldType::DateTime(_),    TokenType::DateTime(_, _)) => true,
                        (FieldType::Money(_),   TokenType::Money(_, _)) => true,
                        (FieldType::Duration(_), TokenType::Duration(_)) => true,
                        (FieldType::Month(_),   TokenType::Month(_)) => true,
                        (FieldType::Group(_, items),    TokenType::Text(text)) => items.iter().any(|item| item == text),
                        (FieldType::TypeGroup(types, _), right_ast) => types.contains(&right_ast.type_name()),
                        (_, _) => false,
                    }
                },
                (_, TokenType::Field(r_value)) => {
                    match (r_value.deref(), &l_token) {
                        (FieldType::Percent(_), TokenType::Percent(_)) => true,
                        (FieldType::Memory(_), TokenType::Memory(_, _)) => true,
                        (FieldType::Timezone(_), TokenType::Timezone(_, _)) => true,
                        (FieldType::Number(_),  TokenType::Number(_, _)) => true,
                        (FieldType::Text(_),    TokenType::Text(_) ) => true,
                        (FieldType::Time(_),    TokenType::Time(_, _)) => true,
                        (FieldType::Date(_),    TokenType::Date(_, _)) => true,
                        (FieldType::DateTime(_),    TokenType::DateTime(_, _)) => true,
                        (FieldType::Money(_),   TokenType::Money(_, _)) => true,
                        (FieldType::Month(_),   TokenType::Month(_)) => true,
                        (FieldType::Duration(_), TokenType::Duration(_)) => true,
                        (FieldType::Group(_, items),    TokenType::Text(text)) => items.iter().any(|item| item == text),
                        (FieldType::TypeGroup(types, _), right_ast) => types.contains(&right_ast.type_name()),
                        (_, _) => false
                    }
                },
                (_, _)  => false
            },
            (_, _) => false
        }
    }
}

pub struct TokinizerBackup {
    pub index: u16,
    pub indexer: usize,
    pub column: u16
}

pub trait CharTraits {
    fn is_new_line(&self) -> bool;
    fn is_whitespace(&self) -> bool;
}

impl CharTraits for char {
    fn is_new_line(&self) -> bool {
        *self == '\n'
    }

    fn is_whitespace(&self) -> bool {
        matches!(*self, ' ' | '\r' | '\t')
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum MemoryType {
    Byte      = 1,
    KiloByte  = 2,
    MegaByte  = 3,
    GigaByte  = 4 ,
    TeraByte  = 5,
    PetaByte  = 6,
    ExaByte   = 7,
    ZettaByte = 8,
    YottaByte = 9
}

impl MemoryType {
    pub fn from(text: &str) -> Option<MemoryType> {
        match text {
            "b" | "byte"         => Some(MemoryType::Byte),
            "k" | "kb" | "kilo"  => Some(MemoryType::KiloByte),
            "m" | "mb" | "mega"  => Some(MemoryType::MegaByte),
            "g" | "gb" | "giga"  => Some(MemoryType::GigaByte),
            "t" | "tb" | "tera"  => Some(MemoryType::TeraByte),
            "p" | "pb" | "peta"  => Some(MemoryType::PetaByte),
            "e" | "eb" | "exa"   => Some(MemoryType::ExaByte),
            "z" | "zb" | "zetta" => Some(MemoryType::ZettaByte),
            "y" | "yb" | "yotta" => Some(MemoryType::YottaByte),
            _ =>             None
        }
    }
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
pub enum SmartCalcAstType {
    None,
    Field(Rc<FieldType>),
    Item(Arc<dyn DataItem>),
    Month(u32),
    Binary {
        left: Rc<SmartCalcAstType>,
        operator: char,
        right: Rc<SmartCalcAstType>
    },
    PrefixUnary(char, Rc<SmartCalcAstType>),
    Assignment {
        index: usize,
        expression: Rc<SmartCalcAstType>
    },
    Symbol(String),
    Variable(Rc<VariableInfo>)
}

impl SmartCalcAstType {
    pub fn type_name(&self) -> String {
        match self {
            SmartCalcAstType::None => "NONE".to_string(),
            SmartCalcAstType::Item(item) => item.type_name().to_string(),
            SmartCalcAstType::Field(field) => field.type_name(),
            SmartCalcAstType::Month(_) => "MONTH".to_string(),
            SmartCalcAstType::Binary {
                left: _,
                operator: _,
                right: _
            } => "BINARY".to_string(),
            SmartCalcAstType::PrefixUnary(_, ast) => ast.type_name(),
            SmartCalcAstType::Assignment {
                index: _,
                expression: _
            } => "ASSIGNMENT".to_string(),
            SmartCalcAstType::Symbol(_) => "SYMBOL".to_string(),
            SmartCalcAstType::Variable(variable) => variable.data.borrow().type_name()
        }
    }
}
