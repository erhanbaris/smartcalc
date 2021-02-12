mod number;
mod operator;
mod text;
mod whitespace;
mod field;
mod percent;
mod atom;
mod time;
mod money;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::string::ToString;
use mut_static::MutStatic;
use crate::types::*;
use self::number::number_parser;
use self::operator::operator_parser;
use self::text::text_parser;
use self::whitespace::whitespace_parser;
use self::field::field_parser;
use self::percent::percent_parser;
use self::atom::atom_parser;
use crate::tokinizer::time::time_regex_parser;
use crate::tokinizer::number::number_regex_parser;
use crate::tokinizer::percent::percent_regex_parser;
use crate::tokinizer::money::money_regex_parser;
use crate::tokinizer::text::text_regex_parser;
use crate::tokinizer::field::field_regex_parser;
use crate::tokinizer::atom::{atom_regex_parser, get_atom};
use crate::tokinizer::whitespace::whitespace_regex_parser;
use crate::constants::{TOKEN_PARSE_REGEXES, ALIAS_REGEXES, RULES};

use operator::operator_regex_parser;
use regex::{Regex};
use lazy_static::*;
use alloc::collections::btree_map::BTreeMap;

lazy_static! {
    pub static ref TOKEN_PARSER: Vec<TokenParser> = {
        let mut m = Vec::new();
        m.push(atom_parser as TokenParser);
        m.push(field_parser as TokenParser);
        m.push(percent_parser as TokenParser);
        m.push(whitespace_parser as TokenParser);
        m.push(text_parser as TokenParser);
        m.push(number_parser as TokenParser);
        m.push(operator_parser as TokenParser);
        m
    };
    pub static ref TOKEN_REGEX_PARSER: Vec<(&'static str, RegexParser)> = {
        let mut m = Vec::new();
        m.push(("field",      field_regex_parser      as RegexParser));
        m.push(("atom",       atom_regex_parser       as RegexParser));
        m.push(("percent",    percent_regex_parser    as RegexParser));
        m.push(("money",      money_regex_parser      as RegexParser));
        m.push(("time",       time_regex_parser       as RegexParser));
        m.push(("number",     number_regex_parser     as RegexParser));
        m.push(("text",       text_regex_parser       as RegexParser));
        m.push(("whitespace", whitespace_regex_parser as RegexParser));
        m.push(("operator",   operator_regex_parser   as RegexParser));
        m
    };
}


pub type TokenParser = fn(tokinizer: &mut Tokinizer) -> TokenParserResult;
pub type RegexParser = fn(tokinizer: &mut Tokinizer, group_item: &Vec<Regex>);

pub struct Tokinizer {
    pub line  : u16,
    pub column: u16,
    pub tokens: Vec<Token>,
    pub iter: Vec<char>,
    pub data: String,
    pub index: u16,
    pub indexer: usize,
    pub total: usize,
    pub token_locations: Vec<TokenLocation>
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum TokenLocationStatus {
    Active,
    Removed
}

#[derive(Debug)]
#[derive(Clone)]
pub struct TokenLocation {
    pub start: usize,
    pub end: usize,
    pub token_type: Option<TokenType>,
    pub original_text: String,
    pub status: TokenLocationStatus
}

unsafe impl Send for TokenLocation {}
unsafe impl Sync for TokenLocation {}

impl Tokinizer {
    pub fn new(data: &String) -> Tokinizer {
        Tokinizer {
            column: 0,
            line: 0,
            tokens: Vec::new(),
            iter: data.chars().collect(),
            data: data.to_string(),
            index: 0,
            indexer: 0,
            total: data.chars().count(),
            token_locations: Vec::new()
        }
    }

    pub fn token_locations(data: &String) -> Option<Vec<TokenLocation>> {
        let mut tokinizer = Tokinizer {
            column: 0,
            line: 0,
            tokens: Vec::new(),
            iter: data.chars().collect(),
            data: data.to_string(),
            index: 0,
            indexer: 0,
            total: data.chars().count(),
            token_locations: Vec::new()
        };

        tokinizer.tokinize_with_regex();
        tokinizer.apply_aliases();

        Some(tokinizer.token_locations)
    }


    pub fn tokinize(data: &String) -> TokinizeResult {
        let mut tokinizer = Tokinizer {
            column: 0,
            line: 0,
            tokens: Vec::new(),
            iter: data.chars().collect(),
            data: data.to_string(),
            index: 0,
            indexer: 0,
            total: data.chars().count(),
            token_locations: Vec::new()
        };

        tokinizer.tokinize_with_regex();
        tokinizer.apply_aliases();
        tokinizer.apply_rules();

        Ok(tokinizer.token_locations)
    }

    pub fn tokinize_with_regex(&mut self) {
        /* Token parser with regex */
        for (key, func) in TOKEN_REGEX_PARSER.iter() {
            match TOKEN_PARSE_REGEXES.read().unwrap().get(&key.to_string()) {
                Some(items) => func(self, items),
                _ => ()
            };
        }

        self.token_locations.retain(|x| x.token_type.is_some());
        self.token_locations.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());
    }

    pub fn apply_aliases(&mut self) {
        for token in &mut self.token_locations {
            for (re, data) in ALIAS_REGEXES.read().unwrap().iter() {
                if re.is_match(&token.original_text) {
                    let new_values = match TOKEN_PARSE_REGEXES.read().unwrap().get("atom") {
                        Some(items) => get_atom(data, items),
                        _ => Vec::new()
                    };

                    match new_values.len() {
                        1 => {
                            if let Some(token_type) = &new_values[0].2 {
                                token.token_type = Some(token_type.clone());
                                break;
                            }
                        },
                        0 => {
                            token.token_type = Some(TokenType::Text(data.to_string()));
                            break;
                        },
                        _ => () //println!("{} has multiple atoms. It is not allowed", data)
                    };
                }
            }
        }
    }

