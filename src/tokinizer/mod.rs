/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

mod regex_tokinizer;
mod alias_tokinizer;
mod rule_tokinizer;
mod dynamic_type_tokinizer;
mod api_tokinizer;
mod tools;

pub use self::regex_tokinizer::regex_tokinizer;
pub use self::regex_tokinizer::language_tokinizer;
pub use self::alias_tokinizer::alias_tokinizer;
pub use self::dynamic_type_tokinizer::dynamic_type_tokinizer;
pub use self::api_tokinizer::api_tokinizer;
pub use self::tools::*;
pub use self::rule_tokinizer::{rule_tokinizer, RuleItemList, RULE_FUNCTIONS};

use core::cell::Cell;
use core::ops::Deref;
use core::cell::RefCell;

use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::string::ToString;

use regex::Match;

use crate::UiTokenType;
use crate::session::Session;
use crate::config::SmartCalcConfig;
use crate::variable::update_token_variables;
use crate::{token::ui_token::UiTokenCollection, types::*};


pub struct Tokinizer<'a> {
    pub column: u16,
    pub iter: Vec<char>,
    pub data: String,
    pub index: u16,
    pub indexer: usize,
    pub total: usize,
    pub ui_tokens: UiTokenCollection,
    pub config: &'a SmartCalcConfig,
    pub session: &'a Session,
    pub language: String,
    pub token_infos: Vec<Rc<TokenInfo>>,
    pub tokens: Vec<Rc<TokenType>>,
}

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum TokenInfoStatus {
    Active,
    Removed
}

#[derive(Debug)]
#[derive(Clone)]
pub struct TokenInfo {
    pub start: usize,
    pub end: usize,
    pub token_type: RefCell<Option<TokenType>>,
    pub original_text: String,
    pub status: Cell<TokenInfoStatus>
}

unsafe impl Send for TokenInfo {}
unsafe impl Sync for TokenInfo {}

impl<'a> Tokinizer<'a> {
    pub fn new(config: &'a SmartCalcConfig, session: &'a Session) -> Tokinizer<'a> {
        Tokinizer {
            column: 0,
            iter: session.current_line().chars().collect(),
            data: session.current_line().to_string(),
            index: 0,
            indexer: 0,
            total: session.current_line().chars().count(),
            ui_tokens: UiTokenCollection::new(session.current_line()),
            config,
            session,
            language: session.get_language(),
            token_infos: Vec::new(),
            tokens: Vec::new()
        }
    }

    pub fn token_infos(config: &'a SmartCalcConfig, session: &'a Session) -> Vec<Rc<TokenInfo>> {
        let mut tokinizer = Tokinizer {
            column: 0,
            iter: session.current_line().chars().collect(),
            data: session.current_line().to_string(),
            index: 0,
            indexer: 0,
            total: session.current_line().chars().count(),
            ui_tokens: UiTokenCollection::new(session.current_line()),
            config,
            session,
            language: session.get_language(),
            token_infos: Vec::new(),
            tokens: Vec::new()
        };

        language_tokinizer(&mut tokinizer);
        regex_tokinizer(&mut tokinizer);
        alias_tokinizer(&mut tokinizer);
        tokinizer.token_infos
    }

    pub fn tokinize(&mut self) -> bool {
        language_tokinizer(self);
        log::debug!(" > language_tokinizer");
        regex_tokinizer(self);
        log::debug!(" > regex_tokinizer");
        alias_tokinizer(self);
        log::debug!(" > alias_tokinizer");
        update_token_variables(self);
        log::debug!(" > update_token_variables");
        dynamic_type_tokinizer(self);
        log::debug!(" > dynamic_type_tokinizer");
        rule_tokinizer(self);
        log::debug!(" > rule_tokinizer");
        api_tokinizer(self);
        log::debug!(" > api_tokinizer");

        /* Post process operations */
        self.token_generator();
        log::debug!(" > token_generator");        
        self.token_cleaner();
        log::debug!(" > token_cleaner");
        self.missing_token_adder();
        log::debug!(" > missing_token_adder");

        !self.token_infos.is_empty()
    }

    pub fn add_token_from_match<'t>(&mut self, capture: &Option<Match<'t>>, token_type: Option<TokenType>) -> bool {
        match capture {
            Some(content) => self.add_token_location(content.start(), content.end(), token_type, content.as_str().to_string()),
            None => false
        }
    }

    pub fn add_uitoken_from_match(&mut self, capture: Option<Match<'_>>, token_type: UiTokenType) {
        self.ui_tokens.add_from_regex_match(capture, token_type)
    }

    pub fn add_token_location(&mut self, start: usize, end: usize, token_type: Option<TokenType>, text: String) -> bool {
        for item in self.token_infos.iter() {
            if (item.start <= start && item.end > start) || (item.start < end && item.end >= end) {
                return false
            }
        }

        self.token_infos.push(Rc::new(TokenInfo {
            start,
            end,
            token_type: RefCell::new(token_type),
            original_text: text,
            status: Cell::new(TokenInfoStatus::Active)
        }));
        true
    }

    pub fn token_generator(&mut self) {
        let mut tokens = Vec::new();
        for token_location in self.token_infos.iter() {
            if token_location.status.get() == TokenInfoStatus::Active {
                if let Some(token_type) = &token_location.token_type.borrow().deref() {
                    tokens.push(token_type.clone());
                }
            }
        }
        
        for token in tokens {
            self.tokens.push(Rc::new(token));
        }
    }

    pub fn token_cleaner(&mut self) {
        let mut index = 0;
        for (token_index, token) in self.token_infos.iter().enumerate() {
            if let Some(TokenType::Operator('=')) = token.token_type.borrow().deref() {
                index = token_index as usize + 1;
                break;
            };
        }

        while index < self.tokens.len() {
            match self.tokens[index].deref() {
                TokenType::Text(_) => {
                    self.tokens.remove(index);
                },
                _ => index += 1
            };
        }
    }

    fn missing_token_adder(&mut self) {
        let mut index = 0;
        
        if self.tokens.is_empty() {
            return;
        }
        
        for (token_index, token) in self.tokens.iter().enumerate() {
            match token.deref() {
                TokenType::Operator('=') | 
                TokenType::Operator('(')=> {
                    index = token_index as usize + 1;
                    break;
                },
                _ => ()
            };
        }

        if index + 1 >= self.tokens.len() {
            return;
        }

        if let TokenType::Operator('(') = self.tokens[index].deref() {
            index += 1;
        }

        let mut operator_required = false;

        if let TokenType::Operator(_) = self.tokens[index].deref() {
            self.tokens.insert(index, Rc::new(TokenType::Number(0.0, NumberType::Decimal)));
        }

        while index < self.tokens.len() {
            match self.tokens[index].deref() {
                TokenType::Operator(_) => operator_required = false,
                _ => {
                    if operator_required {
                        log::debug!("Added missing operator between two token");
                        self.tokens.insert(index, Rc::new(TokenType::Operator('+')));
                        index += 1;
                    }
                    operator_required = true;
                }
            };
            
            index += 1;
        }
    }
    
    pub fn cleanup_token_infos(&mut self) {
        self.token_infos.retain(|x| (*x).token_type.borrow().deref().is_some());
        self.token_infos.sort_by(|a, b| (*a).start.partial_cmp(&b.start).unwrap());
    }
}

