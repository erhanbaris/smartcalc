use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::btree_map::BTreeMap;
use regex::Regex;
use crate::types::Money;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum DurationFormatType {
  Second,
  Minute,
  Hour,
  Day,
  Week,
  Month,
  Year
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct DurationFormat {
  pub count: String,
  pub format: String,
  pub duration_type: DurationFormatType
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct MonthInfo {
  pub short: String,
  pub long: String,
  pub month: u8
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct JsonFormat {
  pub duration: Vec<DurationFormat>,
  pub date: BTreeMap<String, String>,

  #[serde(skip)]
  pub language: String
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
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
    Now = 11
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
            _ => None
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsonLanguageConstant {
  pub long_months: BTreeMap<String, u8>,
  pub short_months: BTreeMap<String, u8>,
  pub word_group: BTreeMap<String, Vec<String>>,
  pub constant_pair: BTreeMap<String, u8>,
  pub rules: BTreeMap<String, Vec<String>>,
  pub alias: BTreeMap<String, String>,
  pub format: JsonFormat
}

#[derive(Serialize, Deserialize)]
pub struct JsonConstant {
  pub default_language: String,
  pub parse: BTreeMap<String, Vec<String>>,
  pub currency_alias: BTreeMap<String, String>,
  pub currency_rates: BTreeMap<String, f64>,
  pub currencies:  BTreeMap<String, Money>,
  pub languages: BTreeMap<String, JsonLanguageConstant>,
  pub type_group: BTreeMap<String, Vec<String>>
}

pub type MonthItemList     = Vec<(Regex, MonthInfo)>;
pub type MonthLanguage     = BTreeMap<String, MonthItemList>;

pub const JSON_DATA: &str = include_str!("./json/config.json");
