/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::borrow::Borrow;

use alloc::format;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::collections::btree_map::BTreeMap;
use regex::Regex;
use serde_json::from_str;
use crate::session::Session;
use crate::tokinizer::RuleItemList;
use crate::types::CurrencyInfo;
use crate::types::TimeOffset;
use crate::tokinizer::Tokinizer;
use crate::tokinizer::TokenInfo;
use crate::tokinizer::RULE_FUNCTIONS;
use crate::constants::*;

pub type LanguageData<T> = BTreeMap<String, T>;
pub type CurrencyData<T> = BTreeMap<Rc<CurrencyInfo>, T>;

#[derive(Default)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct DynamicType {
    pub group_name: String,
    pub index: usize,
    pub format: String,
    pub parse: Vec<Vec<Rc<TokenInfo>>>,
    pub multiplier: f64,
    pub names:Vec<String>,
    pub decimal_digits: Option<u8>,
    pub use_fract_rounding: Option<bool>,
    pub remove_fract_if_zero: Option<bool>
}

impl DynamicType {
    pub fn new(group_name: String, index: usize, format: String, parse: Vec<Vec<Rc<TokenInfo>>>, multiplier: f64, names:Vec<String>, decimal_digits: Option<u8>, use_fract_rounding: Option<bool>, remove_fract_if_zero: Option<bool>) -> Self {
        DynamicType {
            group_name,
            index,
            format,
            parse,
            multiplier,
            names,
            decimal_digits,
            use_fract_rounding,
            remove_fract_if_zero
        }
    }
}

pub struct SmartCalcConfig {
    pub(crate) json_data: JsonConstant,
    pub(crate) format: LanguageData<JsonFormat>,
    pub(crate) currency: LanguageData<Rc<CurrencyInfo>>,
    pub(crate) currency_alias: LanguageData<Rc<CurrencyInfo>>,
    pub(crate) timezones: BTreeMap<String, i32>,
    pub(crate) currency_rate: CurrencyData<f64>,
    pub(crate) token_parse_regex: LanguageData<Vec<Regex>>,
    pub(crate) word_group: LanguageData<BTreeMap<String, Vec<String>>>,
    pub(crate) constant_pair: LanguageData<BTreeMap<String, ConstantType>>,
    pub(crate) language_alias_regex: LanguageData<Vec<(Regex, String)>>,
    pub(crate) alias_regex: Vec<(Regex, String)>,
    pub(crate) rule: LanguageData<RuleItemList>,
    pub(crate) types: BTreeMap<String, BTreeMap<usize, Rc<DynamicType>>>,
    pub(crate) type_conversion: Vec<JsonTypeConversion>,
    pub(crate) month_regex: LanguageData<MonthItemList>,
    pub(crate) decimal_seperator: String,
    pub(crate) thousand_separator: String,
    pub(crate) timezone: String,
    pub(crate) timezone_offset: i32
}

impl Default for SmartCalcConfig {
    fn default() -> Self {
        SmartCalcConfig::load_from_json(JSON_DATA)
    }
}

impl SmartCalcConfig {
    pub fn get_time_offset(&self) -> TimeOffset {
        TimeOffset {
            name: self.timezone.to_string(),
            offset: self.timezone_offset
        }
    }

    pub fn get_currency<T: Borrow<String>>(&self, currency: T) -> Option<Rc<CurrencyInfo>> {
        self.currency
            .get(currency.borrow())
            .cloned()
    }

