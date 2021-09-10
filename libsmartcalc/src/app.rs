use core::borrow::Borrow;
use core::cell::{Cell, RefCell};
use core::ops::Deref;

use alloc::vec::Vec;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use crate::compiler::Interpreter;
use crate::logger::LOGGER;
use crate::syntax::SyntaxParser;
use crate::token::ui_token::{UiToken, UiTokenCollection};
use crate::tokinizer::TokenInfo;
use crate::tokinizer::TokenInfoStatus;
use crate::tokinizer::Tokinizer;
use crate::types::TokenType;
use crate::types::{BramaAstType, VariableInfo};
use crate::formatter::format_result;
use crate::worker::tools::read_currency;
use regex::Regex;
use crate::config::SmartCalcConfig;

pub type ParseFunc     = fn(data: &mut String, group_item: &[Regex]) -> String;
pub type ExecutionLine = Option<ExecuteLine>;

#[derive(Debug)]
#[derive(Default)]
pub struct ExecuteResult {
    pub status: bool,
    pub lines: Vec<ExecutionLine>
}

#[derive(Debug)]
pub struct ExecuteLineResult {
    pub output: String,
    pub ast: Rc<BramaAstType>
}

impl ExecuteLineResult {
    pub fn new(output: String, ast: Rc<BramaAstType>) -> Self {
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


#[derive(Default)]
pub struct Session {
    text: String,
    text_parts: Vec<String>,
    language: String,
    position: Cell<usize>,
    
    pub asts: Vec<Rc<BramaAstType>>,
    pub variables: Vec<Rc<VariableInfo>>,
    
    pub tokens: Vec<Rc<TokenType>>,
    pub token_infos: Vec<Rc<TokenInfo>>,
    pub ui_tokens: UiTokenCollection
}

impl Session {
    pub fn new() -> Session {
        Session {
            text: String::new(),
            text_parts: Vec::new(),
            language: String::new(),
            asts: Vec::new(),
            variables: Vec::new(),
            tokens: Vec::new(),
            token_infos: Vec::new(),
            ui_tokens: UiTokenCollection::default(),
            position: Cell::default()
        }
    }
    
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        
        self.text_parts = match Regex::new(r"\r\n|\n") {
            Ok(re) => re.split(&self.text).map(|item| item.to_string()).collect::<Vec<_>>(),
            _ => self.text.lines().map(|item| item.to_string()).collect::<Vec<_>>()
        };
    }
    
    pub fn set_text_parts(&mut self, parts: Vec<String>) {
        self.text_parts = parts;
    }
    
    pub fn set_language(&mut self, language: String) {
        self.language = language;
    }
    
    pub fn current(&self) -> &'_ String { 
        &self.text_parts[self.position.get()]
    }
    
    pub fn has_value(&self) -> bool { 
        self.text_parts.len() > self.position.get()
    }
    
