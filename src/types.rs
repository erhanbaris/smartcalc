use std::vec::Vec;
use std::result::Result;
use std::rc::Rc;
use std::collections::HashMap;
use chrono::{NaiveDateTime, NaiveTime};
use crate::executer::Storage;

pub type TokinizeResult     = Result<Vec<Token>, (&'static str, u16, u16)>;
pub type ExpressionFunc     = fn(fields: &HashMap<String, &Token>) -> std::result::Result<Token, String>;
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
pub enum Token {
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

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Number(number) => number.to_string(),
            Token::Text(text) => text.to_string(),
            Token::Time(time) => time.to_string(),
            Token::Date(date) => date.to_string(),
            Token::DateTime(datetime) => datetime.to_string(),
            Token::Operator(ch) => ch.to_string(),
            Token::Field(field) => "field".to_string(),
            Token::Percent(number) => format!("%{}", number),
            Token::Money(price, currency) => format!("{}{}", price, currency),
            Token::Variable(var) => var.to_string()
        }
    }
}

impl Token {
    pub fn data_compare(left: &Token, right: &Token) -> bool {
        match (left, right) {
            (Token::Text(l_value),     Token::Text(r_value))     => l_value == r_value,
            (Token::Number(l_value),   Token::Number(r_value))   => l_value == r_value,
            (Token::Percent(l_value),  Token::Percent(r_value))  => l_value == r_value,
            (Token::Operator(l_value), Token::Operator(r_value)) => l_value == r_value,
            (Token::Variable(l_value), Token::Variable(r_value)) => l_value == r_value,
            (Token::Field(l_value),    Token::Field(r_value))    => l_value == r_value,
            (_, _)  => false
        }
    }

    pub fn variable_compare(left: &Token, right: Rc<BramaAstType>) -> bool {
        match (left, &*right) {
            (Token::Text(l_value), BramaAstType::Symbol(r_value)) => &**l_value == r_value,
            (Token::Number(l_value), BramaAstType::Number(r_value)) => l_value == r_value,
            (Token::Percent(l_value), BramaAstType::Percent(r_value)) => l_value == r_value,
            (Token::Time(l_value), BramaAstType::Time(r_value)) => l_value == r_value,
            (Token::Field(l_value), _) => {
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
        match token {
            Token::Field(field) => Some(match &**field {
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
            match token {
                Token::Operator('=') => {
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
                let remove_start_index = token_start_index + closest_variable;
                let remove_end_index   = remove_start_index + variable_size;
                tokens.drain(remove_start_index..remove_end_index);
                tokens.insert(remove_start_index, Token::Variable(storage.variables.borrow()[variable_index].clone()));
                update_tokens = true;
            }
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Text(l_value),     Token::Text(r_value)) => l_value == r_value,
            (Token::Number(l_value),   Token::Number(r_value)) => l_value == r_value,
            (Token::Percent(l_value),  Token::Percent(r_value)) => l_value == r_value,
            (Token::Operator(l_value), Token::Operator(r_value)) => l_value == r_value,
            (Token::Variable(l_value), Token::Variable(r_value)) => l_value == r_value,
            (Token::Field(l_value), _) => {
                match (&**l_value, other) {
                    (FieldType::Percent(_), Token::Percent(_)) => true,
                    (FieldType::Number(_),  Token::Number(_)) => true,
                    (FieldType::Text(_),    Token::Text(_)) => true,
                    (FieldType::Time(_),    Token::Time(_)) => true,
                    (_, _) => false,
                }
            },
            (_, Token::Field(r_value)) => {
                match (&**r_value, self) {
                    (FieldType::Percent(_), Token::Percent(_)) => true,
                    (FieldType::Number(_),  Token::Number(_)) => true,
                    (FieldType::Text(_),    Token::Text(_)) => true,
                    (FieldType::Time(_),    Token::Time(_)) => true,
                    (_, _) => false
                }
            },
            (_, _)  => false
        }
    }
}

pub struct TokinizerBackup {
    pub index: u16,
    pub indexer: usize
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