#[cfg(test)]
extern crate alloc;

#[macro_export]
macro_rules! setup_tokinizer {
    ($data:expr) => {        
        let mut session = Session::new();
        let config = SmartCalcConfig::default();
        session.set_language("en".to_string());
        session.set_text($data);
        
        let session = RefCell::new(session);
        
        let mut tokinizer_mut = Tokinizer::new(&config, &session);
        tokinizer_mut
    };
}

#[cfg(test)]
pub mod test {
    use crate::tokinizer::Tokinizer;
    use crate::types::TokenType;
    use crate::session::Session;
    use alloc::rc::Rc;
    use alloc::string::String;
    use alloc::string::ToString;
    use alloc::vec::Vec;
    use crate::config::SmartCalcConfig;
    use crate::tokinizer::TokenInfo;

    pub fn execute(data: String) -> Vec<Rc<TokenInfo>> {
        use crate::smartcalc::SmartCalc;
        let calculator = SmartCalc::default();
        
        
        let result = calculator.execute("en", data);
        assert_eq!(result.status, true);
        assert_eq!(result.lines.len(), 1);
        
        result.lines[0].as_ref().unwrap().calculated_tokens.clone()
    }

    pub fn get_executed_raw_tokens(data: String) -> Vec<Rc<TokenType>> {
        use crate::smartcalc::SmartCalc;
        let calculator = SmartCalc::default();
        
        
        let result = calculator.execute("en", data);
        assert_eq!(result.status, true);
        assert_eq!(result.lines.len(), 1);
        
        result.lines[0].as_ref().unwrap().raw_tokens.clone()
    }

    pub fn setup_tokinizer<'a>(data: String, session: &'a mut Session, config: &'a SmartCalcConfig) -> Tokinizer<'a> {
        session.set_language("en".to_string());
        session.set_text(data);
    
        let tokinizer = Tokinizer::new(&config, session);
        tokinizer
    }
}
