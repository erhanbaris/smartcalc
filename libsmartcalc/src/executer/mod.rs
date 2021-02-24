use alloc::vec::Vec;
use alloc::string::String;
use core::cell::RefCell;
use alloc::format;
use alloc::string::ToString;
use log;

use crate::worker::rule::RuleItemList;
use crate::worker::rule::RULE_FUNCTIONS;
use crate::tokinizer::{Tokinizer, TokenInfo, TokenInfoStatus};
use crate::syntax::SyntaxParser;
use crate::types::{TokenType, BramaAstType, VariableInfo, Money};
use crate::compiler::Interpreter;
use crate::logger::{LOGGER};
use crate::token::ui_token::{UiToken};
use crate::constants::{Constant, JSON_DATA, CONSTANTS, CURRENCIES, CURRENCY_ALIAS, MONTHS_REGEXES, SYSTEM_INITED, TOKEN_PARSE_REGEXES, ALIAS_REGEXES, RULES, CURRENCY_RATES, WORD_GROUPS};

use serde_json::{from_str, Value};
use regex::{Regex};

pub type ParseFunc = fn(data: &mut String, group_item: &Vec<Regex>) -> String;

pub struct Storage {
    pub asts: RefCell<Vec<alloc::rc::Rc<BramaAstType>>>,
    pub variables: RefCell<Vec<alloc::rc::Rc<VariableInfo>>>
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            asts: RefCell::new(Vec::new()),
            variables: RefCell::new(Vec::new())
        }
    }
}

pub fn token_generator(token_infos: &Vec<TokenInfo>) -> Vec<TokenType> {
    let mut tokens = Vec::new();

    for token_location in token_infos.iter() {
        if token_location.status == TokenInfoStatus::Active {
            match &token_location.token_type {
                Some(token_type) => {
                    tokens.push(token_type.clone());
                },
                _ => ()
            };
        }
    }

    return tokens;
}

pub fn missing_token_adder(tokens: &mut Vec<TokenType>) {
    let mut index = 0;
    for (token_index, token) in tokens.iter().enumerate() {
        match token {
            TokenType::Operator('=') => {
                index = token_index as usize + 1;
                break;
            },
            _ => ()
        };
    }

    if tokens.len() == 0 {
        return;
    }

    if index + 1 >= tokens.len() {
        return;
    }

    let mut operator_required = false;

    if let TokenType::Operator(_) = tokens[index] {
        tokens.insert(index, TokenType::Number(0.0));
    }

    while index < tokens.len() {
        match tokens[index] {
            TokenType::Operator(_) => operator_required = false,
            _ => {
                if operator_required {
                    log::debug!("Added missing operator between two token");
                    tokens.insert(index, TokenType::Operator('+'));
                    index += 1;
                }
                operator_required = true;
            }
        };
        
        index += 1;
    }
}

pub fn initialize() {
    use alloc::collections::btree_map::BTreeMap;

    if unsafe { !SYSTEM_INITED } {

        match log::set_logger(&LOGGER) {
            Ok(_) => {
                if cfg!(debug_assertions) {
                    log::set_max_level(log::LevelFilter::Debug)
                } else {
                    log::set_max_level(log::LevelFilter::Info)
                }
            },
            _ => ()
        };

        let constant: Constant = match from_str(&JSON_DATA) {
            Ok(data) => data,
            Err(error) => {
                log::debug!("{}", error);
                Constant {
                    constants: BTreeMap::new(),
                    parse: BTreeMap::new(),
                    currency_alias: BTreeMap::new(),
                    currency_rates: BTreeMap::new(),
                    currencies:  BTreeMap::new(),
                    default_language: "en".to_string(),
                    languages: BTreeMap::new()
                  }
            }
        };
        
        let json_value: serde_json::Result<Value> = from_str(&JSON_DATA);
        match json_value {
            Ok(json) => {
                if let Some(group) = json.get("currencies").unwrap().as_object() {
                    for (key, value) in group.iter() {
                        CURRENCIES.write().unwrap().insert(key.as_str().to_string().to_lowercase(), Money {
                            code: value.get("code").unwrap().as_str().unwrap().to_string(),
                            symbol: value.get("symbol").unwrap().as_str().unwrap().to_string(),
                            thousands_separator: value.get("thousandsSeparator").unwrap().as_str().unwrap().to_string(),
                            decimal_separator: value.get("decimalSeparator").unwrap().as_str().unwrap().to_string(),
                            symbol_on_left: value.get("symbolOnLeft").unwrap().as_bool().unwrap(),
                            space_between_amount_and_symbol: value.get("spaceBetweenAmountAndSymbol").unwrap().as_bool().unwrap(),
                            decimal_digits: value.get("decimalDigits").unwrap().as_f64().unwrap() as u8
                        });
                    }
                }

                if let Some(group) = json.get("currency_alias").unwrap().as_object() {
                    for (key, value) in group.iter() {
                        CURRENCY_ALIAS.write().unwrap().insert(key.as_str().to_string(), value.as_str().unwrap().to_string());
                    }
                }
                
                if let Some(group) = json.get("currency_rates").unwrap().as_object() {
                    for (key, value) in group.iter() {
                        CURRENCY_RATES.write().unwrap().insert(key.as_str().to_string(), value.as_f64().unwrap());
                    }
                }

                if let Some(group) = json.get("parse").unwrap().as_object() {
                    for (group, group_item) in group.iter() {
                        let mut patterns = Vec::new();

                        for pattern in group_item.as_array().unwrap() {
                            let re = Regex::new(pattern.as_str().unwrap()).unwrap();
                            patterns.push(re);
                        }

                        TOKEN_PARSE_REGEXES.write().unwrap().insert(group.as_str().to_string(), patterns);
                    }
                }

                if let Some(months) = json.get("months").unwrap().as_object() {
                    for (language, group_item) in months.iter() {
                        let mut language_group = Vec::new();

                        for (key, value) in group_item.as_object().unwrap() {

                            let re = Regex::new(&format!(r"\b{}\b", key.as_str())[..]).unwrap();
                            let month_number =  match value.as_u64() {
                                Some(number) => number,
                                 None => 1
                            };

                            language_group.push((re, month_number));
                        }

                        MONTHS_REGEXES.write().unwrap().insert(language.as_str().to_string(), language_group);
                    }
                }

                if let Some(group) = json.get("word_group").unwrap().as_object() {
                    for (group, group_item) in group.iter() {
                        let mut patterns = Vec::new();

                        for pattern in group_item.as_array().unwrap() {
                            patterns.push(pattern.as_str().unwrap().to_string());
                        }

                        WORD_GROUPS.write().unwrap().insert(group.as_str().to_string(), patterns);
                    }
                }

                if let Some(group) = json.get("alias").unwrap().as_object() {
                    for (key, value) in group.iter() {
                        let re = Regex::new(&format!(r"\b{}\b", key.as_str())[..]).unwrap();
                        ALIAS_REGEXES.write().unwrap().push((re, value.as_str().unwrap().to_string()));
                    }
                }

                if let Some(group) = json.get("constants").unwrap().as_object() {
                    for (key, value) in group.iter() {
                        CONSTANTS.write().unwrap().insert(key.to_string(), value.as_u64().unwrap() as u8);
                    }
                }

                if let Some(group) = json.get("rules").unwrap().as_object() {
                    for (language, rules_object) in group.iter() {
                        let mut rule_items = RuleItemList::new();
                        for (function_name, rules) in rules_object.as_object().unwrap().iter() {

                            if let Some(function_ref) = RULE_FUNCTIONS.get(function_name) {
                                let mut function_items = Vec::new();
        
                                for item in  rules.as_array().unwrap().iter() {
                                    function_items.push(Tokinizer::token_infos(&item.as_str().unwrap().to_string()));
                                }
        
                                rule_items.push((*function_ref, function_items));
                            }
                            else {
                                log::warn!("Function not found : {}", function_name);
                            }
                        }

                        RULES.write().unwrap().insert(language.to_string(), rule_items);
                    }
                }

                unsafe {
                    SYSTEM_INITED = true;
                }
            },
            Err(error) => panic!("{}", &format!("Initialize json not parsed. Error: {}", error))
        };
    }
}


