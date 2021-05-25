use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::collections::btree_map::BTreeMap;
use regex::Regex;
use serde_json::from_str;
use crate::types::Money;
use crate::worker::rule::RuleLanguage;
use crate::tokinizer::Tokinizer;
use crate::worker::rule::RULE_FUNCTIONS;
use crate::constants::*;

pub struct SmartCalcConfig {
    pub json_data: JsonConstant,
    pub format: BTreeMap<String, JsonFormat>,
    pub currency: BTreeMap<String, Money>,
    pub currency_alias: BTreeMap<String, String>,
    pub currency_rate: BTreeMap<String, f64>,
    pub token_parse_regex: BTreeMap<String, Vec<Regex>>,
    pub word_group: BTreeMap<String, BTreeMap<String, Vec<String>>>,
    pub constant_pair: BTreeMap<String, BTreeMap<String, ConstantType>>,
    pub alias_regex: BTreeMap<String, Vec<(Regex, String)>>,
    pub rule: RuleLanguage,
    pub month_regex: MonthLanguage
}

impl Default for SmartCalcConfig {
    fn default() -> Self {
        SmartCalcConfig::load_from_json(&JSON_DATA)
    }
}

impl SmartCalcConfig {
    pub fn load_from_json(json_data: &str) -> Self {
        let mut format_collection = BTreeMap::new();
        let mut currency_collection = BTreeMap::new();
        let mut currency_alias_collection = BTreeMap::new();
        let mut currency_rate_collection = BTreeMap::new();
        let mut token_parse_regex_collection = BTreeMap::new();
        let mut word_group_collection = BTreeMap::new();
        let mut constant_pair_collection = BTreeMap::new();
        let mut alias_regex_collection = BTreeMap::new();
        let mut rule_collection = RuleLanguage::new();
        let mut month_regex_collection = MonthLanguage::new();

        let json_data: JsonConstant = match from_str(&json_data) {
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

        for (name, currency) in json_data.currencies.iter() {
            currency_collection.insert(name.to_lowercase(), currency.clone());
        }

        for (language, language_object) in json_data.languages.iter() {
            let mut language_clone = language_object.format.clone();
            language_clone.language = language.to_string();
            format_collection.insert(language.to_string(), language_clone);
        }

        currency_alias_collection = json_data.currency_alias.clone();
        currency_rate_collection = json_data.currency_rates.clone();

        for (language, language_constant) in json_data.languages.iter() {
            let mut language_aliases = Vec::new();
            for (alias, target_name) in language_constant.alias.iter() {
                
                match Regex::new(&format!(r"\b{}\b", alias)) {
                    Ok(re) => language_aliases.push((re, target_name.to_string())),
                    Err(error) => log::error!("Alias parser error ({}) {}", alias, error)
                }

            }

            alias_regex_collection.insert(language.to_string(), language_aliases);
        }
        
        for (parse_type, items) in &json_data.parse {
            let mut patterns = Vec::new();
            for pattern in items {
                match Regex::new(&pattern) {
                    Ok(re) => patterns.push(re),
                    Err(error) => log::error!("Token parse regex error ({}) {}", pattern, error)
                }
            }

            token_parse_regex_collection.insert(parse_type.to_string(), patterns);
        }

        for (language, language_constant) in json_data.languages.iter() {
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

            month_regex_collection.insert(language.to_string(), language_group);
        }

        for (language, language_constant) in json_data.languages.iter() {
            let mut word_groups = BTreeMap::new();
            for (word_group_name, word_group_items) in language_constant.word_group.iter() {
                let mut patterns = Vec::new();

                for pattern in word_group_items {
                    patterns.push(pattern.to_string());
                }

                word_groups.insert(word_group_name.to_string(), patterns);
            }

            word_group_collection.insert(language.to_string(), word_groups);
        }

        for (language, language_constant) in json_data.languages.iter() {

            let mut constants = BTreeMap::new();
            for (alias_name, constant_type) in language_constant.constant_pair.iter() {

                match ConstantType::from_u8(*constant_type) {
                    Some(const_type) => {
                        constants.insert(alias_name.to_string(), const_type);
                    },
                    _ => log::error!("Constant type not parsed. {}", constant_type)
                };
            }

            constant_pair_collection.insert(language.to_string(), constants);
        }
        
        for (language, language_constant) in json_data.languages.iter() {
            let mut language_rules = Vec::new();
            for (rule_name, rules) in language_constant.rules.iter() {
                if let Some(function_ref) = RULE_FUNCTIONS.get(rule_name) {
                    let mut function_items = Vec::new();

                    for item in  rules {
                        function_items.push(Tokinizer::token_infos(&language, item));
                    }

                    language_rules.push((rule_name.to_string(), *function_ref, function_items));
                }
                else {
                    log::warn!("Function not found : {}", rule_name);
                }
            }

            rule_collection.insert(language.to_string(), language_rules);
        }

        SmartCalcConfig {
            json_data: json_data,
            format: format_collection,
            currency: currency_collection,
            currency_alias: currency_alias_collection,
            currency_rate: currency_rate_collection,
            token_parse_regex: token_parse_regex_collection,
            word_group: word_group_collection,
            constant_pair: constant_pair_collection,
            alias_regex: alias_regex_collection,
            rule: rule_collection,
            month_regex: month_regex_collection
        }
    }
}