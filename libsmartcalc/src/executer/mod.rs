use alloc::vec::Vec;
use alloc::string::String;
use core::cell::RefCell;
use alloc::format;
use alloc::string::ToString;
use log;

use alloc::collections::btree_map::BTreeMap;
use crate::worker::rule::RULE_FUNCTIONS;
use crate::tokinizer::{Tokinizer, TokenInfo, TokenInfoStatus};
use crate::syntax::SyntaxParser;
use crate::types::{TokenType, BramaAstType, VariableInfo};
use crate::compiler::Interpreter;
use crate::logger::{LOGGER};
use crate::token::ui_token::{UiToken};
use crate::constants::{RULES, CONSTANT_PAIRS, JSON_CONSTANT_DEF, FORMATS, CURRENCIES, CURRENCY_ALIAS, MONTHS_REGEXES, SYSTEM_INITED, TOKEN_PARSE_REGEXES, ALIAS_REGEXES, CURRENCY_RATES, WORD_GROUPS, ConstantType, MonthInfo};

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
        
        match JSON_CONSTANT_DEF.read() {
            Ok(constant) => {

                match CURRENCIES.write() {
                    Ok(mut currencies) => {
                        for (name, currency) in constant.currencies.iter() {
                            currencies.insert(name.to_lowercase(), currency.clone());
                        }

                        log::info!("Currencies updated");
                    },
                    Err(error) => log::error!("Currencies assignation error. {}", error)
                };

                match FORMATS.write() {
                    Ok(mut formats) => {
                        for (language, language_object) in constant.languages.iter() {
                            let mut language_clone = language_object.format.clone();
                            language_clone.language = language.to_string();
                            formats.insert(language.to_string(), language_clone);
                        }

                        log::info!("Formats updated");
                    },
                    Err(error) => log::error!("Format assignation error. {}", error)
                };

                match CURRENCY_ALIAS.set(constant.currency_alias.clone()) {
                    Ok(_) => log::info!("Currency alias updated"),
                    Err(error) => log::error!("Currency alias assignation error. {}", error)
                };

                match CURRENCY_RATES.set(constant.currency_rates.clone()) {
                    Ok(_) => log::info!("Default currency rates updated"),
                    Err(error) => log::error!("Default currency rates update error. {}", error)
                };

                match ALIAS_REGEXES.write() {
                    Ok(mut aliases) => {
                        for (language, language_constant) in constant.languages.iter() {
                            let mut language_aliases = Vec::new();
                            for (alias, target_name) in language_constant.alias.iter() {
                                
                                match Regex::new(&format!(r"\b{}\b", alias)) {
                                    Ok(re) => language_aliases.push((re, target_name.to_string())),
                                    Err(error) => log::error!("Alias parser error ({}) {}", alias, error)
                                }

                            }

                            aliases.insert(language.to_string(), language_aliases);
                        }

                        log::info!("Alias regexes updated");
                    },
                    Err(error) => log::error!("Alias regex could not opened for write. {}", error)
                };

                match TOKEN_PARSE_REGEXES.write() {
                    Ok(mut regexes) => {
                        for (parse_type, items) in &constant.parse {
                            let mut patterns = Vec::new();
                            for pattern in items {
                                match Regex::new(&pattern) {
                                    Ok(re) => patterns.push(re),
                                    Err(error) => log::error!("Token parse regex error ({}) {}", pattern, error)
                                }
                            }
    
                            regexes.insert(parse_type.to_string(), patterns);
                        }

                        log::info!("Token parse regexes updated");
                    },
                    Err(error) => log::error!("Token parse regex could not opened for write. {}", error)
                };

                match MONTHS_REGEXES.write() {
                    Ok(mut months) => {
                        for (language, language_constant) in constant.languages.iter() {
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
    
                            months.insert(language.to_string(), language_group);
                        }

                        log::info!("Month informations updated");
                    },
                    Err(error) => log::error!("Month regex could not opened for write. {}", error)
                };

                match WORD_GROUPS.write() {
                    Ok(mut word_groups_collection) => {
                        for (language, language_constant) in constant.languages.iter() {
                            let mut word_groups = BTreeMap::new();
                            for (word_group_name, word_group_items) in language_constant.word_group.iter() {
                                let mut patterns = Vec::new();
        
                                for pattern in word_group_items {
                                    patterns.push(pattern.to_string());
                                }
        
                                word_groups.insert(word_group_name.to_string(), patterns);
                            }

                            word_groups_collection.insert(language.to_string(), word_groups);
                        }

                        log::info!("Word groups informations updated");
                    },
                    Err(error) => log::error!("Word group could not opened for write. {}", error)
                };

                match CONSTANT_PAIRS.write() {
                    Ok(mut constant_pairs) => {
                        for (language, language_constant) in constant.languages.iter() {

                            let mut constants = BTreeMap::new();
                            for (alias_name, constant_type) in language_constant.constant_pair.iter() {

                                match ConstantType::from_u8(*constant_type) {
                                    Some(const_type) => {
                                        constants.insert(alias_name.to_string(), const_type);
                                    },
                                    _ => log::error!("Constant type not parsed. {}", constant_type)
                                };
                            }

                            constant_pairs.insert(language.to_string(), constants);
                        }

                        log::info!("Constant pairs informations updated");
                    },
                    Err(error) => log::error!("Constant pairs could not opened for write. {}", error)
                };

                match RULES.write() {
                    Ok(mut rules_constant) => {
                        for (language, language_constant) in constant.languages.iter() {
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

                            rules_constant.insert(language.to_string(), language_rules);
                        }

                        log::info!("Rules informations updated");
                    },
                    Err(error) => log::error!("Rules constant could not opened for write. {}", error)
                };

                unsafe {
                    SYSTEM_INITED = true;
                }
            },
            Err(error) => log::error!("Alias regex assignation error. {}", error)
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

pub fn execute(language: &String, data: &String) -> Vec<Result<(Vec<UiToken>, alloc::rc::Rc<BramaAstType>), String>> {
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

        let mut tokinize = Tokinizer::new(language, &prepared_text.to_string());
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