mod number;
mod operator;
mod text;
mod whitespace;
mod field;
mod percent;
mod atom;
use std::fs;

use crate::types::*;
use self::number::number_parser;
use self::operator::operator_parser;
use self::text::text_parser;
use self::whitespace::whitespace_parser;
use self::field::field_parser;
use self::percent::percent_parser;
use self::atom::atom_parser;

pub type TokenParser = fn(tokinizer: &mut Tokinizer) -> TokenParserResult;
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
        let mut tokinizer = Tokinizer {
            column: 0,
            line: 0,
            tokens: Vec::new(),
            iter: data.chars().collect(),
            data: data.to_string(),
            index: 0,
            indexer: 0,
            total: data.chars().count()
        };

        let token_parses : Vec<TokenParser> = vec![atom_parser, field_parser, percent_parser, whitespace_parser, text_parser, number_parser, operator_parser];

        while !tokinizer.is_end() {
            for parse in &token_parses {
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
            index: self.index
        }
    }

    pub fn set_indexer(&mut self, backup: TokinizerBackup) {
        self.indexer = backup.indexer;
        self.index   = backup.index;
    }

    pub fn add_token(&mut self, _start: u16, token_type: Token) {
        /*let token = Token {
            start,
            end: self.column,
            token_type
        };*/
        self.tokens.push(token_type);
    }

    pub fn increase_index(&mut self) {
        self.index   += self.get_char().len_utf8() as u16;
        self.indexer += 1;
        self.column  += 1;
    }
}

