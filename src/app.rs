/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::borrow::Borrow;
use core::cell::RefCell;
use core::ops::Deref;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use crate::Session;
use crate::tokinizer::{language_tokinizer, read_currency, dynamic_type_tokimizer, rule_tokinizer};

use crate::compiler::Interpreter;
use crate::logger::{LOGGER, initialize_logger};
use crate::syntax::SyntaxParser;
use crate::token::ui_token::UiToken;
use crate::tokinizer::{TokenInfo, alias_tokinizer, regex_tokinizer};
use crate::tokinizer::TokenInfoStatus;
use crate::tokinizer::Tokinizer;
use crate::tools::parse_timezone;
use crate::types::{TokenType, NumberType};
use crate::types::SmartCalcAstType;
use crate::formatter::format_result;
use crate::variable::update_token_variables;
use crate::config::SmartCalcConfig;

pub type ExecutionLine = Option<ExecuteLine>;
pub type RuleFunc<'a>  = fn(fields: &BTreeMap<String, &'a TokenType>) -> core::result::Result<TokenType, String>;

#[derive(Debug)]
#[derive(Default)]
pub struct ExecuteResult {
    pub status: bool,
    pub lines: Vec<ExecutionLine>
}

#[derive(Debug, Clone)]
pub struct ExecuteLineResult {
    pub output: String,
    pub ast: Rc<SmartCalcAstType>
}

impl ExecuteLineResult {
    pub fn new(output: String, ast: Rc<SmartCalcAstType>) -> Self {
        ExecuteLineResult { output, ast }
    }
}

#[derive(Debug)]
pub struct ExecuteLine {
    pub result: Result<ExecuteLineResult, String>,
    pub raw_tokens: Vec<Rc<TokenType>>,
    pub ui_tokens: Vec<UiToken>,
    pub calculated_tokens: Vec<Rc<TokenInfo>>
}

impl ExecuteLine {
    pub fn new(result: Result<ExecuteLineResult, String>, ui_tokens: Vec<UiToken>, raw_tokens: Vec<Rc<TokenType>>, calculated_tokens: Vec<Rc<TokenInfo>>) -> Self {
        ExecuteLine { result, ui_tokens, raw_tokens, calculated_tokens }
    }
}

pub struct SmartCalc {
    config: SmartCalcConfig
}

impl Default for SmartCalc {
    fn default() -> Self {
        initialize_logger();
        SmartCalc {
            config: SmartCalcConfig::default()
        }
    }
}

impl SmartCalc {
    pub fn set_decimal_seperator(&mut self, decimal_seperator: String) {
        self.config.decimal_seperator = decimal_seperator;
    }
    
    pub fn set_thousand_separator(&mut self, thousand_separator: String) {
        self.config.thousand_separator = thousand_separator;
    }
    
    pub fn set_timezone(&mut self, timezone: String) -> Result<(), String> {
        let timezone = match self.config.token_parse_regex.get("timezone") {
            Some(regexes) => {
                let capture = regexes[0].captures(&timezone).unwrap();
                match capture.name("timezone") {
                    Some(_) => parse_timezone(&self.config, &capture),
                    None => None
                }
            },
            _ => None
        };
        
        match timezone {
            Some((timezone, offset)) => {
                self.config.timezone = timezone.to_uppercase();
                self.config.timezone_offset = offset;
                Ok(())
            },
            None => Err("Timezone information not found".to_string())
        }
    }
    
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

    pub fn add_rule(&mut self, language: String, rules: Vec<String>, _callback: RuleFunc) -> Result<(), ()> {
        let mut rule_tokens = Vec::new();
        
        for rule_item in rules.iter() {
            let mut session = Session::new();
            session.set_language(language.to_string());
            session.set_text(rule_item.to_string());
            
            let ref_session = RefCell::new(session);
            rule_tokens.push(Tokinizer::token_infos(&self.config, &ref_session));
        }
                                
        Ok(())
    }

    pub(crate) fn token_generator(&self, session: &RefCell<Session>) {
        let mut tokens = Vec::new();
        let mut session_mut = session.borrow_mut();
        for token_location in session_mut.token_infos.iter() {
            if token_location.status.get() == TokenInfoStatus::Active {
                if let Some(token_type) = &token_location.token_type.borrow().deref() {
                    tokens.push(token_type.clone());
                }
            }
        }
        
        for token in tokens {
            session_mut.tokens.push(Rc::new(token));
        }
    }
    
    pub fn format_result(&self, session: &RefCell<Session>, result: Rc<SmartCalcAstType>) -> String {
        format_result(&self.config, session, result)
    }

