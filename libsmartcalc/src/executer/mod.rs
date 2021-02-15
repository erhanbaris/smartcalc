use alloc::vec::Vec;
use alloc::string::String;
use core::cell::RefCell;
use alloc::format;
use alloc::string::ToString;
use log;

use crate::worker::rule::RuleItemList;
use crate::worker::rule::RULE_FUNCTIONS;
use crate::tokinizer::{Tokinizer, TokenLocation, TokenLocationStatus};
use crate::syntax::SyntaxParser;
use crate::types::{Token, TokenType, BramaAstType, VariableInfo, Money};
use crate::compiler::Interpreter;
use crate::logger::{LOGGER};
use crate::constants::{JSON_DATA, CURRENCIES, CURRENCY_ALIAS, SYSTEM_INITED, TOKEN_PARSE_REGEXES, ALIAS_REGEXES, RULES, CURRENCY_RATES, WORD_GROUPS};

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

pub fn token_generator(token_locations: &Vec<TokenLocation>) -> Vec<Token> {
    let mut tokens = Vec::new();

    for token_location in token_locations.iter() {
        if token_location.status == TokenLocationStatus::Active {
            match &token_location.token_type {
                Some(token_type) => {
                    let token = Token {
                        start: token_location.start as u16,
                        end: token_location.end as u16,
                        is_temp: false,
                        token: token_type.clone()
                    };

                    tokens.push(token);
                },
                _ => ()
            };
        }
    }

    return tokens;
}

pub fn missing_token_adder(tokens: &mut Vec<Token>) {
    let mut index = 0;
    for (token_index, token) in tokens.iter().enumerate() {
        match token.token {
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

    if let TokenType::Operator(_) = tokens[index].token {
        tokens.insert(index, Token {
            start: 0,
            end: 1,
            token: TokenType::Number(0.0),
            is_temp: true
        });
    }

    while index < tokens.len() {
        match tokens[index].token {
            TokenType::Operator(_) => operator_required = false,
            _ => {
                if operator_required {
                    log::debug!("Added missing operator between two token");
                    tokens.insert(index, Token {
                        start: 0,
                        end: 1,
                        token: TokenType::Operator('+'),
                        is_temp: true
                    });
                    index += 1;
                }
                operator_required = true;
            }
        };
        
        index += 1;
    }
}

pub fn initialize() {
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

                if let Some(group) = json.get("rules").unwrap().as_object() {
                    for (language, rules_object) in group.iter() {
                        let mut rule_items = RuleItemList::new();
                        for (function_name, rules) in rules_object.as_object().unwrap().iter() {

                            if let Some(function_ref) = RULE_FUNCTIONS.get(function_name) {
                                let mut function_items = Vec::new();
        
                                for item in  rules.as_array().unwrap().iter() {
                                    match Tokinizer::token_locations(&item.as_str().unwrap().to_string()) {
                                        Some(tokens) => function_items.push(tokens),
                                        _ => () //println!("Error : token_locations not working")
                                    }
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
            Err(error) => panic!(&format!("Initialize json not parsed. Error: {}", error)[..])
        };
    }
}


pub fn token_cleaner(tokens: &mut Vec<Token>) {
    let mut index = 0;
    for (token_index, token) in tokens.iter().enumerate() {
        match token.token {
            TokenType::Operator('=') => {
                index = token_index as usize + 1;
                break;
            },
            _ => ()
        };
    }

    while index < tokens.len() {
        if let TokenType::Text(_) = tokens[index].token {
            tokens.remove(index);
        }
        else {
            index += 1;
        }
    }
}

pub fn execute(data: &String, _language: &String) -> Vec<Result<(Vec<TokenLocation>, alloc::rc::Rc<BramaAstType>), String>> {
    let mut results     = Vec::new();
    let storage         = alloc::rc::Rc::new(Storage::new());

    for text in data.lines() {
        let prepared_text = text.to_string();

        if prepared_text.len() == 0 {
            storage.asts.borrow_mut().push(alloc::rc::Rc::new(BramaAstType::None));
            results.push(Ok((Vec::new(), alloc::rc::Rc::new(BramaAstType::None))));
            continue;
        }

        let mut tokinize = Tokinizer::new(&prepared_text.to_string());
        tokinize.tokinize_with_regex();
        tokinize.apply_aliases();
        Token::update_for_variable(&mut tokinize.token_locations, storage.clone());
        tokinize.apply_rules();
        let mut tokens = token_generator(&tokinize.token_locations);
        token_cleaner(&mut tokens);

        missing_token_adder(&mut tokens);

        let tokens_rc = alloc::rc::Rc::new(tokens);
        let syntax = SyntaxParser::new(tokens_rc.clone(), storage.clone());

        match syntax.parse() {
            Ok(ast) => {
                let ast_rc = alloc::rc::Rc::new(ast);
                storage.asts.borrow_mut().push(ast_rc.clone());

                match Interpreter::execute(ast_rc.clone(), storage.clone()) {
                    Ok(ast) => {
                        results.push(Ok((tokinize.token_locations, ast.clone())))
                    },
                    Err(error) => results.push(Err(error))
                };
            },
            Err((error, _, _)) => log::info!("Syntax parse error, {}", error)
        }
    }

    results
}