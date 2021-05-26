use alloc::vec::Vec;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use core::cell::RefCell;
use crate::compiler::Interpreter;
use crate::logger::LOGGER;
use crate::syntax::SyntaxParser;
use crate::token::ui_token::UiToken;
use crate::tokinizer::TokenInfo;
use crate::tokinizer::TokenInfoStatus;
use crate::tokinizer::Tokinizer;
use crate::types::TokenType;
use crate::types::{BramaAstType, VariableInfo};
use crate::formatter::format_result;
use crate::worker::tools::read_currency;

use regex::Regex;

pub type ParseFunc     = fn(data: &mut String, group_item: &[Regex]) -> String;
pub type ExecuteResult = Vec<Result<(Vec<UiToken>, Rc<BramaAstType>), String>>;

use crate::config::SmartCalcConfig;

#[derive(Default)]
pub struct Storage {
    pub asts: RefCell<Vec<Rc<BramaAstType>>>,
    pub variables: RefCell<Vec<Rc<VariableInfo>>>
}

impl Storage {
    pub fn new() -> Storage {
        Storage::default()
    }
}

pub struct SmartCalc {
    config: SmartCalcConfig
}

impl Default for SmartCalc {
    fn default() -> Self {
        SmartCalc {
            config: SmartCalcConfig::default()
        }
    }
}

impl SmartCalc {
    pub fn load_from_json(json_data: &str) -> Self {
        SmartCalc {
            config: SmartCalcConfig::load_from_json(json_data)
        }
    }

    pub fn update_currency(&mut self, currency: &str, rate: f64) -> bool {
        match read_currency(&self.config, currency) {
            Some(real_currency) => {
                self.config.currency_rate.insert(real_currency, rate);
                true
            },
             _ => false
        }
    }

    fn token_generator(&self, token_infos: &[TokenInfo]) -> Vec<TokenType> {
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
    
    pub fn format_result(&self, language: &str, result: Rc<BramaAstType>) -> String {
        match self.config.format.get(language) {
            Some(formats) => format_result(&self.config, formats, result),
            _ => "".to_string()
        }
    }

    fn missing_token_adder(&self, tokens: &mut Vec<TokenType>) {
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


    pub fn token_cleaner(&self, tokens: &mut Vec<TokenType>) {
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

    pub fn execute(&self, language: &str, data: &str) -> ExecuteResult {
        let mut results     = Vec::new();
        let storage         = Rc::new(Storage::new());
        let lines = match Regex::new(r"\r\n|\n") {
            Ok(re) => re.split(data).collect::<Vec<_>>(),
            _ => data.lines().collect::<Vec<_>>()
        };

        for text in lines {
            log::debug!("> {}", text);
            let prepared_text = text.to_string();

            if prepared_text.is_empty() {
                storage.asts.borrow_mut().push(Rc::new(BramaAstType::None));
                results.push(Ok((Vec::new(), Rc::new(BramaAstType::None))));
                continue;
            }

            let mut tokinize = Tokinizer::new(language, &prepared_text.to_string(), &self.config);
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
            let mut tokens = self.token_generator(&tokinize.token_infos);
            log::debug!(" > token_generator");
            self.token_cleaner(&mut tokens);
            log::debug!(" > token_cleaner");

            self.missing_token_adder(&mut tokens);
            log::debug!(" > missing_token_adder");

            let tokens_rc = Rc::new(tokens);
            let syntax = SyntaxParser::new(tokens_rc.clone(), storage.clone());

            log::debug!(" > parse starting");

            match syntax.parse() {
                Ok(ast) => {
                    log::debug!(" > parse Ok");
                    let ast_rc = Rc::new(ast);
                    storage.asts.borrow_mut().push(ast_rc.clone());

                    match Interpreter::execute(&self.config, ast_rc.clone(), storage.clone()) {
                        Ok(ast) => {
                            results.push(Ok((tokinize.ui_tokens.get_tokens(), ast.clone())))
                        },
                        Err(error) => results.push(Err(error))
                    };
                },
                Err((error, _, _)) => {
                    log::debug!(" > parse Err");
                    results.push(Ok((tokinize.ui_tokens.get_tokens(), Rc::new(BramaAstType::None))));
                    log::info!("Syntax parse error, {}", error);
                }
            }
        }

        results
    }
}