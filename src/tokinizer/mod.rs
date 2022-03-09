/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

mod number;
mod operator;
mod text;
mod whitespace;
mod field;
mod percent;
mod atom;
mod time;
mod money;
mod comment;
mod month;
mod timezone;

use core::cell::RefCell;

use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use alloc::string::ToString;
use crate::app::Session;
use crate::config::SmartCalcConfig;
use crate::token::ui_token::UiTokenType;
use crate::{token::ui_token::UiTokenCollection, types::*};
use crate::tokinizer::time::time_regex_parser;
use crate::tokinizer::number::number_regex_parser;
use crate::tokinizer::percent::percent_regex_parser;
use crate::tokinizer::money::money_regex_parser;
use crate::tokinizer::text::text_regex_parser;
use crate::tokinizer::field::field_regex_parser;
use crate::tokinizer::atom::{atom_regex_parser, get_atom};
use crate::tokinizer::whitespace::whitespace_regex_parser;
use crate::tokinizer::comment::comment_regex_parser;
use crate::tokinizer::timezone::timezone_regex_parser;

use operator::operator_regex_parser;
use regex::{Match, Regex};
use lazy_static::*;
use alloc::collections::btree_map::BTreeMap;
use core::ops::Deref;

use self::month::month_parser;

lazy_static! {
    pub static ref TOKEN_REGEX_PARSER: Vec<(&'static str, RegexParser)> = {
        let m = vec![
        ("comment",    comment_regex_parser    as RegexParser),
        ("field",      field_regex_parser      as RegexParser),
        ("money",      money_regex_parser      as RegexParser),
        ("atom",       atom_regex_parser       as RegexParser),
        ("percent",    percent_regex_parser    as RegexParser),
        ("timezone",   timezone_regex_parser   as RegexParser),
        ("time",       time_regex_parser       as RegexParser),
        ("number",     number_regex_parser     as RegexParser),
        ("text",       text_regex_parser       as RegexParser),
        ("whitespace", whitespace_regex_parser as RegexParser),
        ("operator",   operator_regex_parser   as RegexParser)];
        m
    };
}

lazy_static! {
    pub static ref LANGUAGE_BASED_TOKEN_PARSER: Vec<Parser> = {
        let m = vec![month_parser as Parser];
        m
    };
}


pub type RegexParser = fn(config: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]);
pub type Parser      = fn(config: &SmartCalcConfig, tokinizer: &mut Tokinizer, data: &str);

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
    pub status: TokenInfoStatus
}

#[derive(Debug)]
#[derive(Clone)]
pub struct DynamicType_ {
    pub start: usize,
    pub end: usize,
    pub token_type: RefCell<Option<TokenType>>,
    pub original_text: String,
    pub status: TokenInfoStatus
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

    pub fn token_infos(config: &'a SmartCalcConfig, session: &'a RefCell<Session>) -> Vec<Arc<TokenInfo>> {
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

        tokinizer.tokinize_with_regex();
        tokinizer.apply_aliases();
        tokinizer.session.borrow().token_infos.clone()
    }
    
    pub fn language_based_tokinize(&mut self) {
        let lowercase_data = self.data.to_lowercase();
        for func in LANGUAGE_BASED_TOKEN_PARSER.iter() {
            func(self.config, self, &lowercase_data);
        }

        self.session.borrow_mut().cleanup_token_infos();
    }

    pub fn tokinize_with_regex(&mut self) {
        /* Token parser with regex */
        for (key, func) in TOKEN_REGEX_PARSER.iter() {
            if let Some(items) = self.config.token_parse_regex.get(&key.to_string()) { 
                func(self.config, self, items) 
            }
        }
        
        self.session.borrow_mut().cleanup_token_infos();
    }

    pub fn apply_aliases(&mut self) {
        let session_mut = self.session.borrow_mut();
        let language = session_mut.get_language();

        for token in session_mut.token_infos.iter() {
            for (re, data) in self.config.alias_regex.iter() {
                if re.is_match(&token.original_text.to_lowercase()) {
                    let new_values = match self.config.token_parse_regex.get("atom") {
                        Some(items) => get_atom(self.config, data, items),
                        _ => Vec::new()
                    };

                    match new_values.len() {
                        1 => {
                            if let Some(token_type) = &new_values[0].2 {
                                *token.token_type.borrow_mut() = Some(token_type.clone());
                                break;
                            }
                        },
                        0 => {
                            *token.token_type.borrow_mut() = Some(TokenType::Text(data.to_string()));
                            break;
                        },
                        _ => log::warn!("{} has multiple atoms. It is not allowed", data)
                    };
                }
            }
        }

        for token in session_mut.token_infos.iter() {
            for (re, data) in self.config.language_alias_regex.get(&language).unwrap().iter() {
                if re.is_match(&token.original_text.to_lowercase()) {
                    let new_values = match self.config.token_parse_regex.get("atom") {
                        Some(items) => get_atom(self.config, data, items),
                        _ => Vec::new()
                    };

                    match new_values.len() {
                        1 => {
                            if let Some(token_type) = &new_values[0].2 {
                                *token.token_type.borrow_mut() = Some(token_type.clone());
                                break;
                            }
                        },
                        0 => {
                            *token.token_type.borrow_mut() = Some(TokenType::Text(data.to_string()));
                            break;
                        },
                        _ => log::warn!("{} has multiple atoms. It is not allowed", data)
                    };
                }
            }
        }
    }

