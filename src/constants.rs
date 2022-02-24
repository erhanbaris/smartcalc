/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use crate::types::CurrencyInfo;
use alloc::borrow::Cow;
use alloc::sync::Arc;
use alloc::{collections::btree_map::BTreeMap};
use alloc::string::String;
use alloc::vec::Vec;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use serde_repr::*;


#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum DurationFormatType {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DurationFormat<'a> {
    #[serde(borrow)]
    pub count: Cow<'a, str>,
    
    #[serde(borrow)]
    pub format: Cow<'a, str>,
    pub duration_type: DurationFormatType,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MonthInfo {
    pub short: String,
    pub long: String,
    pub month: u8,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct JsonFormat<'a> {
    pub duration: Vec<DurationFormat<'a>>,
    
    #[serde(borrow)]
    pub date: BTreeMap<Cow<'a, str>, Cow<'a, str>>,

    #[serde(skip)]
    pub language: String,
}

#[derive(Clone, Debug)]
pub enum ConstantType {
    None = 0,
    Day = 1,
    Week = 2,
    Month = 3,
    Year = 4,
    Second = 5,
    Minute = 6,
    Hour = 7,
    Today = 8,
    Tomorrow = 9,
    Yesterday = 10,
    Now = 11,
}

#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum NumberNotationType {
    None = 0,
    Thousand = 1,
    Million = 2,
    Billion = 3,
    Trillion = 4,
    Quadrillion = 5,
    Quintillion = 6,
    Sextillion = 7,
}

impl ConstantType {
    pub fn from_u8(number: u8) -> Option<Self> {
        match number {
            1 => Some(ConstantType::Day),
            2 => Some(ConstantType::Week),
            3 => Some(ConstantType::Month),
            4 => Some(ConstantType::Year),
            5 => Some(ConstantType::Second),
            6 => Some(ConstantType::Minute),
            7 => Some(ConstantType::Hour),
            8 => Some(ConstantType::Today),
            9 => Some(ConstantType::Tomorrow),
            10 => Some(ConstantType::Yesterday),
            11 => Some(ConstantType::Now),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Sample<'a> {
    #[serde(borrow)]
    pub query: Cow<'a, str>,

    #[serde(borrow)]
    pub result: Cow<'a, str>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LanguageRule<'a> {
    #[serde(borrow)]
    pub rules: Vec<Cow<'a, str>>,

    #[serde(borrow)]
    pub samples: Vec<Sample<'a>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JsonLanguageConstant<'a> {
    #[serde(borrow)]
    pub number_notation: BTreeMap<Cow<'a, str>, NumberNotationType>,

    #[serde(borrow)]
    pub long_months: BTreeMap<Cow<'a, str>, u8>,

    #[serde(borrow)]
    pub short_months: BTreeMap<Cow<'a, str>, u8>,

    #[serde(borrow)]
    pub word_group: BTreeMap<Cow<'a, str>, Vec<Cow<'a, str>>>,

    #[serde(borrow)]
    pub constant_pair: BTreeMap<Cow<'a, str>, u8>,

    #[serde(borrow)]
    pub rules: BTreeMap<Cow<'a, str>, LanguageRule<'a>>,
    
    #[serde(borrow)]
    pub alias: BTreeMap<Cow<'a, str>, Cow<'a, str>>,

    pub format: JsonFormat<'a>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct JsonConstant<'a> {
    #[serde(borrow)]
    pub default_language: Cow<'a, str>,

    #[serde(borrow)]
    pub parse: BTreeMap<Cow<'a, str>, Vec<Cow<'a, str>>>,

    #[serde(borrow)]
    pub alias: BTreeMap<Cow<'a, str>, Cow<'a, str>>,

    #[serde(borrow)]
    pub currency_alias: BTreeMap<Cow<'a, str>, Cow<'a, str>>,

    #[serde(borrow)]
    pub currency_rates: BTreeMap<Cow<'a, str>, f64>,
    
    #[serde(borrow)]
    pub currencies: BTreeMap<Cow<'a, str>, Arc<CurrencyInfo>>,
    
    #[serde(borrow)]
    pub languages: BTreeMap<Cow<'a, str>, JsonLanguageConstant<'a>>,

    #[serde(borrow)]
    pub type_group: BTreeMap<Cow<'a, str>, Vec<Cow<'a, str>>>,
}

pub type MonthItemList = Vec<(Regex, MonthInfo)>;

pub const JSON_DATA: &str = include_str!("./json/config.json");