    pub fn load_from_json(json_data: &str) -> Self {
        let mut config = SmartCalcConfig {
            json_data: match from_str(json_data) {
                Ok(data) => data,
                Err(error) => panic!("JSON parse error: {}", error)
            },
            format: LanguageData::new(),
            currency: LanguageData::new(),
            currency_alias: LanguageData::new(),
            timezones: BTreeMap::new(),
            currency_rate: CurrencyData::new(),
            token_parse_regex: LanguageData::new(),
            word_group: LanguageData::new(),
            constant_pair: LanguageData::new(),
            language_alias_regex: LanguageData::new(),
            rule: LanguageData::new(),
            types: BTreeMap::new(),
            type_conversion: Vec::new(),
            month_regex: LanguageData::new(),
            alias_regex: Vec::new(),
            decimal_seperator: ",".to_string(),
            thousand_separator: ".".to_string(),
            timezone: "UTC".to_string(),
            timezone_offset: 0
        };
        
        for (name, currency) in config.json_data.currencies.iter() {
            config.currency.insert(name.to_lowercase(), currency.clone());
        }

        for (timezone, offset) in config.json_data.timezones.iter() {
            config.timezones.insert(timezone.clone(), *offset);
        }

        for (from, to) in config.json_data.alias.iter() {
            match Regex::new(&format!(r"\b{}\b", from)) {
                Ok(re) => config.alias_regex.push((re, to.to_string())),
                Err(error) => log::error!("Alias parser error ({}) {}", from, error)
            }
        }

        for (language, language_object) in config.json_data.languages.iter() {
            let mut language_clone = language_object.format.clone();
            language_clone.language = language.to_string();
            config.format.insert(language.to_string(), language_clone);
        }

        for (key, value) in config.json_data.currency_alias.iter() {
            match config.get_currency(value) {
                Some(currency) => { config.currency_alias.insert(key.to_string(), currency.clone()); },
                None => log::warn!("'{}' currency not found at alias", value)
            };
        }

        for (key, value) in config.json_data.currency_rates.iter() {
            match config.get_currency(key) {
                Some(currency) => { config.currency_rate.insert(currency.clone(), *value); },
                None => log::warn!("'{}' currency not found at rate", key)
            };
        }

        for (language, language_constant) in config.json_data.languages.iter() {
            let mut language_aliases = Vec::new();
            for (alias, target_name) in language_constant.alias.iter() {
                
                match Regex::new(&format!(r"\b{}\b", alias)) {
                    Ok(re) => language_aliases.push((re, target_name.to_string())),
                    Err(error) => log::error!("Alias parser error ({}) {}", alias, error)
                }

            }

            config.language_alias_regex.insert(language.to_string(), language_aliases);
        }
        
        for (parse_type, items) in &config.json_data.parse {
            let mut patterns = Vec::new();
            for pattern in items {
                match Regex::new(pattern) {
                    Ok(re) => patterns.push(re),
                    Err(error) => log::error!("Token parse regex error ({}) {}", pattern, error)
                }
            }

            config.token_parse_regex.insert(parse_type.to_string(), patterns);
        }

        for (language, language_constant) in config.json_data.languages.iter() {
            let mut language_group = Vec::new();
            let mut month_list = Vec::with_capacity(12);
            for i in 0..12 {
                month_list.push(MonthInfo {
                    short: String::new(),
                    long: String::new(),
                    month: i + 1
                });
            }

            for (month_name, month_number) in &language_constant.long_months {
                match month_list.get_mut((*month_number - 1) as usize) {
                    Some(month_object) => month_object.long = month_name.to_string(),
                    None => log::warn!("Month not fetched. {}", month_number)
                };
            }

            for (month_name, month_number) in &language_constant.short_months {
                match month_list.get_mut((*month_number - 1) as usize) {
                    Some(month_object) => month_object.short = month_name.to_string(),
                    None => log::warn!("Month not fetched. {}", month_number)
                };
            }

            for month in month_list.iter() {
                let pattern = &format!(r"\b{}\b|\b{}\b", month.long, month.short);
                match Regex::new(pattern) {
                    Ok(re) => language_group.push((re, month.clone())),
                    Err(error) => log::error!("Month parser error ({}) {}", month.long, error)
                }
            }

            config.month_regex.insert(language.to_string(), language_group);
        }

        for (language, language_constant) in config.json_data.languages.iter() {
            let mut word_groups = BTreeMap::new();
            for (word_group_name, word_group_items) in language_constant.word_group.iter() {
                let mut patterns = Vec::new();

                for pattern in word_group_items {
                    patterns.push(pattern.to_string());
                }

                word_groups.insert(word_group_name.to_string(), patterns);
            }

            config.word_group.insert(language.to_string(), word_groups);
        }

        for (language, language_constant) in config.json_data.languages.iter() {
            let mut constants = BTreeMap::new();
            for (alias_name, constant_type) in language_constant.constant_pair.iter() {

                match ConstantType::from_u8(*constant_type) {
                    Some(const_type) => {
                        constants.insert(alias_name.to_string(), const_type);
                    },
                    _ => log::error!("Constant type not parsed. {}", constant_type)
                };
            }

            config.constant_pair.insert(language.to_string(), constants);
        }
        
        for (language, language_constant) in config.json_data.languages.iter() {
            let mut language_rules = Vec::new();
            for (rule_name, rule) in language_constant.rules.iter() {
                if let Some(function_ref) = RULE_FUNCTIONS.get(rule_name) {
                    let mut function_items = Vec::new();

                    for rule_item in &rule.rules {
                        let mut session = Session::new();
                        session.set_language(language.to_string());
                        session.set_text(rule_item.to_string());
                        function_items.push(Tokinizer::token_infos(&config, &session));
                    }

                    language_rules.push((rule_name.to_string(), *function_ref, function_items));
                }
                else {
                    log::warn!("Function not found : {}", rule_name);
                }
            }

            config.rule.insert(language.to_string(), language_rules);
        }
        
        for dynamic_type in config.json_data.types.iter() {
            let mut dynamic_type_holder = BTreeMap::new();
            
            for type_item in dynamic_type.items.iter() {
                let mut token_info = DynamicType {
                    group_name: dynamic_type.name.to_string(),
                    index: type_item.index,
                    format: type_item.format.to_string(),
                    parse: Vec::new(),
                    multiplier: type_item.multiplier,
                    names: type_item.names.clone(),
                    decimal_digits: type_item.decimal_digits,
                    use_fract_rounding: type_item.use_fract_rounding,
                    remove_fract_if_zero: type_item.remove_fract_if_zero
                };

                for type_parse_item in type_item.parse.iter() {
                    let mut session = Session::new();
                    session.set_language("en".to_string());
                    session.set_text(type_parse_item.to_string());
                    
                    let tokens = Tokinizer::token_infos(&config, &session);
                    token_info.parse.push(tokens);
                }
                
                dynamic_type_holder.insert(token_info.index, Rc::new(token_info));
            }
            
            config.types.insert(dynamic_type.name.to_string(), dynamic_type_holder);
        }
        
        for type_conversion in config.json_data.type_conversion.iter() {
            let source = config.types.get(&type_conversion.source.name);
            let target = config.types.get(&type_conversion.target.name);

            let mut source_found = false;
            let mut target_found = false;

            if let Some(source) = source {
                source_found = source.contains_key(&type_conversion.source.index)

            }

            if let Some(target) = target {
                target_found = target.contains_key(&type_conversion.target.index)

            }

            if !source_found {
                log::warn!("{} type not defined", type_conversion.source.name);
            }

            if !target_found {
                log::warn!("{} type not defined", type_conversion.target.name);
            }
            
            if source_found && target_found {
                config.type_conversion.push(type_conversion.clone());
            }
        }

        config
    }
}