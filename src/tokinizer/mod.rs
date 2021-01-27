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

use serde_json::{Result, Value, from_str};
use regex::{Regex};

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

fn time_parse(data: &mut String, group_item: &Value) -> String {
    let mut data_str = data.to_string();

    for time_pattern in group_item.as_array().unwrap() {
        let re = Regex::new(time_pattern.as_str().unwrap()).unwrap();
        for capture in re.captures_iter(data) {
            let mut hour = capture.name("hour").unwrap().as_str().parse::<i32>().unwrap();
            let minute   = capture.name("minute").unwrap().as_str().parse::<i32>().unwrap();
            let second   = match capture.name("second") {
                Some(second) => second.as_str().parse::<i32>().unwrap(),
                _ => 0
            };

            if let Some(meridiem) = capture.name("meridiem") {
                if meridiem.as_str().to_lowercase() == "pm" {
                    hour += 12;
                }
            }

            let time_number: u32 = ((hour * 60 * 60) + (minute * 60) + second) as u32;
            data_str = data_str.replace(capture.get(0).unwrap().as_str(), &format!("[TIME:{}]", time_number)[..]);
        }
    }

    data_str
}

impl Tokinizer {
    pub fn tokinize(data: &String) -> TokinizeResult {
        let mut data_str = data.to_string();
        let json_data = fs::read_to_string("/Users/erhanbaris/ClionProjects/smartcalculator/smartcalc/src/json/regex.json").expect("{}");
        let json_value: Result<Value> = from_str(&json_data);
        match json_value {
            Ok(json) => {
                if let Some(group) = json.get("alias").unwrap().as_object() {
                    for (key, value) in group.iter() {
                        let re = Regex::new(&format!(r"\b{}\b", key.as_str())[..]).unwrap();
                        data_str = re.replace_all(&data_str, value.as_str().unwrap()).to_string();
                    }
                }

                if let Some(group) = json.get("parse").unwrap().as_object() {
                    for (group, group_item) in group.iter() {
                        data_str = match group.as_str() {
                            "time" => time_parse(&mut data_str, group_item),
                            _ => data_str
                        };
                    }
                }
            },
            _ => ()
        };

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