    fn missing_token_adder(&self, session: &RefCell<Session>) {
        let mut index = 0;
        let mut session_mut = session.borrow_mut();
        let tokens = &mut session_mut.tokens;
        
        if tokens.is_empty() {
            return;
        }
        
        for (token_index, token) in tokens.iter().enumerate() {
            match token.deref() {
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

        if let TokenType::Operator('(') = tokens[index].deref() {
            index += 1;
        }

        let mut operator_required = false;

        if let TokenType::Operator(_) = tokens[index].deref() {
            tokens.insert(index, Rc::new(TokenType::Number(0.0, NumberType::Decimal)));
        }

        while index < tokens.len() {
            match tokens[index].deref() {
                TokenType::Operator(_) => operator_required = false,
                _ => {
                    if operator_required {
                        log::debug!("Added missing operator between two token");
                        tokens.insert(index, Rc::new(TokenType::Operator('+')));
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


    pub fn token_cleaner(&self, session: &RefCell<Session>) {
        let mut index = 0;
        let mut session_mut = session.borrow_mut();
        for (token_index, token) in session_mut.token_infos.iter().enumerate() {
            if let Some(TokenType::Operator('=')) = token.token_type.borrow().deref() {
                index = token_index as usize + 1;
                break;
            };
        }

        while index < session_mut.tokens.len() {
            match session_mut.tokens[index].deref() {
                TokenType::Text(_) => {
                    session_mut.tokens.remove(index);
                },
                _ => index += 1
            };
        }
    }

    pub(crate) fn execute_text(&self, session: &RefCell<Session>) -> ExecutionLine {
        log::debug!("> {}", session.borrow().current());
        if session.borrow().current().is_empty() {
            session.borrow_mut().add_ast(Rc::new(SmartCalcAstType::None));
            return None;
        }

        let mut tokinize = Tokinizer::new(&self.config, session);
        language_tokinizer(&mut tokinize);
        log::debug!(" > language_tokinizer");
        regex_tokinizer(&mut tokinize);
        log::debug!(" > regex_tokinizer");
        alias_tokinizer(&mut tokinize);
        log::debug!(" > alias_tokinizer");
        update_token_variables(&mut tokinize);
        log::debug!(" > update_token_variables");
        dynamic_type_tokimizer(&mut tokinize);
        log::debug!(" > dynamic_type_tokimizer");
        rule_tokinizer(&mut tokinize);
        log::debug!(" > apply_rules");
        self.token_generator(session);
        log::debug!(" > token_generator");
        self.token_cleaner(session);
        log::debug!(" > token_cleaner");

        self.missing_token_adder(session);
        log::debug!(" > missing_token_adder");

        if session.borrow().token_infos.is_empty() {
            session.borrow_mut().add_ast(Rc::new(SmartCalcAstType::None));
            return None;
        }

        let mut syntax = SyntaxParser::new(session);

        log::debug!(" > parse starting");

        let execution_result = match syntax.parse() {
            Ok(ast) => {
                log::debug!(" > parse Ok {:?}", ast);
                let ast_rc = Rc::new(ast);
                session.borrow_mut().add_ast(ast_rc.clone());

                match Interpreter::execute(&self.config, ast_rc, session) {
                    Ok(ast) => Ok(ExecuteLineResult::new(self.format_result(session, ast.clone()), ast)),
                    Err(error) => Err(error)
                }
            },
            Err((error, _, _)) => {
                log::debug!(" > parse Err");
                log::info!("Syntax parse error, {}", error);
                Err(error.to_string())
            }
        };
        
        Some(ExecuteLine::new(execution_result, tokinize.ui_tokens.get_tokens(), tokinize.session.borrow().tokens.clone(), tokinize.session.borrow().token_infos.clone()))
    }

    pub fn execute<Tlan: Borrow<str>, Tdata: Borrow<str>>(
        &self,
        language: Tlan,
        data: Tdata,
    ) -> ExecuteResult {
        let mut session = Session::new();

        session.set_text(data.borrow().to_string());
        session.set_language(language.borrow().to_string());

        let session = RefCell::new(session);
        self.execute_session(&session)
    }

    pub fn execute_session(&self, session: &RefCell<Session>) -> ExecuteResult {
        let mut results = ExecuteResult::default();

        if session.borrow().has_value() {
            results.status = true;
            loop {
                let line_result = self.execute_text(session);
                results.lines.push(line_result);
                session.borrow_mut().cleanup();
                if session.borrow().next().is_none() {
                    break;
                }
            }
        }

        results
    }
}
