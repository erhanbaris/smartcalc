use std::rc::Rc;
use std::cell::RefCell;

use crate::worker::WorkerExecuter;
use crate::tokinizer::Tokinizer;
use crate::syntax::SyntaxParser;
use crate::types::{Token, TokenType, BramaAstType, VariableInfo};
use crate::compiler::Interpreter;
use crate::constants::{JSON_DATA, CURRENCIES, SYSTEM_INITED, TOKEN_PARSE_REGEXES, ALIAS_REGEXES};

use serde_json::{Value, from_str};
use regex::{Regex, Captures};

pub type ParseFunc = fn(data: &mut String, group_item: &Vec<Regex>) -> String;

pub struct Storage {
    pub asts: RefCell<Vec<Rc<BramaAstType>>>,
    pub variables: RefCell<Vec<Rc<VariableInfo>>>
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            asts: RefCell::new(Vec::new()),
            variables: RefCell::new(Vec::new())
        }
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

    if let TokenType::Operator(_) = tokens[index].token {
        tokens.insert(index, Token {
            start: 0,
            end: 1,
            token: TokenType::Number(0.0),
            is_temp: true
        });
    }

    index += 1;
    while index < tokens.len() {
        match tokens[index].token {
            TokenType::Operator(_) => index += 2,
            _ => {
                tokens.insert(index, Token {
                    start: 0,
                    end: 1,
                    token: TokenType::Operator('+'),
                    is_temp: true
                });
                index += 2;
            }
        };
    }

    if let TokenType::Operator(_) = tokens[tokens.len()-1].token {
        tokens.insert(tokens.len()-1, Token {
            start: 0,
            end: 1,
            token: TokenType::Number(0.0),
            is_temp: true
        });
    }
}

pub fn prepare_code(data: &String) -> String {
    let mut data_str = data.to_string();
    for (re, value) in ALIAS_REGEXES.lock().unwrap().iter() {
        data_str = re.replace_all(&data_str, |_: &Captures| {
            value.to_string()
        }).to_string();
    }

    data_str
}

pub fn initialize() {
    if unsafe { !SYSTEM_INITED } {
        let json_value: serde_json::Result<Value> = from_str(&JSON_DATA);
        match json_value {
            Ok(json) => {
                if let Some(group) = json.get("currencies").unwrap().as_object() {
                    for (key, value) in group.iter() {
                        CURRENCIES.lock().unwrap().insert(key.as_str().to_string(), value.as_str().unwrap().to_string());
                    }
                }

                if let Some(group) = json.get("parse").unwrap().as_object() {
                    for (group, group_item) in group.iter() {
                        let mut patterns = Vec::new();

                        for pattern in group_item.as_array().unwrap() {
                            let re = Regex::new(pattern.as_str().unwrap()).unwrap();
                            patterns.push(re);
                        }

                        TOKEN_PARSE_REGEXES.lock().unwrap().insert(group.as_str().to_string(), patterns);
                    }
                }

                if let Some(group) = json.get("alias").unwrap().as_object() {
                    for (key, value) in group.iter() {
                        let re = Regex::new(&format!(r"\b{}\b", key.as_str())[..]).unwrap();
                        ALIAS_REGEXES.lock().unwrap().push((re, value.as_str().unwrap().to_string()));
                    }
                }

                unsafe {
                    SYSTEM_INITED = true;
                }
            },
            Err(error) => panic!(format!("Initialize json not parsed. Error: {}", error))
        };
    }
}

pub fn execute(data: &String, language: &String) -> Vec<Result<(Rc<Vec<Token>>, Rc<BramaAstType>), String>> {
    let mut results     = Vec::new();
    let storage         = Rc::new(Storage::new());
    let worker_executer = WorkerExecuter::new();

    for text in data.lines() {
        let prepared_text = prepare_code(&text.to_string());

        if prepared_text.len() == 0 {
            storage.asts.borrow_mut().push(Rc::new(BramaAstType::None));
            results.push(Ok((Rc::new(Vec::new()), Rc::new(BramaAstType::None))));
            continue;
        }

        let result = Tokinizer::tokinize(&prepared_text.to_string());
        match result {
            Ok(mut tokens) => {
                //println!("tokens {:?}", tokens);
                Token::update_for_variable(&mut tokens, storage.clone());
                let original_tokens = Rc::new(tokens.clone());
                worker_executer.process(&language, &mut tokens, storage.clone());
                token_cleaner(&mut tokens);
                missing_token_adder(&mut tokens);

                let tokens_rc = Rc::new(tokens);
                let syntax = SyntaxParser::new(tokens_rc.clone(), storage.clone());
                match syntax.parse() {
                    Ok(ast) => {
                        let ast_rc = Rc::new(ast);
                        storage.asts.borrow_mut().push(ast_rc.clone());

                        match Interpreter::execute(ast_rc.clone(), storage.clone()) {
                            Ok(ast) => results.push(Ok((original_tokens.clone(), ast.clone()))),
                            Err(error) => results.push(Err(error))
                        };

                        //println!("Ast {:?}", ast_rc.clone());
                    },
                    Err((error, _, _)) => println!("error, {}", error)
                }
            },
            Err((error, _, _)) => {
                println!("error, {}", error);
                results.push(Err(error.to_string()));
                storage.asts.borrow_mut().push(Rc::new(BramaAstType::None));
            }
        };
    }

    results
}