pub fn token_cleaner(tokens: &mut Vec<TokenType>) {
    let mut index = 0;
    for (token_index, token) in tokens.iter().enumerate() {
        match token {
            TokenType::Operator('=') => {
                index = token_index as usize + 1;
                break;
            },
            _ => ()
        };
    }

    while index < tokens.len() {
        match tokens[index] {
            TokenType::Text(_) => {
                tokens.remove(index);
            },
            _ => index += 1
        };
    }
}

pub fn execute(data: &String, _language: &String) -> Vec<Result<(Vec<UiToken>, alloc::rc::Rc<BramaAstType>), String>> {
    let mut results     = Vec::new();
    let storage         = alloc::rc::Rc::new(Storage::new());
    let lines = match Regex::new(r"\r\n|\n") {
        Ok(re) => re.split(data).collect::<Vec<_>>(),
        _ => data.lines().collect::<Vec<_>>()
    };

    for text in lines {
        log::debug!("> {}", text);
        let prepared_text = text.to_string();

        if prepared_text.len() == 0 {
            storage.asts.borrow_mut().push(alloc::rc::Rc::new(BramaAstType::None));
            results.push(Ok((Vec::new(), alloc::rc::Rc::new(BramaAstType::None))));
            continue;
        }

        let mut tokinize = Tokinizer::new(&prepared_text.to_string());
        tokinize.language_based_tokinize();
        log::debug!(" > language_based_tokinize");
        tokinize.tokinize_with_regex();
        log::debug!(" > tokinize_with_regex");
        tokinize.apply_aliases();
        log::debug!(" > apply_aliases");
        TokenType::update_for_variable(&mut tokinize, storage.clone());
        log::debug!(" > update_for_variable");
        tokinize.apply_rules();
        log::debug!(" > apply_rules");
        let mut tokens = token_generator(&tokinize.token_infos);
        log::debug!(" > token_generator");
        token_cleaner(&mut tokens);
        log::debug!(" > token_cleaner");

        missing_token_adder(&mut tokens);
        log::debug!(" > missing_token_adder");

        let tokens_rc = alloc::rc::Rc::new(tokens);
        let syntax = SyntaxParser::new(tokens_rc.clone(), storage.clone());

        log::debug!(" > parse starting");

        match syntax.parse() {
            Ok(ast) => {
                log::debug!(" > parse Ok");
                let ast_rc = alloc::rc::Rc::new(ast);
                storage.asts.borrow_mut().push(ast_rc.clone());

                match Interpreter::execute(ast_rc.clone(), storage.clone()) {
                    Ok(ast) => {
                        results.push(Ok((tokinize.ui_tokens.clone(), ast.clone())))
                    },
                    Err(error) => results.push(Err(error))
                };
            },
            Err((error, _, _)) => {
                log::debug!(" > parse Err");
                results.push(Ok((tokinize.ui_tokens.clone(), alloc::rc::Rc::new(BramaAstType::None))));
                log::info!("Syntax parse error, {}", error);
            }
        }
    }

    results
}