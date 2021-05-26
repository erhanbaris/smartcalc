use alloc::vec::Vec;

use crate::tokinizer::{TokenInfo, TokenInfoStatus};
use crate::types::{TokenType};
use crate::logger::{LOGGER};

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
    if log::set_logger(&LOGGER).is_ok() {
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
