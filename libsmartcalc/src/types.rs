use std::vec::Vec;
use std::result::Result;
use std::rc::Rc;
use std::collections::HashMap;
use chrono::{NaiveDateTime, NaiveTime};
use crate::executer::Storage;

#[cfg(target_arch = "wasm32")]
use js_sys::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub type TokinizeResult     = Result<Vec<Token>, (&'static str, u16, u16)>;
pub type ExpressionFunc     = fn(fields: &HashMap<String, &Token>) -> std::result::Result<TokenType, String>;
pub type TokenParserResult  = Result<bool, (&'static str, u16)>;
pub type AstResult          = Result<BramaAstType, (&'static str, u16, u16)>;


#[derive(Debug)]
#[derive(PartialEq)]
pub struct VariableInfo {
    pub index: usize,
    pub name: String,
    pub tokens: Vec<Token>,
    pub data: Rc<BramaAstType>
}

impl VariableInfo {
    pub fn update_data(&mut self, data: Rc<BramaAstType>) {
        self.data = data;
    }
}

impl ToString for VariableInfo {
    fn to_string(&self) -> String {
        self.name.to_string()
    }
}

pub struct Sentence {
    pub text: String,
    pub func: ExpressionFunc
}

impl Sentence {
    pub fn new(text: String, func: ExpressionFunc) -> Sentence {
        Sentence {
            text,
            func
        }
    }
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum FieldType {
    Text(String),
    Date(String),
    Time(String),
    Money(String),
    Percent(String),
    Number(String)
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaNumberSystem {
    Binary      = 0,
    Octal       = 1,
    Decimal     = 2,
    Hexadecimal = 3
}

#[derive(Debug, Clone)]
pub struct Token {
    pub start: u16,
    pub end: u16,
    pub token: TokenType,
    pub is_temp: bool
}

impl Token {
    #[cfg(target_arch = "wasm32")]
    pub fn as_js_object(&self) -> Object {
        let start_ref       = JsValue::from("start");
        let end_ref         = JsValue::from("end");
        let type_ref        = JsValue::from("type");

        let token_object = js_sys::Object::new();
        let token_type = match self.token {
            TokenType::Number(_) => 1,
            TokenType::Percent(_) => 2,
            TokenType::Time(_) => 3,
            TokenType::Operator(_) => 4,
            TokenType::Text(_) => 5,
            TokenType::DateTime(_) => 6,
            TokenType::Money(_, _) => 7,
            TokenType::Variable(_) => 8,
            _ => 0
        };

        Reflect::set(token_object.as_ref(), start_ref.as_ref(),  JsValue::from(self.start).as_ref()).unwrap();
        Reflect::set(token_object.as_ref(), end_ref.as_ref(),    JsValue::from(self.end).as_ref()).unwrap();
        Reflect::set(token_object.as_ref(), type_ref.as_ref(),   JsValue::from(token_type).as_ref()).unwrap();
        token_object
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Number(f64),
    Text(Rc<String>),
    Time(NaiveTime),
    Date(NaiveDateTime),
    DateTime(NaiveDateTime),
    Operator(char),
    Field(Rc<FieldType>),
    Percent(f64),
    Money(f64, String),
    Variable(Rc<VariableInfo>)
}


impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        match (&self, &other) {
            (TokenType::Text(l_value),     TokenType::Text(r_value)) => *l_value == *r_value,
            (TokenType::Number(l_value),   TokenType::Number(r_value)) => l_value == r_value,
            (TokenType::Percent(l_value),  TokenType::Percent(r_value)) => l_value == r_value,
            (TokenType::Operator(l_value), TokenType::Operator(r_value)) => l_value == r_value,
            (TokenType::Variable(l_value), TokenType::Variable(r_value)) => l_value == r_value,
            (TokenType::Money(l_value, l_symbol), TokenType::Money(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
            (TokenType::Time(l_value),     TokenType::Time(r_value)) => l_value == r_value,
            (TokenType::Field(l_value),    TokenType::Field(r_value)) => {
                match (&**l_value, &**r_value) {
                    (FieldType::Percent(l), FieldType::Percent(r)) => r == l,
                    (FieldType::Number(l),  FieldType::Number(r)) => r == l,
                    (FieldType::Text(l),    FieldType::Text(r)) => r == l,
                    (FieldType::Date(l),    FieldType::Date(r)) => r == l,
                    (FieldType::Time(l),    FieldType::Time(r)) => r == l,
                    (FieldType::Money(l),   FieldType::Money(r)) => r == l,
                    (_, _) => false,
                }
            },
            (_, _)  => false
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match &self.token {
            TokenType::Number(number) => number.to_string(),
            TokenType::Text(text) => text.to_string(),
            TokenType::Time(time) => time.to_string(),
            TokenType::Date(date) => date.to_string(),
            TokenType::DateTime(datetime) => datetime.to_string(),
            TokenType::Operator(ch) => ch.to_string(),
            TokenType::Field(_) => "field".to_string(),
            TokenType::Percent(number) => format!("%{}", number),
            TokenType::Money(price, currency) => format!("{}{}", price, currency),
            TokenType::Variable(var) => var.to_string()
        }
    }
}

impl Token {
    pub fn data_compare(left: &Token, right: &Token) -> bool {
        match (&left.token, &right.token) {
            (TokenType::Text(l_value),     TokenType::Text(r_value))     => l_value == r_value,
            (TokenType::Number(l_value),   TokenType::Number(r_value))   => l_value == r_value,
            (TokenType::Percent(l_value),  TokenType::Percent(r_value))  => l_value == r_value,
            (TokenType::Operator(l_value), TokenType::Operator(r_value)) => l_value == r_value,
            (TokenType::Variable(l_value), TokenType::Variable(r_value)) => l_value == r_value,
            (TokenType::Field(l_value),    TokenType::Field(r_value))    => l_value == r_value,
            (_, _)  => false
        }
    }

    pub fn variable_compare(left: &Token, right: Rc<BramaAstType>) -> bool {
        match (&left.token, &*right) {
            (TokenType::Text(l_value), BramaAstType::Symbol(r_value)) => &**l_value == r_value,
            (TokenType::Number(l_value), BramaAstType::Number(r_value)) => *l_value == *r_value,
            (TokenType::Percent(l_value), BramaAstType::Percent(r_value)) => *l_value == *r_value,
            (TokenType::Time(l_value), BramaAstType::Time(r_value)) => *l_value == *r_value,
            (TokenType::Field(l_value), _) => {
                match (&**l_value, &*right) {
                    (FieldType::Percent(_), BramaAstType::Percent(_)) => true,
                    (FieldType::Number(_), BramaAstType::Number(_)) => true,
                    (FieldType::Text(_), BramaAstType::Symbol(_)) => true,
                    (FieldType::Time(_), BramaAstType::Time(_)) => true,
                    (_, _) => false,
                }
            },
            (_, _) => false
        }
    }

    pub fn get_field_name(token: &Token) -> Option<String> {
        match &token.token {
            TokenType::Field(field) => Some(match &**field {
                FieldType::Text(field_name)    => field_name.to_string(),
                FieldType::Date(field_name)    => field_name.to_string(),
                FieldType::Time(field_name)    => field_name.to_string(),
                FieldType::Money(field_name)   => field_name.to_string(),
                FieldType::Percent(field_name) => field_name.to_string(),
                FieldType::Number(field_name)  => field_name.to_string()
            }),
            _ => None
        }
    }

    pub fn is_same(tokens: &Vec<Token>, rule_tokens: &Vec<Token>) -> Option<usize> {
        let total_rule_token       = rule_tokens.len();
        let mut rule_token_index   = 0;
        let mut target_token_index = 0;
        let mut start_token_index  = 0;

        loop {
            match tokens.get(target_token_index) {
                Some(token) => {
                    if token == &rule_tokens[rule_token_index] {
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
                _=> break
            }
        }

        if total_rule_token == rule_token_index {
            return Some(start_token_index);
        }
        None
    }

    pub fn update_for_variable(tokens: &mut Vec<Token>, storage: Rc<Storage>) {
        let mut token_start_index = 0;
        for (index, token) in tokens.iter().enumerate() {
            match token.token {
                TokenType::Operator('=') => {
                    token_start_index = index as usize + 1;
                    break;
                },
                _ => ()
            };
        }

       let mut update_tokens = true;

        while update_tokens {
            let mut found            = false;
            let mut closest_variable = usize::max_value();
            let mut variable_index   = 0;
            let mut variable_size    = 0;

            update_tokens            = false;

            for (index, variable) in storage.variables.borrow().iter().enumerate() {
                if let Some(start_index) = Token::is_same(&tokens[token_start_index..].to_vec(), &variable.tokens) {
                    if start_index == closest_variable && variable_size < variable.tokens.len() {
                        closest_variable = start_index;
                        variable_index   = index;
                        variable_size    = variable.tokens.len();
                        found = true;
                    }
                    else if start_index < closest_variable {
                        closest_variable = start_index;
                        variable_index   = index;
                        variable_size    = variable.tokens.len();
                        found = true;
                    }
                }
            }

            if found {
                let remove_start_index  = token_start_index + closest_variable;
                let remove_end_index    = remove_start_index + variable_size;
                let text_start_position = tokens[remove_start_index].start;
                let text_end_position   = tokens[remove_end_index - 1].end;
                tokens.drain(remove_start_index..remove_end_index);
                tokens.insert(remove_start_index, Token {
                    start: text_start_position,
                    end: text_end_position,
                    token: TokenType::Variable(storage.variables.borrow()[variable_index].clone()),
                    is_temp: false
                });
                update_tokens = true;
            }
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (&self.token, &other.token) {
            (TokenType::Text(l_value),     TokenType::Text(r_value)) => l_value == r_value,
            (TokenType::Number(l_value),   TokenType::Number(r_value)) => l_value == r_value,
            (TokenType::Percent(l_value),  TokenType::Percent(r_value)) => l_value == r_value,
            (TokenType::Operator(l_value), TokenType::Operator(r_value)) => l_value == r_value,
            (TokenType::Variable(l_value), TokenType::Variable(r_value)) => l_value == r_value,
            (TokenType::Field(l_value), _) => {
                match (&**l_value, &other.token) {
                    (FieldType::Percent(_), TokenType::Percent(_)) => true,
                    (FieldType::Number(_),  TokenType::Number(_)) => true,
                    (FieldType::Text(_),    TokenType::Text(_)) => true,
                    (FieldType::Time(_),    TokenType::Time(_)) => true,
                    (_, _) => false,
                }
            },
            (_, TokenType::Field(r_value)) => {
                match (&**r_value, &self.token) {
                    (FieldType::Percent(_), TokenType::Percent(_)) => true,
                    (FieldType::Number(_),  TokenType::Number(_)) => true,
                    (FieldType::Text(_),    TokenType::Text(_)) => true,
                    (FieldType::Time(_),    TokenType::Time(_)) => true,
                    (_, _) => false
                }
            },
            (_, _)  => false
        }
    }
}

pub struct TokinizerBackup {
    pub index: u16,
    pub indexer: usize,
    pub column: u16
}

pub trait CharTraits {
    fn is_new_line(&self) -> bool;
    fn is_whitespace(&self) -> bool;
}

impl CharTraits for char {
    fn is_new_line(&self) -> bool {
        *self == '\n'
    }

    fn is_whitespace(&self) -> bool {
        matches!(*self, ' ' | '\r' | '\t')
    }
}


#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaAstType {
    None,
    Number(f64),
    Field(Rc<FieldType>),
    Percent(f64),
    Time(NaiveTime),
    Binary {
        left: Rc<BramaAstType>,
        operator: char,
        right: Rc<BramaAstType>
    },
    PrefixUnary(char, Rc<BramaAstType>),
    Assignment {
        index: usize,
        expression: Rc<BramaAstType>
    },
    Symbol(String),
    Variable(Rc<VariableInfo>)
}