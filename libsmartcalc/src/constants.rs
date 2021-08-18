use crate::types::CurrencyInfo;
use alloc::sync::Arc;
use alloc::{collections::btree_map::BTreeMap};
use alloc::string::String;
use alloc::vec::Vec;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use serde_repr::*;


#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
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
pub struct DurationFormat {
    pub count: String,
    pub format: String,
    pub duration_type: DurationFormatType,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MonthInfo {
    pub short: String,
    pub long: String,
    pub month: u8,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct JsonFormat {
    pub duration: Vec<DurationFormat>,
    pub date: BTreeMap<String, String>,

    #[serde(skip)]
    pub language: String,
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq, Serialize_repr, Deserialize_repr)]
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

impl NumberNotationType {
    pub fn from_u8(number: u8) -> Option<Self> {
        match number {
            0 => Some(NumberNotationType::None),
            1 => Some(NumberNotationType::Thousand),
            2 => Some(NumberNotationType::Million),
            3 => Some(NumberNotationType::Billion),
            4 => Some(NumberNotationType::Trillion),
            5 => Some(NumberNotationType::Quadrillion),
            6 => Some(NumberNotationType::Quintillion),
            7 => Some(NumberNotationType::Sextillion),
            _ => None,
        }
    }
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
pub struct Sample {
    pub query: String,
    pub result: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LanguageRule {
    pub rules: Vec<String>,
    pub samples: Vec<Sample>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JsonLanguageConstant {
    pub number_notation: BTreeMap<String, NumberNotationType>,
    pub long_months: BTreeMap<String, u8>,
    pub short_months: BTreeMap<String, u8>,
    pub word_group: BTreeMap<String, Vec<String>>,
    pub constant_pair: BTreeMap<String, u8>,
    pub rules: BTreeMap<String, LanguageRule>,
    pub alias: BTreeMap<String, String>,
    pub format: JsonFormat,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct JsonConstant {
    pub default_language: String,
    pub parse: BTreeMap<String, Vec<String>>,
    pub currency_alias: BTreeMap<String, String>,
    pub currency_rates: BTreeMap<String, f64>,
    pub currencies: BTreeMap<String, Arc<CurrencyInfo>>,
    pub languages: BTreeMap<String, JsonLanguageConstant>,
    pub type_group: BTreeMap<String, Vec<String>>,
}

pub type MonthItemList = Vec<(Regex, MonthInfo)>;

pub const JSON_DATA: &str = include_str!("./json/config.json");
