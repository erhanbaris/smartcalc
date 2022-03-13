use core::cell::Cell;
use alloc::string::{String, ToString};
use core::ops::Deref;

use alloc::{rc::Rc, vec::Vec};
use regex::Regex;

use crate::tokinizer::TokenInfo;
use crate::types::TokenType;
use crate::{variable::VariableInfo, SmartCalcAstType};




#[derive(Default)]
pub struct Session {
    text: String,
    text_parts: Vec<String>,
    language: String,
    position: Cell<usize>,

    pub(crate) asts: Vec<Rc<SmartCalcAstType>>,
    pub(crate) variables: Vec<Rc<VariableInfo>>,

    pub(crate) tokens: Vec<Rc<TokenType>>,
    pub(crate) token_infos: Vec<Rc<TokenInfo>>
}

impl Session {
    /// Create an empty session.
    ///
    /// In order to be executed, a session must have a language and some text.
    pub fn new() -> Session {
        Session {
            text: String::new(),
            text_parts: Vec::new(),
            language: String::new(),
            asts: Vec::new(),
            variables: Vec::new(),
            tokens: Vec::new(),
            token_infos: Vec::new(),
            position: Cell::default()
        }
    }

    /// Set the text to be executed.
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        
        self.text_parts = match Regex::new(r"\r\n|\n") {
            Ok(re) => re.split(&self.text).map(|item| item.to_string()).collect::<Vec<_>>(),
            _ => self.text.lines().map(|item| item.to_string()).collect::<Vec<_>>()
        };
    }

    /// Set the language used to interpret input.
    pub fn set_language(&mut self, language: String) {
        self.language = language;
    }
    
    pub(crate) fn current(&self) -> &'_ String { 
        &self.text_parts[self.position.get()]
    }
    
    pub(crate) fn has_value(&self) -> bool { 
        self.text_parts.len() > self.position.get()
    }
    
    pub(crate) fn next(&self) -> Option<&'_ String> {
        match self.text_parts.len() > self.position.get() + 1 {
            true => {
                let current = Some(self.current());
                self.position.set(self.position.get() + 1);
                current
            }
            false => None
        }
    }
    
    pub(crate) fn add_ast(&mut self, ast: Rc<SmartCalcAstType>) {
        self.asts.push(ast);
    }
    
    pub(crate) fn add_variable(&mut self, variable_info: Rc<VariableInfo>) {
        self.variables.push(variable_info);
    }
    
    /// Returns the language configured for this session.
    pub fn get_language(&self) -> String {
        self.language.to_string()
    }
    
    pub(crate) fn cleanup_token_infos(&mut self) {
        self.token_infos.retain(|x| (*x).token_type.borrow().deref().is_some());
        self.token_infos.sort_by(|a, b| (*a).start.partial_cmp(&b.start).unwrap());
        //self.ui_tokens.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());
    }
    
    pub(crate) fn cleanup(&mut self) {
        self.token_infos.clear();
        self.tokens.clear();
        self.asts.clear();
    }
}