    pub fn next(&self) -> Option<&'_ String> {
        match self.text_parts.len() > self.position.get() + 1 {
            true => {
                let current = Some(self.current());
                self.position.set(self.position.get() + 1);
                current
            }
            false => None
        }
    }
    
    pub fn add_ast(&mut self, ast: Rc<BramaAstType>) {
        self.asts.push(ast);
    }
    
    pub fn add_variable(&mut self, variable_info: Rc<VariableInfo>) {
        self.variables.push(variable_info);
    }
    
    pub fn get_tokens(&self) -> &'_ Vec<Rc<TokenType>> {
        &self.tokens
    }
    
    pub fn get_ui_tokens(&self) -> &'_ UiTokenCollection {
        &self.ui_tokens
    }
    
    pub fn get_token_infos(&self) -> &'_ Vec<Rc<TokenInfo>> {
        &self.token_infos
    }
    
    pub fn get_variables(&self) -> &'_ Vec<Rc<VariableInfo>> {
        &self.variables
    }
    
    pub fn get_language(&self) -> String {
        self.language.to_string()
    }
    
    pub fn get_variable(&self, index: usize) -> Rc<VariableInfo> {
        self.variables[index].clone()
    }
    
    pub fn cleanup_token_infos(&mut self) {
        self.token_infos.retain(|x| (*x).token_type.borrow().deref().is_some());
        self.token_infos.sort_by(|a, b| (*a).start.partial_cmp(&b.start).unwrap());
        //self.ui_tokens.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());
    }
    
    pub fn cleanup(&mut self) {
        self.token_infos.clear();
        self.tokens.clear();
        self.asts.clear();
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

    pub fn token_generator(&self, session: &RefCell<Session>) {
        let mut tokens = Vec::new();
        let mut session_mut = session.borrow_mut();
        for token_location in session_mut.token_infos.iter() {
            if token_location.status == TokenInfoStatus::Active {
                if let Some(token_type) = &token_location.token_type.borrow().deref() {
                    tokens.push(token_type.clone());
                }
            }
        }
        
        for token in tokens {
            session_mut.tokens.push(Rc::new(token));
        }
    }
    
    pub fn format_result<T: Borrow<str>>(&self, language: T, result: Rc<BramaAstType>) -> String {
        match self.config.format.get(language.borrow()) {
            Some(formats) => format_result(&self.config, formats, result),
            _ => "".to_string()
        }
    }

    fn missing_token_adder(&self, session: &RefCell<Session>) {
        let mut index = 0;
        let mut session_mut = session.borrow_mut();
        let tokens = &mut session_mut.tokens;
        
        if tokens.is_empty() {
            return;
        }
        
        for (token_index, token) in tokens.iter().enumerate() {
            match &**token {
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

        if let TokenType::Operator(_) = &*tokens[index] {
            tokens.insert(index, Rc::new(TokenType::Number(0.0)));
        }

        while index < tokens.len() {
            match &*tokens[index] {
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
            match token.token_type.borrow().deref() {
                Some(TokenType::Operator('=')) => {
                    index = token_index as usize + 1;
                    break;
                },
                _ => ()
            };
        }

        while index < session_mut.tokens.len() {
            match &*session_mut.tokens[index] {
                TokenType::Text(_) => {
                    session_mut.tokens.remove(index);
                },
                _ => index += 1
            };
        }
    }

    pub fn execute_text(&self, session: &RefCell<Session>) -> ExecutionLine {
        log::debug!("> {}", session.borrow().current());
        if session.borrow().current().is_empty() {
            session.borrow_mut().add_ast(Rc::new(BramaAstType::None));
            return None;
        }

        let mut tokinize = Tokinizer::new(&self.config, session);
        tokinize.language_based_tokinize();
        log::debug!(" > language_based_tokinize");
        tokinize.tokinize_with_regex();
        log::debug!(" > tokinize_with_regex");
        tokinize.apply_aliases();
        log::debug!(" > apply_aliases");
        TokenType::update_for_variable(&mut tokinize);
        log::debug!(" > update_for_variable");
        tokinize.apply_rules();
        log::debug!(" > apply_rules");
        self.token_generator(session);
        log::debug!(" > token_generator");
        self.token_cleaner(session);
        log::debug!(" > token_cleaner");

        self.missing_token_adder(session);
        log::debug!(" > missing_token_adder");

        if session.borrow().token_infos.is_empty() {
            session.borrow_mut().add_ast(Rc::new(BramaAstType::None));
            return None;
        }

        let mut syntax = SyntaxParser::new(session);

        log::debug!(" > parse starting");

        let execution_result = match syntax.parse() {
            Ok(ast) => {
                log::debug!(" > parse Ok {:?}", ast);
                let ast_rc = Rc::new(ast);
                session.borrow_mut().add_ast(ast_rc.clone());

                match Interpreter::execute(&self.config, ast_rc.clone(), session) {
                    Ok(ast) => Ok(ExecuteLineResult::new(self.format_result(session.borrow().get_language(), ast.clone()), ast.clone())),
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

    pub fn execute<Tlan: Borrow<str>, Tdata: Borrow<str>>(&self, language: Tlan, data: Tdata) -> ExecuteResult {
        let mut results     = ExecuteResult::default();
        let mut session         = Session::new();
        
        session.set_text(data.borrow().to_string());
        session.set_language(language.borrow().to_string());
        
        let session = RefCell::new(session);
        
        if session.borrow().has_value() {
            results.status = true;
            loop {
                let line_result = self.execute_text(&session);
                results.lines.push(line_result);
                session.borrow_mut().cleanup();
                if session.borrow().next().is_none() { break; }
            }
        }

        results
    }
}