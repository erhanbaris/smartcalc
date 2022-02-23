/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

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

pub type LanguageData<'a, T> = BTreeMap<&'a str, T>;
pub type CurrencyData<T> = BTreeMap<Arc<CurrencyInfo>, T>;

pub struct SmartCalcConfig<'a> {
    pub json_data: JsonConstant<'a>,
    pub format: LanguageData<'a, JsonFormat<'a>>,
    pub currency: LanguageData<'a, Arc<CurrencyInfo>>,
    pub currency_alias: LanguageData<'a, Arc<CurrencyInfo>>,
    pub currency_rate: CurrencyData<f64>,
    pub token_parse_regex: LanguageData<'a, Vec<Regex>>,
    pub word_group: LanguageData<'a, BTreeMap<&'a str, Vec<&'a str>>>,
    pub constant_pair: LanguageData<'a, BTreeMap<&'a str, ConstantType>>,
    pub language_alias_regex: LanguageData<'a, Vec<(Regex, &'a str)>>,
    pub alias_regex: Vec<(Regex, &'a str)>,
    pub rule: LanguageData<'a, RuleItemList<'a>>,
    pub month_regex: LanguageData<'a, MonthItemList>,
    pub numeric_notation: LanguageData<'a, JsonFormat<'a>>,
    pub decimal_seperator: &'a str,
    pub thousand_separator: &'a str
}

impl<'a> Default for SmartCalcConfig<'a> {
    fn default() -> Self {
        SmartCalcConfig::load_from_json(JSON_DATA)
    }
}

impl<'a> SmartCalcConfig<'a> {
    pub fn get_currency<T: AsRef<str>>(&self, currency: T) -> Option<Arc<CurrencyInfo>> {
        self.currency
            .get(currency.as_ref())
            .map(|currency_info| currency_info.clone())
    }

    pub fn load_from_json(json_data: &'a str) -> Self {
        let mut config = SmartCalcConfig {
            json_data: match from_str(json_data) {
                Ok(data) => data,
                Err(error) => panic!("JSON parse error: {}", error)
            },
            format: LanguageData::new(),
            currency: LanguageData::new(),
            currency_alias: LanguageData::new(),
            currency_rate: CurrencyData::new(),
            token_parse_regex: LanguageData::new(),
            word_group: LanguageData::new(),
            constant_pair: LanguageData::new(),
            language_alias_regex: LanguageData::new(),
            rule: LanguageData::new(),
            month_regex: LanguageData::new(),
            numeric_notation: LanguageData::new(),
            alias_regex: Vec::new(),
            decimal_seperator: ",",
            thousand_separator: "."
        };

        for (name, currency) in config.json_data.currencies.iter() {
            config.currency.insert(name.as_ref(), currency.clone());
        }

        for (from, to) in config.json_data.alias.iter() {
            match Regex::new(&format!(r"\b{}\b", from)) {
                Ok(re) => config.alias_regex.push((re, to.as_ref())),
                Err(error) => log::error!("Alias parser error ({}) {}", from, error)
            }
        }

        for (language, language_object) in config.json_data.languages.iter() {
            let mut language_clone = language_object.format.clone();
            language_clone.language = language.as_ref();
            config.format.insert(language.as_ref(), language_clone);
        }

        for (key, value) in config.json_data.currency_alias.iter() {
            match config.get_currency(value.as_ref()) {
                Some(currency) => { config.currency_alias.insert(key.as_ref(), currency.clone()); },
                None => log::warn!("'{}' currency not found at alias", value)
            };
        }

        for (key, value) in config.json_data.currency_rates.iter() {
            match config.get_currency(key.as_ref()) {
                Some(currency) => { config.currency_rate.insert(currency.clone(), *value); },
                None => log::warn!("'{}' currency not found at rate", key)
            };
        }

        for (language, language_constant) in config.json_data.languages.iter() {
            let mut language_aliases = Vec::new();
            for (alias, target_name) in language_constant.alias.iter() {
                
                match Regex::new(&format!(r"\b{}\b", alias)) {
                    Ok(re) => language_aliases.push((re, target_name.as_ref())),
                    Err(error) => log::error!("Alias parser error ({}) {}", alias, error)
                }

            }

            config.language_alias_regex.insert(language.as_ref(), language_aliases);
        }
        
        for (parse_type, items) in &config.json_data.parse {
            let mut patterns = Vec::new();
            for pattern in items {
                match Regex::new(&pattern) {
                    Ok(re) => patterns.push(re),
                    Err(error) => log::error!("Token parse regex error ({}) {}", pattern, error)
                }
            }

            config.token_parse_regex.insert(parse_type.as_ref(), patterns);
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

            config.month_regex.insert(language.as_ref(), language_group);
        }

        for (language, language_constant) in config.json_data.languages.iter() {
            let mut word_groups = BTreeMap::new();
            for (word_group_name, word_group_items) in language_constant.word_group.iter() {
                let mut patterns = Vec::new();

                for pattern in word_group_items {
                    patterns.push(pattern.as_ref());
                }

                word_groups.insert(word_group_name.as_ref(), patterns);
            }

            config.word_group.insert(language.as_ref(), word_groups);
        }

        for (language, language_constant) in config.json_data.languages.iter() {
            let mut constants = BTreeMap::new();
            for (alias_name, constant_type) in language_constant.constant_pair.iter() {

                match ConstantType::from_u8(*constant_type) {
                    Some(const_type) => {
                        constants.insert(alias_name.as_ref(), const_type);
                    },
                    _ => log::error!("Constant type not parsed. {}", constant_type)
                };
            }

            config.constant_pair.insert(language.as_ref(), constants);
        }
        
        for (language, language_constant) in config.json_data.languages.iter() {
            let mut language_rules = Vec::new();
            for (rule_name, rule) in language_constant.rules.iter() {
                if let Some(function_ref) = RULE_FUNCTIONS.get(rule_name.as_ref()) {
                    let mut function_items = Vec::new();

                    for rule_item in &rule.rules {
                        let mut session = Session::new();
                        session.set_language(language.as_ref());
                        session.set_text(rule_item.as_ref());
                        
                        let ref_session = RefCell::new(session);
                        function_items.push(Tokinizer::token_infos(&config, &ref_session));
                    }

                    language_rules.push((rule_name.as_ref(), *function_ref, function_items));
                }
                else {
                    log::warn!("Function not found : {}", rule_name);
                }
            }

            config.rule.insert(language.as_ref(), language_rules);
        }

        config
    }
}