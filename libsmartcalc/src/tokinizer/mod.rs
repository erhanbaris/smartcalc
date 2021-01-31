mod number;
mod operator;
mod text;
mod whitespace;
mod field;
mod percent;
mod atom;
mod time;
mod money;

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
use crate::constants::TOKEN_PARSE_REGEXES;

use regex::{Regex, Match};
use lazy_static::*;

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
        let m = vec![
            ("percent", percent_regex_parser as RegexParser),
            ("money",   money_regex_parser   as RegexParser),
            ("time",    time_regex_parser    as RegexParser),
            ("number",  number_regex_parser  as RegexParser)
        ];
        m
    };
}


pub type TokenParser = fn(tokinizer: &mut Tokinizer) -> TokenParserResult;
pub type RegexParser = fn(data: &mut String, group_item: &Vec<Regex>) -> String;

pub fn validate_capture<'t>(text: &String, capture: Match<'t>) -> bool {
    let start = capture.start();
    let end   = capture.end();

    if start > 0 && text.chars().nth(start - 1).unwrap() == ':' {
        return false
    }

    if text.len() > end && text.chars().nth(end).unwrap() == ']' {
        return false
    }

    true
}

pub struct Tokinizer {
    pub line  : u16,
    pub column: u16,
    pub tokens: Vec<Token>,
    pub iter: Vec<char>,
    pub data: String,
    pub index: u16,
    pub indexer: usize,
    pub total: usize
}

impl Tokinizer {
    pub fn tokinize(data: &String) -> TokinizeResult {
        let mut data_str = data.to_string();

        /* Token parser with regex */
        for (key, func) in TOKEN_REGEX_PARSER.iter() {
            data_str = match TOKEN_PARSE_REGEXES.lock().unwrap().get(&key.to_string()) {
                Some(items) => func(&mut data_str, items),
                _ => data_str
            };
        }

        let mut tokinizer = Tokinizer {
            column: 0,
            line: 0,
            tokens: Vec::new(),
            iter: data_str.chars().collect(),
            data: data_str.to_string(),
            index: 0,
            indexer: 0,
            total: data_str.chars().count()
        };

        while !tokinizer.is_end() {
            for parse in TOKEN_PARSER.iter() {
                let status = parse(&mut tokinizer);
                match status {
                    Ok(true) => break,
                    Ok(false) => continue,
                    Err((message, column)) => return Err((message, 0, column))
                }
            }
        }

        Ok(tokinizer.tokens)
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

