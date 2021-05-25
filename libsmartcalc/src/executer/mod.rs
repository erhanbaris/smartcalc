use alloc::vec::Vec;
use alloc::string::String;
use alloc::string::ToString;
use log;

use crate::app::Storage;
use crate::tokinizer::{Tokinizer, TokenInfo, TokenInfoStatus};
use crate::syntax::SyntaxParser;
use crate::types::{TokenType, BramaAstType};
use crate::compiler::Interpreter;
use crate::logger::{LOGGER};
use crate::token::ui_token::{UiToken};

use regex::{Regex};

pub fn token_generator(token_infos: &[TokenInfo]) -> Vec<TokenType> {
    let mut tokens = Vec::new();

    for token_location in token_infos.iter() {
        if token_location.status == TokenInfoStatus::Active {
            if let Some(token_type) = &token_location.token_type {
                tokens.push(token_type.clone());
            }
        }
    }

    tokens
}

pub fn missing_token_adder(tokens: &mut Vec<TokenType>) {
    let mut index = 0;
    if tokens.is_empty() {
        return;
    }
    
    for (token_index, token) in tokens.iter().enumerate() {
        match token {
            TokenType::Operator('=') | 
            TokenType::Operator('(')=> {
                index = token_index as usize + 1;
                break;
            },
            _ => ()
        };
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
    if let Ok(_) = log::set_logger(&LOGGER) {
        if cfg!(debug_assertions) {
            log::set_max_level(log::LevelFilter::Debug)
        } else {
            log::set_max_level(log::LevelFilter::Info)
        }
    }
}


pub fn token_cleaner(tokens: &mut Vec<TokenType>) {
    let mut index = 0;
    for (token_index, token) in tokens.iter().enumerate() {
        if let TokenType::Operator('=') = token {
            index = token_index as usize + 1;
            break;
        }
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

pub fn execute(language: &String, data: &str) -> Vec<Result<(Vec<UiToken>, alloc::rc::Rc<BramaAstType>), String>> {
    let mut results     = Vec::new();
    let storage         = alloc::rc::Rc::new(Storage::new());
    let lines = match Regex::new(r"\r\n|\n") {
        Ok(re) => re.split(data).collect::<Vec<_>>(),
        _ => data.lines().collect::<Vec<_>>()
    };

    for text in lines {
        log::debug!("> {}", text);
        let prepared_text = text.to_string();

        if prepared_text.is_empty() {
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