    pub fn apply_rules(&mut self) {
        if let Some(rules) = RULES.read().unwrap().get("en") {

            let mut execute_rules = true;
            while execute_rules {
                execute_rules = false;

                for (function, tokens_list) in rules.iter() {

                    for rule_tokens in tokens_list {

                        let total_rule_token       = rule_tokens.len();
                        let mut rule_token_index   = 0;
                        let mut target_token_index = 0;
                        let mut start_token_index  = 0;
                        let mut fields             = BTreeMap::new();

                        loop {
                            match self.token_locations.get(target_token_index) {
                                Some(token) => {

                                    match &token.token_type {
                                        Some(token_type) => {

                                            if let TokenType::Variable(variable) = &token_type {
                                                let is_same = Token::variable_compare(&rule_tokens[rule_token_index], variable.data.clone());
                                                if is_same {
                                                    match Token::get_field_name(&rule_tokens[rule_token_index]) {
                                                        Some(field_name) => fields.insert(field_name.to_string(), token),
                                                        None => None
                                                    };

                                                    rule_token_index   += 1;
                                                    target_token_index += 1;
                                                } else {
                                                    rule_token_index    = 0;
                                                    target_token_index += 1;
                                                    start_token_index   = target_token_index;
                                                }

                                                //println!("{:?}", variable.data.clone());
                                            }
                                            else if token == &rule_tokens[rule_token_index] {
                                                match Token::get_field_name(&rule_tokens[rule_token_index]) {
                                                    Some(field_name) => fields.insert(field_name.to_string(), token),
                                                    None => None
                                                };

                                                rule_token_index   += 1;
                                                target_token_index += 1;
                                            }
                                            else {
                                                rule_token_index    = 0;
                                                target_token_index += 1;
                                                start_token_index   = target_token_index;
                                            }

                                            if total_rule_token == rule_token_index { break; }
                                        },
                                        _ => {
                                            target_token_index += 1;
                                            continue;
                                        }
                                    }
                                },
                                _=> break
                            }
                        }

                        if total_rule_token == rule_token_index {
                            match function(&fields) {
                                Ok(token) => {
                                    let text_start_position = self.token_locations[start_token_index].start;
                                    let text_end_position   = self.token_locations[(start_token_index+total_rule_token) - 1].end;
                                    execute_rules = true;

                                    for index in start_token_index..(start_token_index+total_rule_token) {
                                        self.token_locations[index].status = TokenLocationStatus::Removed;
                                    }

                                    self.token_locations.insert(start_token_index, TokenLocation {
                                        start: text_start_position,
                                        end: text_end_position,
                                        token_type: Some(token),
                                        original_text: "".to_string(),
                                        status: TokenLocationStatus::Active
                                    });
                                },
                                Err(error) => () //println!("Parse issue: {}", error)
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn add_token_location(&mut self, start: usize, end: usize, token_type: Option<TokenType>, text: String) -> bool {
        for item in &self.token_locations {
            if item.start < start && item.end > start {
                return false
            }
            else if item.start < end && item.end > end {
                return false
            }
        }

        self.token_locations.push(TokenLocation {
            start: start,
            end: end,
            token_type: token_type,
            original_text: text,
            status: TokenLocationStatus::Active
        });
        true
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

    pub fn add_token(&mut self, start: u16, token: TokenType) {
        let token = Token {
            start,
            end: self.column,
            token,
            is_temp: false
        };
        self.tokens.push(token);
    }

    pub fn increase_index(&mut self) {
        self.index   += self.get_char().len_utf8() as u16;
        self.indexer += 1;
        self.column  += 1;
    }
}

#[cfg(test)]
extern crate alloc;

#[cfg(test)]
pub mod test {
    use crate::executer::initialize;
    use crate::tokinizer::Tokinizer;
    use core::cell::RefCell;
    use crate::types::TokenType;
    use alloc::vec::Vec;
    use alloc::string::String;
    use alloc::string::ToString;

    pub fn setup(data: String) -> RefCell<Tokinizer> {
        let tokinizer = Tokinizer {
            column: 0,
            line: 0,
            tokens: Vec::new(),
            iter: data.chars().collect(),
            data: data.to_string(),
            index: 0,
            indexer: 0,
            total: data.chars().count(),
            token_locations: Vec::new()
        };
        initialize();
        RefCell::new(tokinizer)
    }

    #[cfg(test)]
    #[test]
    fn alias_test() {
        use alloc::string::ToString;
        use crate::tokinizer::test::setup;
        let tokinizer_mut = setup("add 1024 percent".to_string());

        tokinizer_mut.borrow_mut().tokinize_with_regex();
        tokinizer_mut.borrow_mut().apply_aliases();
        let tokens = &tokinizer_mut.borrow().token_locations;

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].start, 0);
        assert_eq!(tokens[0].end, 3);
        assert_eq!(tokens[0].token_type, Some(TokenType::Operator('+')));

        assert_eq!(tokens[1].start, 4);
        assert_eq!(tokens[1].end, 8);
        assert_eq!(tokens[1].token_type, Some(TokenType::Number(1024.0)));

        assert_eq!(tokens[2].start, 9);
        assert_eq!(tokens[2].end, 16);
        assert_eq!(tokens[2].token_type, Some(TokenType::Operator('%')));
    }
}