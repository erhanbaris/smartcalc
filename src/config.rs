/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::borrow::Borrow;
use core::cell::RefCell;

use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::collections::btree_map::BTreeMap;
use regex::Regex;
use serde_json::from_str;
use crate::app::Session;
use crate::types::CurrencyInfo;
use crate::worker::rule::RuleItemList;
use crate::tokinizer::Tokinizer;
use crate::worker::rule::RULE_FUNCTIONS;
use crate::constants::*;

pub type LanguageData<T> = BTreeMap<String, T>;
pub type CurrencyData<T> = BTreeMap<Arc<CurrencyInfo>, T>;

pub struct SmartCalcConfig {
    pub json_data: JsonConstant,
    pub format: LanguageData<JsonFormat>,
    pub currency: LanguageData<Arc<CurrencyInfo>>,
    pub currency_alias: LanguageData<Arc<CurrencyInfo>>,
    pub timezones: BTreeMap<String, i32>,
    pub currency_rate: CurrencyData<f64>,
    pub token_parse_regex: LanguageData<Vec<Regex>>,
    pub word_group: LanguageData<BTreeMap<String, Vec<String>>>,
    pub constant_pair: LanguageData<BTreeMap<String, ConstantType>>,
    pub language_alias_regex: LanguageData<Vec<(Regex, String)>>,
    pub alias_regex: Vec<(Regex, String)>,
    pub rule: LanguageData<RuleItemList>,
    pub month_regex: LanguageData<MonthItemList>,
    pub numeric_notation: LanguageData<JsonFormat>,
    pub decimal_seperator: String,
    pub thousand_separator: String
}

impl Default for SmartCalcConfig {
    fn default() -> Self {
        SmartCalcConfig::load_from_json(&JSON_DATA)
    }
}

impl SmartCalcConfig {
    pub fn get_currency<T: Borrow<String>>(&self, currency: T) -> Option<Arc<CurrencyInfo>> {
        self.currency
            .get(currency.borrow())
            .map(|currency_info| currency_info.clone())
    }

    pub fn load_from_json(json_data: &str) -> Self {
        let mut config = SmartCalcConfig {
            json_data: match from_str(&json_data) {
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
            month_regex: LanguageData::new(),
            numeric_notation: LanguageData::new(),
            alias_regex: Vec::new(),
            decimal_seperator: ",".to_string(),
            thousand_separator: ".".to_string()
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
                match Regex::new(&pattern) {
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
                        
                        let ref_session = RefCell::new(session);
                        function_items.push(Tokinizer::token_infos(&config, &ref_session));
                    }

                    language_rules.push((rule_name.to_string(), *function_ref, function_items));
                }
                else {
                    log::warn!("Function not found : {}", rule_name);
                }
            }

            config.rule.insert(language.to_string(), language_rules);
        }

        config
    }
}