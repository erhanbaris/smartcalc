/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

mod regex_tokinizer;
mod alias_tokinizer;
mod rule_tokimizer;
mod dynamic_type_tokimizer;
mod tools;

pub use self::regex_tokinizer::regex_tokinizer;
pub use self::regex_tokinizer::language_tokinizer;
pub use self::alias_tokinizer::alias_tokinizer;
pub use self::dynamic_type_tokimizer::dynamic_type_tokimizer;
pub use self::tools::*;
pub use self::rule_tokimizer::{rule_tokinizer, RuleItemList, RULE_FUNCTIONS};

use core::cell::Cell;
use core::cell::RefCell;

use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::string::ToString;

use regex::Match;

use crate::session::Session;
use crate::config::SmartCalcConfig;
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
    pub session: &'a RefCell<Session>,
    pub language: String
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
    pub fn new(config: &'a SmartCalcConfig, session: &'a RefCell<Session>) -> Tokinizer<'a> {
        Tokinizer {
            column: 0,
            iter: session.borrow().current().chars().collect(),
            data: session.borrow().current().to_string(),
            index: 0,
            indexer: 0,
            total: session.borrow().current().chars().count(),
            ui_tokens: UiTokenCollection::new(session.borrow().current()),
            config,
            session,
            language: session.borrow().get_language()
        }
    }

    pub fn token_infos(config: &'a SmartCalcConfig, session: &'a RefCell<Session>) -> Vec<Rc<TokenInfo>> {
        let mut tokinizer = Tokinizer {
            column: 0,
            iter: session.borrow().current().chars().collect(),
            data: session.borrow().current().to_string(),
            index: 0,
            indexer: 0,
            total: session.borrow().current().chars().count(),
            ui_tokens: UiTokenCollection::new(session.borrow().current()),
            config,
            session,
            language: session.borrow().get_language()
        };

        regex_tokinizer(&mut tokinizer);
        alias_tokinizer(&mut tokinizer);
        tokinizer.session.borrow().token_infos.clone()
    }

    pub fn add_token_location(&mut self, start: usize, end: usize, token_type: Option<TokenType>, text: String) -> bool {
        let mut session_mut = self.session.borrow_mut();
        for item in session_mut.token_infos.iter() {
            if (item.start <= start && item.end > start) || (item.start < end && item.end >= end) {
                return false
            }
        }

        session_mut.token_infos.push(Rc::new(TokenInfo {
            start,
            end,
            token_type: RefCell::new(token_type),
            original_text: text,
            status: Cell::new(TokenInfoStatus::Active)
        }));
        true
    }

    pub fn add_token<'t>(&mut self, capture: &Option<Match<'t>>, token_type: Option<TokenType>) -> bool {
        match capture {
            Some(content) => self.add_token_location(content.start(), content.end(), token_type, content.as_str().to_string()),
            None => false
        }
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
    use core::cell::RefCell;
    use alloc::rc::Rc;
    use alloc::string::String;
    use alloc::string::ToString;
    use alloc::vec::Vec;
    use crate::config::SmartCalcConfig;
    use crate::tokinizer::TokenInfo;

    pub fn execute(data: String) -> Vec<Rc<TokenInfo>> {
        use crate::app::SmartCalc;
        let calculator = SmartCalc::default();
        
        
        let result = calculator.execute("en", data);
        assert_eq!(result.status, true);
        assert_eq!(result.lines.len(), 1);
        
        result.lines[0].as_ref().unwrap().calculated_tokens.clone()
    }

    pub fn get_executed_raw_tokens(data: String) -> Vec<Rc<TokenType>> {
        use crate::app::SmartCalc;
        let calculator = SmartCalc::default();
        
        
        let result = calculator.execute("en", data);
        assert_eq!(result.status, true);
        assert_eq!(result.lines.len(), 1);
        
        result.lines[0].as_ref().unwrap().raw_tokens.clone()
    }

    pub fn setup_tokinizer<'a>(data: String, session: &'a RefCell<Session>, config: &'a SmartCalcConfig) -> Tokinizer<'a> {
        session.borrow_mut().set_language("en".to_string());
        session.borrow_mut().set_text(data);
    
        let tokinizer = Tokinizer::new(&config, &session);
        tokinizer
    }

    #[cfg(test)]
    #[test]
    fn alias_test() {
        use core::ops::Deref;
        use crate::{app::SmartCalc, types::NumberType};

        let smartcalc = SmartCalc::default();
        let result = smartcalc.execute("en", "add 1024 percent");
        assert!(result.status);
        assert!(result.lines.len() == 1);

        assert!(result.lines[0].is_some());
        
        let tokens = result.lines[0].as_ref().unwrap().calculated_tokens.to_vec();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].start, 0);
        assert_eq!(tokens[0].end, 3);
        assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Operator('+')));

        assert_eq!(tokens[1].start, 4);
        assert_eq!(tokens[1].end, 8);
        assert_eq!(tokens[1].token_type.borrow().deref(), &Some(TokenType::Number(1024.0, NumberType::Decimal)));

        assert_eq!(tokens[2].start, 9);
        assert_eq!(tokens[2].end, 16);
        //assert_eq!(tokens[2].token_type.borrow().deref(), &Some(TokenType::Operator('%')));
    }
}