    pub fn apply_rules(&mut self) {
        let mut session_mut = self.session.borrow_mut();
        let language = session_mut.get_language();
        
        if let Some(language) = self.config.rule.get(&language) {

            let mut execute_rules = true;
            while execute_rules {
                execute_rules = false;

                for (function_name, function, tokens_list) in language.iter() {
                    if cfg!(feature="debug-rules") {
                        log::debug!("# Checking for '{}'", function_name);
                    }

                    for rule_tokens in tokens_list {

                        let total_rule_token       = rule_tokens.len();
                        let mut rule_token_index   = 0;
                        let mut target_token_index = 0;
                        let mut start_token_index  = 0;
                        let mut fields             = BTreeMap::new();
                        let mut determinants = Vec::new();

                        while let Some(token) = session_mut.token_infos.get(target_token_index) {
                            target_token_index += 1;
                            if token.status == TokenInfoStatus::Removed {
                                continue;
                            }

                            match &token.token_type.borrow().deref() {
                                Some(token_type) => {

                                    if let TokenType::Variable(variable) = &token_type {
                                        let is_same = TokenType::variable_compare(&rule_tokens[rule_token_index], variable.data.borrow().clone());
                                        if is_same {
                                            match TokenType::get_field_name(&rule_tokens[rule_token_index]) {
                                                Some(field_name) => { fields.insert(field_name.to_string(), token.clone()); },
                                                None => { determinants.push((token.start, token.end)); }
                                            };

                                            rule_token_index   += 1;
                                        } else {
                                            rule_token_index    = 0;
                                            start_token_index   = target_token_index;
                                        }
                                    }
                                    else if token == &rule_tokens[rule_token_index] {
                                        match TokenType::get_field_name(&rule_tokens[rule_token_index]) {
                                            Some(field_name) => {fields.insert(field_name.to_string(), token.clone()); },
                                            None => { determinants.push((token.start, token.end)); }
                                        };

                                        if cfg!(feature="debug-rules") {
                                            log::debug!("Ok, {:?} == {:?}", token.token_type, &rule_tokens[rule_token_index].token_type);
                                        }

                                        rule_token_index   += 1;
                                    }
                                    else {
                                        if cfg!(feature="debug-rules") {
                                            log::debug!("No, {:?} == {:?}", token.token_type, &rule_tokens[rule_token_index].token_type);
                                        }
                                        rule_token_index    = 0;
                                        start_token_index   = target_token_index;
                                    }

                                    if total_rule_token == rule_token_index { break; }
                                },
                                _ => ()
                            }
                        }

                        if total_rule_token == rule_token_index {
                            if cfg!(feature="debug-rules") {
                                log::debug!(" --------- {} executing", function_name);
                            }

                            match function(self.config, self, &fields) {
                                Ok(token) => {
                                    if cfg!(feature="debug-rules") {
                                        log::debug!("Rule function success with new token: {:?}", token);
                                    }

                                    let text_start_position = session_mut.token_infos[start_token_index].start;
                                    let text_end_position   = session_mut.token_infos[target_token_index - 1].end;
                                    execute_rules = true;

                                    for index in start_token_index..target_token_index {
                                        (*Arc::make_mut(&mut session_mut.token_infos[index])).status = TokenInfoStatus::Removed;
                                    }

                                    session_mut.token_infos.insert(start_token_index, Arc::new(TokenInfo {
                                        start: text_start_position,
                                        end: text_end_position,
                                        token_type: RefCell::new(Some(token)),
                                        original_text: "".to_string(),
                                        status: TokenInfoStatus::Active
                                    }));
                                    break;
                                },
                                Err(error) => log::info!("Rule execution error, {}", error)
                            }
                        }
                    }
                }
            }
        }

        if cfg!(feature="debug-rules") {
            log::debug!("Updated token_infos: {:?}", session_mut.token_infos);
        }
    }

