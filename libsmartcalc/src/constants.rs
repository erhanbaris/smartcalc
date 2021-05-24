use lazy_static::*;
use mut_static::MutStatic;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::btree_map::BTreeMap;
use regex::Regex;
use serde_json::from_str;
use crate::types::Money;
use crate::worker::rule::RuleLanguage;

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

pub static mut SYSTEM_INITED: bool = false;
lazy_static! {
    pub static ref FORMATS: MutStatic<BTreeMap<String, JsonFormat>> = {
      let m = BTreeMap::new();
      MutStatic::from(m)
    };

    pub static ref CURRENCIES: MutStatic<BTreeMap<String, Money>> = {
      let m = BTreeMap::new();
      MutStatic::from(m)
    };

    pub static ref CURRENCY_ALIAS: MutStatic<BTreeMap<String, String>> = {
      MutStatic::new()
    };
    
    pub static ref CURRENCY_RATES: MutStatic<BTreeMap<String, f64>> = {
      MutStatic::new()
    };

    pub static ref TOKEN_PARSE_REGEXES: MutStatic<BTreeMap<String, Vec<Regex>>> = {
      let m = BTreeMap::new();
      MutStatic::from(m)
    };

    pub static ref WORD_GROUPS: MutStatic<BTreeMap<String, BTreeMap<String, Vec<String>>>> = {
      let m = BTreeMap::new();
      MutStatic::from(m)
    };

    pub static ref CONSTANT_PAIRS: MutStatic<BTreeMap<String, BTreeMap<String, ConstantType>>> = {
      let m = BTreeMap::new();
      MutStatic::from(m)
    };

    pub static ref ALIAS_REGEXES: MutStatic<BTreeMap<String, Vec<(Regex, String)>>> = {
      let m = BTreeMap::new();
      MutStatic::from(m)
    };

    pub static ref RULES: MutStatic<RuleLanguage> = {
      let m = RuleLanguage::new();
      MutStatic::from(m)
    };
    
    pub static ref MONTHS_REGEXES: MutStatic<MonthLanguage> = {
      let m = MonthLanguage::new();
      MutStatic::from(m)
    };

    pub static ref JSON_CONSTANT_DEF: MutStatic<JsonConstant> = {
      let constant: JsonConstant = match from_str(&JSON_DATA) {
        Ok(data) => data,
        Err(error) => {
            log::error!("JSON parse error: {}", error);
            JsonConstant {
                parse: BTreeMap::new(),
                currency_alias: BTreeMap::new(),
                currency_rates: BTreeMap::new(),
                currencies:  BTreeMap::new(),
                default_language: String::from("en"),
                languages: BTreeMap::new(),
                type_group: BTreeMap::new()
              }
          }
      };
      MutStatic::from(constant)
    };
}

pub const JSON_DATA: &str = include_str!("./json/config.json");
