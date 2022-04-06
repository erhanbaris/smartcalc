/*
 * smartcalc v1.0.8
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::vec::Vec;
use core::result::Result;
use alloc::rc::Rc;
use alloc::string::ToString;
use alloc::string::String;
use alloc::format;
use core::ops::Deref;
use chrono::{NaiveDateTime, TimeZone};

use serde_derive::{Deserialize, Serialize};
use alloc::collections::btree_map::BTreeMap;
use chrono::{Duration, NaiveDate};
use crate::compiler::DataItem;
use crate::compiler::dynamic_type::DynamicTypeItem;
use crate::config::DynamicType;
use crate::config::SmartCalcConfig;

use crate::tokinizer::TokenInfoStatus;
use crate::tokinizer::{TokenInfo, Tokinizer};
use crate::variable::VariableInfo;

pub type ExpressionFunc     = fn(config: &SmartCalcConfig, tokinizer: &Tokinizer, fields: &BTreeMap<String, Rc<TokenInfo>>) -> core::result::Result<TokenType, String>;
pub type AstResult          = Result<SmartCalcAstType, (&'static str, u16, u16)>;

pub struct Money(pub f64, pub Rc<CurrencyInfo>);
impl Money {
    pub fn get_price(&self) -> f64 {
        self.0
    }
    
    pub fn get_currency(&self) -> Rc<CurrencyInfo> {
        self.1.clone()
    }
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
pub enum FieldType {
    Text(String, Option<String>),
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
    Timezone(String),
    DynamicType(String, Option<String>)
}

unsafe impl Send for FieldType {}
unsafe impl Sync for FieldType {}

impl FieldType {
    pub fn type_name(&self) -> String {
        match self {
            FieldType::Text(_, _) => "TEXT".to_string(),
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
            FieldType::Timezone(_) => "TIMEZONE".to_string(),
            FieldType::DynamicType(_, _) => "DYNAMIC_TYPE".to_string()
        }
    }
}


impl PartialEq for FieldType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FieldType::Timezone(l), FieldType::Timezone(r)) => r == l,
            (FieldType::Percent(l), FieldType::Percent(r)) => r == l,
            (FieldType::Number(l),  FieldType::Number(r)) => r == l,
            (FieldType::Text(l, _),    FieldType::Text(r, _)) => r.to_lowercase() == l.to_lowercase(),
            (FieldType::Date(l),    FieldType::Date(r)) => r == l,
            (FieldType::DateTime(l),    FieldType::DateTime(r)) => r == l,
            (FieldType::Time(l),    FieldType::Time(r)) => r == l,
            (FieldType::Money(l),   FieldType::Money(r)) => r == l,
            (FieldType::Month(l),   FieldType::Month(r)) => r == l,
            (FieldType::Duration(l),   FieldType::Duration(r)) => r == l,
            (FieldType::Group(_, l),   FieldType::Group(_, r)) => r == l,
            (FieldType::DynamicType(l, _),   FieldType::DynamicType(r, _)) => r == l,
            (FieldType::TypeGroup(l1, l2),   FieldType::TypeGroup(r1, r2)) => r1 == l1 && r2 == l2,
            (_, _) => false,
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
        self.code.eq(&other.code)
    }
}
impl PartialOrd for CurrencyInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.code.partial_cmp(&other.code)
    }
}
impl Eq for CurrencyInfo {}
impl Ord for CurrencyInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.code.cmp(&other.code)
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
    Raw
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
    DynamicType(f64, Rc<DynamicType>),
    Money(f64, Rc<CurrencyInfo>),
    Variable(Rc<VariableInfo>),
    Month(u32),
    Duration(Duration),
    Timezone(String, i32)
}


impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        match (&self, &other) {
            (TokenType::Timezone(l_value, l_type),     TokenType::Timezone(r_value, r_type)) => *l_value == *r_value && *l_type == *r_type,
            (TokenType::Text(l_value),     TokenType::Text(r_value)) => l_value.to_lowercase() == r_value.to_lowercase(),
            (TokenType::Number(l_value, _),   TokenType::Number(r_value, _)) => l_value == r_value,
            (TokenType::Percent(l_value),  TokenType::Percent(r_value)) => l_value == r_value,
            (TokenType::Operator(l_value), TokenType::Operator(r_value)) => l_value == r_value,
            (TokenType::Variable(l_value), TokenType::Variable(r_value)) => l_value == r_value,
            (TokenType::Money(l_value, l_symbol), TokenType::Money(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
            (TokenType::Time(l_value, l_tz),     TokenType::Time(r_value, r_tz)) => l_value == r_value && l_tz == r_tz,
            (TokenType::Month(l_value),     TokenType::Month(r_value)) => l_value == r_value,
            (TokenType::Duration(l_value),     TokenType::Duration(r_value)) => l_value == r_value,
            (TokenType::Date(l_value, l_tz),     TokenType::Date(r_value, r_tz)) => l_value == r_value && l_tz == r_tz,
            (TokenType::Field(l_value),    TokenType::Field(r_value)) => l_value.deref() == r_value.deref(),
            (_, _)  => false
        }
    }
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match &self {
            TokenType::DynamicType(number, dynamic_type) => dynamic_type.format.replace("{value}", &number.to_string()),
            TokenType::Number(number, _) => number.to_string(),
            TokenType::Text(text) => text.to_string(),
            TokenType::Time(time, tz) => {
                let tz_offset = chrono::FixedOffset::east(tz.offset * 60);
                let datetime = tz_offset.from_utc_datetime(time);
                alloc::format!("{} {}", datetime.format("%H:%M:%S").to_string(), tz.name)
            },
            TokenType::Date(date, tz) => {
                let tz_offset = chrono::FixedOffset::east(tz.offset * 60);
                let datetime = tz_offset.from_utc_date(date);
                alloc::format!("{} {}", datetime.format("%d/%m/%Y").to_string(), tz.name)
            },
            TokenType::DateTime(datetime, tz) => {
                let tz_offset = chrono::FixedOffset::east(tz.offset * 60);
                let datetime = tz_offset.from_utc_datetime(datetime);
                alloc::format!("{} {}", datetime.format("%d/%m/%Y %H:%M:%S").to_string(), tz.name)
            },
            TokenType::Operator(ch) => ch.to_string(),
            TokenType::Field(_) => "field".to_string(),
            TokenType::Percent(number) => format!("%{}", number),
            TokenType::Money(price, currency) => format!("{} {}", price, currency.code.to_string()),
            TokenType::Variable(var) => var.to_string(),
            TokenType::Month(month) => month.to_string(),
            TokenType::Duration(duration) => duration.to_string(),
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
            TokenType::Timezone(_, _) => "TIMEZONE".to_string(),
            TokenType::DynamicType(_, _) => "DYNAMIC_TYPE".to_string()
        }
    }

    pub fn field_compare(&self, field: &FieldType) -> bool {
        match (field, self) {
            (FieldType::DynamicType(_, expected), TokenType::DynamicType(_, dynamic_type)) => expected.as_ref().map_or(true, |v| v.to_lowercase() == dynamic_type.group_name.to_lowercase()),
            (FieldType::Percent(_), TokenType::Percent(_)) => true,
            (FieldType::Timezone(_),  TokenType::Timezone(_, _)) => true,
            (FieldType::Number(_),  TokenType::Number(_, _)) => true,
            (FieldType::Text(_, expected),    TokenType::Text(text) ) => expected.as_ref().map_or(true, |v| v.to_lowercase() == text.to_lowercase()),
            (FieldType::Time(_),    TokenType::Time(_, _)) => true,
            (FieldType::DateTime(_),    TokenType::DateTime(_, _)) => true,
            (FieldType::Date(_),    TokenType::Date(_, _)) => true,
            (FieldType::Money(_),   TokenType::Money(_, _)) => true,
            (FieldType::Month(_),   TokenType::Month(_)) => true,
            (FieldType::Duration(_),   TokenType::Duration(_)) => true,
            (FieldType::Group(_, items),   TokenType::Text(text)) => items.iter().any(|item| item.to_lowercase() == text.to_lowercase()),
            (FieldType::TypeGroup(types, _), right_ast) => types.contains(&right_ast.type_name()),
            (_, _) => false,
        }
    }

    pub fn variable_compare(left: &TokenInfo, right: Rc<SmartCalcAstType>) -> bool {
        match &left.token_type.borrow().deref() {
            Some(token) => match (&token, right.deref()) {
                (TokenType::Text(l_value), SmartCalcAstType::Symbol(r_value)) => l_value.deref().to_lowercase() == r_value.to_lowercase(),
                (TokenType::Timezone(l_value, l_type), SmartCalcAstType::Item(r_value)) => r_value.is_same(&(l_value.clone(), *l_type)),
                (TokenType::Number(l_value, _), SmartCalcAstType::Item(r_value)) => r_value.is_same(l_value),
                (TokenType::Percent(l_value), SmartCalcAstType::Item(r_value)) => r_value.is_same(l_value),
                (TokenType::Duration(l_value), SmartCalcAstType::Item(r_value)) => r_value.is_same(l_value),
                (TokenType::Time(l_value, l_tz), SmartCalcAstType::Item(r_value)) => r_value.is_same(&(*l_value, l_tz.clone())),
                (TokenType::Money(l_value, l_symbol), SmartCalcAstType::Item(r_value)) => r_value.is_same(&(*l_value, l_symbol.clone())),
                (TokenType::Date(l_value, l_tz), SmartCalcAstType::Item(r_value)) => r_value.is_same(&(*l_value, l_tz.clone())),
                (TokenType::Field(l_value), _) => right.field_compare(l_value.deref()),
                (_, _) => false
            },
            _ => false
        }
    }

    pub fn get_field_name(token: &TokenInfo) -> Option<String> {
        match &token.token_type.borrow().deref() {
            Some(TokenType::Field(field)) =>  match field.deref() {
                FieldType::Text(field_name, _)    => Some(field_name.to_string()),
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
                FieldType::Timezone(field_name) => Some(field_name.to_string()),
                FieldType::DynamicType(field_name, _) => Some(field_name.to_string())
            },
            _ => None
        }
    }
}

pub fn find_location<T: PartialEq<U>, U>(tokens: &[Rc<T>], rule_tokens: &[Rc<U>]) -> Option<usize> {
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

impl core::cmp::PartialEq<TokenType> for TokenInfo {
    fn eq(&self, other: &TokenType) -> bool {
        if self.token_type.borrow().deref().is_none() {
            return false
        }

        match &self.token_type.borrow().deref() {
            Some(l_token) => match (&l_token, &other) {
                (TokenType::Text(l_value), TokenType::Text(r_value)) => l_value.to_lowercase() == r_value.to_lowercase(),
                (TokenType::Number(l_value, _),   TokenType::Number(r_value, _)) => l_value == r_value,
                (TokenType::Percent(l_value),  TokenType::Percent(r_value)) => l_value == r_value,
                (TokenType::Operator(l_value), TokenType::Operator(r_value)) => l_value == r_value,
                (TokenType::Date(l_value, l_tz), TokenType::Date(r_value, r_tz)) => l_value == r_value && l_tz == r_tz,
                (TokenType::Duration(l_value), TokenType::Duration(r_value)) => l_value == r_value,
                (TokenType::Month(l_value), TokenType::Month(r_value)) => l_value == r_value,
                (TokenType::Money(l_value, l_symbol), TokenType::Money(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
                (TokenType::Timezone(l_value, l_symbol), TokenType::Timezone(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
                (TokenType::Variable(l_value), TokenType::Variable(r_value)) => l_value == r_value,
                (TokenType::Field(l_value), _) => other.field_compare(l_value.deref()),
                (_, TokenType::Field(r_value)) => l_token.field_compare(r_value.deref()),
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

        if self.status.get() == TokenInfoStatus::Removed || other.status.get() == TokenInfoStatus::Removed {
            return false;
        }

        match (&self.token_type.borrow().deref(), &other.token_type.borrow().deref()) {
            (Some(l_token), Some(r_token)) => match (&l_token, &r_token) {
                (TokenType::Text(l_value), TokenType::Text(r_value)) => l_value.to_lowercase() == r_value.to_lowercase(),
                (TokenType::Number(l_value, _),   TokenType::Number(r_value, _)) => l_value == r_value,
                (TokenType::Percent(l_value),  TokenType::Percent(r_value)) => l_value == r_value,
                (TokenType::Operator(l_value), TokenType::Operator(r_value)) => l_value == r_value,
                (TokenType::Date(l_value, l_tz), TokenType::Date(r_value, r_tz)) => l_value == r_value && l_tz == r_tz,
                (TokenType::Duration(l_value), TokenType::Duration(r_value)) => l_value == r_value,
                (TokenType::Money(l_value, l_symbol), TokenType::Money(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
                (TokenType::Timezone(l_value, l_symbol), TokenType::Timezone(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
                (TokenType::Variable(l_value), TokenType::Variable(r_value)) => l_value == r_value,
                (TokenType::Field(l_value), _) => r_token.field_compare(l_value.deref()),
                (_, TokenType::Field(r_value)) => l_token.field_compare(r_value.deref()),
                (_, _)  => false
            },
            (_, _) => false
        }
    }
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

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
pub enum SmartCalcAstType {
    None,
    Field(Rc<FieldType>),
    Item(Rc<dyn DataItem>),
    Month(u32),
    Binary {
        left: Rc<SmartCalcAstType>,
        operator: char,
        right: Rc<SmartCalcAstType>
    },
    PrefixUnary(char, Rc<SmartCalcAstType>),
    Assignment {
        variable: Rc<VariableInfo>,
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
                variable: _,
                expression: _
            } => "ASSIGNMENT".to_string(),
            SmartCalcAstType::Symbol(_) => "SYMBOL".to_string(),
            SmartCalcAstType::Variable(variable) => variable.data.borrow().type_name()
        }
    }

    pub fn field_compare(&self, field: &FieldType) -> bool {
        match (field, self) {
            (FieldType::DynamicType(_, expected), SmartCalcAstType::Item(item)) => item.type_name() == "DYNAMIC_TYPE" && expected.as_ref().map_or(true, |v| v.to_lowercase() == match item.as_any().downcast_ref::<DynamicTypeItem>() {
                Some(item) => item.get_type().group_name.to_lowercase(),
                None => return false
            }),
            (FieldType::Percent(_), SmartCalcAstType::Item(item)) => item.type_name() == "PERCENT",
            (FieldType::Number(_), SmartCalcAstType::Item(item)) => item.type_name() == "NUMBER",
            (FieldType::Text(_, expected), SmartCalcAstType::Symbol(symbol)) => expected.as_ref().map_or(true, |v| v.to_lowercase() == symbol.to_lowercase()),
            (FieldType::Time(_), SmartCalcAstType::Item(item)) => item.type_name() == "TIME",
            (FieldType::Money(_),   SmartCalcAstType::Item(item)) => item.type_name() == "MONEY",
            (FieldType::Month(_),   SmartCalcAstType::Month(_)) => true,
            (FieldType::Duration(_),   SmartCalcAstType::Item(item)) => item.type_name() == "DURATION",
            (FieldType::Timezone(_),   SmartCalcAstType::Item(item)) => item.type_name() == "TIMEZONE",
            (FieldType::DateTime(_),   SmartCalcAstType::Item(item)) => item.type_name() == "DATE_TIME",
            (FieldType::Date(_),   SmartCalcAstType::Item(item)) => item.type_name() == "DATE",
            (FieldType::TypeGroup(types, _), right_ast) => types.contains(&right_ast.type_name()),
            (_, _) => false,
        }
    }
}