    pub fn apply_types(&mut self) {
        let mut session_mut = self.session.borrow_mut();
        
        let mut execute_rules = true;
        while execute_rules {
            execute_rules = false;

            for (type_name, type_items) in self.config.types.iter() {
                for (_, dynamic_type) in type_items.iter() {
                    for rule_tokens in dynamic_type.parse.iter() {
                        let total_rule_token       = rule_tokens.len();
                        let mut rule_token_index   = 0;
                        let mut target_token_index = 0;
                        let mut start_token_index  = 0;
                        let mut fields             = BTreeMap::new();
                        let mut determinants = Vec::new();

                        while let Some(token) = session_mut.token_infos.get(target_token_index) {
                            target_token_index += 1;
                            if token.status == TokenInfoStatus::Removed {
                                continue;
                            }

                            match &token.token_type.borrow().deref() {
                                Some(token_type) => {

                                    if let TokenType::Variable(variable) = &token_type {
                                        let is_same = TokenType::variable_compare(&rule_tokens[rule_token_index], variable.data.borrow().clone());
                                        if is_same {
                                            match TokenType::get_field_name(&rule_tokens[rule_token_index]) {
                                                Some(field_name) => { fields.insert(field_name.to_string(), token.clone()); },
                                                None => { determinants.push((token.start, token.end)); }
                                            };

                                            rule_token_index   += 1;
                                        } else {
                                            rule_token_index    = 0;
                                            start_token_index   = target_token_index;
                                        }
                                    }
                                    else if token == &rule_tokens[rule_token_index] {
                                        match TokenType::get_field_name(&rule_tokens[rule_token_index]) {
                                            Some(field_name) => { fields.insert(field_name.to_string(), token.clone()); },
                                            None => { determinants.push((token.start, token.end)); }
                                        };

                                        if cfg!(feature="debug-rules") {
                                            log::debug!("Ok, {:?} == {:?}", token.token_type, &rule_tokens[rule_token_index].token_type);
                                        }

                                        rule_token_index   += 1;
                                    }
                                    else {
                                        if cfg!(feature="debug-rules") {
                                            log::debug!("No, {:?} == {:?}", token.token_type, &rule_tokens[rule_token_index].token_type);
                                        }
                                        rule_token_index    = 0;
                                        start_token_index   = target_token_index;
                                    }

                                    if total_rule_token == rule_token_index { break; }
                                },
                                _ => ()
                            }
                        }

                        if total_rule_token == rule_token_index {                            
                            if cfg!(feature="debug-rules") {
                                log::debug!(" --------- {} found", type_name);
                            }
                            
                            let text_start_position = session_mut.token_infos[start_token_index].start;
                            let text_end_position   = session_mut.token_infos[target_token_index - 1].end;
                            execute_rules = true;

                            for index in start_token_index..target_token_index {
                                (*Arc::make_mut(&mut session_mut.token_infos[index])).status = TokenInfoStatus::Removed;
                            }
                            
                            let value = crate::worker::tools::get_number("value", &fields).unwrap();
                            for (start_position, end_position) in determinants {
                                self.ui_tokens.update_tokens(start_position, end_position, UiTokenType::Symbol2)
                            }

                            session_mut.token_infos.insert(start_token_index, Arc::new(TokenInfo {
                                start: text_start_position,
                                end: text_end_position,
                                token_type: RefCell::new(Some(TokenType::DynamicType(value, dynamic_type.clone()))),
                                original_text: "".to_string(),
                                status: TokenInfoStatus::Active
                            }));
                            break;
                        }
                    }
                }
            }
        }

        if cfg!(feature="debug-rules") {
            log::debug!("Updated token_infos: {:?}", session_mut.token_infos);
        }
    }

    pub fn add_token_location(&mut self, start: usize, end: usize, token_type: Option<TokenType>, text: String) -> bool {
        let mut session_mut = self.session.borrow_mut();
        for item in session_mut.token_infos.iter() {
            if (item.start <= start && item.end > start) || (item.start < end && item.end >= end) {
                return false
            }
        }

        session_mut.token_infos.push(Arc::new(TokenInfo {
            start,
            end,
            token_type: RefCell::new(token_type),
            original_text: text,
            status: TokenInfoStatus::Active
        }));
        true
    }

    pub fn add_token<'t>(&mut self, capture: &Option<Match<'t>>, token_type: Option<TokenType>) -> bool {
        match capture {
            Some(content) => self.add_token_location(content.start(), content.end(), token_type, content.as_str().to_string()),
            None => false
        }
    }

    pub fn is_end(&mut self) -> bool {
        self.total <= self.indexer
    }

    pub fn get_char(&mut self) -> char {
        return match self.iter.get(self.indexer) {
            Some(&c) => c,
            None => '\0'
        };
    }

    pub fn get_next_char(&mut self) -> char {
        return match self.iter.get(self.indexer + 1) {
            Some(&c) => c,
            None => '\0'
        };
    }

    pub fn get_indexer(&self) -> TokinizerBackup {
        TokinizerBackup {
            indexer: self.indexer,
            index: self.index,
            column: self.column
        }
    }

    pub fn set_indexer(&mut self, backup: TokinizerBackup) {
        self.indexer = backup.indexer;
        self.index   = backup.index;
        self.column  = backup.column;
    }

    pub fn increase_index(&mut self) {
        self.index   += self.get_char().len_utf8() as u16;
        self.indexer += 1;
        self.column  += 1;
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
    use crate::app::Session;
    use core::cell::RefCell;
    use alloc::rc::Rc;
    use alloc::string::String;
    use alloc::string::ToString;
    use alloc::sync::Arc;
    use alloc::vec;
    use alloc::vec::Vec;
    use crate::config::SmartCalcConfig;
    use crate::tokinizer::TokenInfo;

    pub fn execute(data: String) -> Vec<Arc<TokenInfo>> {
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
        session.borrow_mut().set_text_parts(vec![data]);
    
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