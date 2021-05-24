use alloc::vec::Vec;
use core::result::Result;
use alloc::rc::Rc;
use alloc::string::ToString;
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::format;

use serde_derive::{Deserialize, Serialize};
use alloc::collections::btree_map::BTreeMap;
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use crate::{executer::Storage};
use crate::token::ui_token::{UiTokenType};

use crate::tokinizer::{TokenInfo, TokenInfoStatus, Tokinizer};

pub type TokinizeResult     = Result<Vec<TokenInfo>, (&'static str, u16, u16)>;
pub type ExpressionFunc     = fn(tokinizer: &Tokinizer, fields: &BTreeMap<String, &TokenInfo>) -> core::result::Result<TokenType, String>;
pub type TokenParserResult  = Result<bool, (&'static str, u16)>;
pub type AstResult          = Result<BramaAstType, (&'static str, u16, u16)>;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct VariableInfo {
    pub index: usize,
    pub name: String,
    pub tokens: Vec<TokenType>,
    pub data: Rc<BramaAstType>
}

unsafe impl Send for VariableInfo {}
unsafe impl Sync for VariableInfo {}

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
    Number(String),
    Group(String, Vec<String>),
    TypeGroup(Vec<String>, String),
    Month(String),
    Duration(String)
}

unsafe impl Send for FieldType {}
unsafe impl Sync for FieldType {}

impl FieldType {
    pub fn type_name(&self) -> String {
        match self {
            FieldType::Text(_) => "TEXT".to_string(),
            FieldType::Date(_) => "DATE".to_string(),
            FieldType::Time(_) => "TIME".to_string(),
            FieldType::Money(_) => "MONEY".to_string(),
            FieldType::Percent(_) => "PERCENT".to_string(),
            FieldType::Number(_) => "NUMBER".to_string(),
            FieldType::Group(_, _) => "GROUP".to_string(),
            FieldType::TypeGroup(_, _) => "TYPE_GROUP".to_string(),
            FieldType::Month(_) => "MONTH".to_string(),
            FieldType::Duration(_) => "DURATION".to_string()
        }
    }
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

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Money {
    pub code: String,
    pub symbol: String,

    #[serde(alias = "thousandsSeparator")]
    pub thousands_separator: String,

    #[serde(alias = "decimalSeparator")]
    pub decimal_separator: String,

    #[serde(alias = "symbolOnLeft")]
    pub symbol_on_left: bool,

    #[serde(alias = "spaceBetweenAmountAndSymbol")]
    pub space_between_amount_and_symbol: bool,

    #[serde(alias = "decimalDigits")]
    pub decimal_digits: u8
}


#[derive(Debug, Clone)]
pub enum TokenType {
    Number(f64),
    Text(String),
    Time(NaiveTime),
    Date(NaiveDate),
    DateTime(NaiveDateTime),
    Operator(char),
    Field(Rc<FieldType>),
    Percent(f64),
    Money(f64, String),
    Variable(Rc<VariableInfo>),
    Month(u32),
    Duration(Duration)
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
            (TokenType::Month(l_value),     TokenType::Month(r_value)) => l_value == r_value,
            (TokenType::Duration(l_value),     TokenType::Duration(r_value)) => l_value == r_value,
            (TokenType::Date(l_value),     TokenType::Date(r_value)) => l_value == r_value,
            (TokenType::Field(l_value),    TokenType::Field(r_value)) => {
                match (&**l_value, &**r_value) {
                    (FieldType::Percent(l), FieldType::Percent(r)) => r == l,
                    (FieldType::Number(l),  FieldType::Number(r)) => r == l,
                    (FieldType::Text(l),    FieldType::Text(r)) => r == l,
                    (FieldType::Date(l),    FieldType::Date(r)) => r == l,
                    (FieldType::Time(l),    FieldType::Time(r)) => r == l,
                    (FieldType::Money(l),   FieldType::Money(r)) => r == l,
                    (FieldType::Month(l),   FieldType::Month(r)) => r == l,
                    (FieldType::Duration(l),   FieldType::Duration(r)) => r == l,
                    (FieldType::Group(_, l),   FieldType::Group(_, r)) => r == l,
                    (FieldType::TypeGroup(l1, l2),   FieldType::TypeGroup(r1, r2)) => r1 == l1 && r2 == l2,
                    (_, _) => false,
                }
            },
            (_, _)  => false
        }
    }
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match &self {
            TokenType::Number(number) => number.to_string(),
            TokenType::Text(text) => text.to_string(),
            TokenType::Time(time) => time.to_string(),
            TokenType::Date(date) => date.to_string(),
            TokenType::DateTime(datetime) => datetime.to_string(),
            TokenType::Operator(ch) => ch.to_string(),
            TokenType::Field(_) => "field".to_string(),
            TokenType::Percent(number) => format!("%{}", number),
            TokenType::Money(price, currency) => format!("{} {}", price, currency.to_string()),
            TokenType::Variable(var) => var.to_string(),
            TokenType::Month(month) => month.to_string(),
            TokenType::Duration(duration) => duration.to_string()
        }
    }
}


impl TokenType {
    pub fn type_name(&self) -> String {
        match self {
            TokenType::Number(_) => "NUMBER".to_string(),
            TokenType::Text(_) => "TEXT".to_string(),
            TokenType::Time(_) => "TIME".to_string(),
            TokenType::Date(_) => "DATE".to_string(),
            TokenType::DateTime(_) => "DATE_TIME".to_string(),
            TokenType::Operator(_) => "OPERATOR".to_string(),
            TokenType::Field(_) => "FIELD".to_string(),
            TokenType::Percent(_) => "PERCENT".to_string(),
            TokenType::Money(_, _) => "MONEY".to_string(),
            TokenType::Variable(_) => "VARIABLE".to_string(),
            TokenType::Month(_) => "MONTH".to_string(),
            TokenType::Duration(_) => "DURATION".to_string()
        }
    }

    pub fn variable_compare(left: &TokenInfo, right: Rc<BramaAstType>) -> bool {
        match &left.token_type {
            Some(token) => match (&token, &*right) {
                (TokenType::Text(l_value), BramaAstType::Symbol(r_value)) => &**l_value == r_value,
                (TokenType::Number(l_value), BramaAstType::Number(r_value)) => *l_value == *r_value,
                (TokenType::Percent(l_value), BramaAstType::Percent(r_value)) => *l_value == *r_value,
                (TokenType::Duration(l_value), BramaAstType::Duration(r_value)) => *l_value == *r_value,
                (TokenType::Time(l_value), BramaAstType::Time(r_value)) => *l_value == *r_value,
                (TokenType::Money(l_value, l_symbol), BramaAstType::Money(r_value, r_symbol)) => l_value == r_value && l_symbol.to_string() == *r_symbol,
                (TokenType::Date(l_value), BramaAstType::Date(r_value)) => *l_value == *r_value,
                (TokenType::Field(l_value), _) => {
                    match (&**l_value, &*right) {
                        (FieldType::Percent(_), BramaAstType::Percent(_)) => true,
                        (FieldType::Number(_), BramaAstType::Number(_)) => true,
                        (FieldType::Text(_), BramaAstType::Symbol(_)) => true,
                        (FieldType::Time(_), BramaAstType::Time(_)) => true,
                        (FieldType::Money(_),   BramaAstType::Money(_, _)) => true,
                        (FieldType::Month(_),   BramaAstType::Month(_)) => true,
                        (FieldType::Duration(_),   BramaAstType::Duration(_)) => true,
                        (FieldType::Date(_),   BramaAstType::Date(_)) => true,
                        (FieldType::TypeGroup(types, _), right_ast) => types.contains(&right_ast.type_name()),
                        (_, _) => false,
                    }
                },
                (_, _) => false
            },
            _ => false
        }
    }

    pub fn get_field_name(token: &TokenInfo) -> Option<String> {
        match &token.token_type {
            Some(token_type) => match &token_type {
                TokenType::Field(field) => match &**field {
                    FieldType::Text(field_name)    => Some(field_name.to_string()),
                    FieldType::Date(field_name)    => Some(field_name.to_string()),
                    FieldType::Time(field_name)    => Some(field_name.to_string()),
                    FieldType::Money(field_name)   => Some(field_name.to_string()),
                    FieldType::Percent(field_name) => Some(field_name.to_string()),
                    FieldType::Number(field_name)  => Some(field_name.to_string()),
                    FieldType::Month(field_name)  => Some(field_name.to_string()),
                    FieldType::Duration(field_name)  => Some(field_name.to_string()),
                    FieldType::Group(field_name, _)  => Some(field_name.to_string()),
                    FieldType::TypeGroup(_, field_name) => Some(field_name.to_string())
                },
                _ => None
            },
            _ => None
        }
    }

    pub fn is_same(tokens: &Vec<TokenType>, rule_tokens: &Vec<TokenType>) -> Option<usize> {
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

    pub fn is_same_location(tokens: &Vec<TokenInfo>, rule_tokens: &Vec<TokenType>) -> Option<usize> {
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

    pub fn update_for_variable(tokenizer: &mut Tokinizer, storage: Rc<Storage>) {
        let mut token_start_index = 0;
        for (index, token) in tokenizer.token_infos.iter().enumerate() {
            match &token.token_type {
                Some(token) => match token {
                    TokenType::Operator('=') => {
                        token_start_index = index as usize + 1;

                        tokenizer.ui_tokens.update_tokens(0, tokenizer.token_infos[index - 1].end, UiTokenType::VariableDefination);                        
                        break;
                    },
                    _ => ()
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
                if let Some(start_index) = TokenType::is_same_location(&tokenizer.token_infos[token_start_index..].to_vec(), &variable.tokens) {
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
                let text_start_position = tokenizer.token_infos[remove_start_index].start;
                let text_end_position   = tokenizer.token_infos[remove_end_index - 1].end;

                tokenizer.ui_tokens.update_tokens(text_start_position, text_end_position, UiTokenType::VariableUse);

                let buffer_length: usize = tokenizer.token_infos[remove_start_index..remove_end_index].iter().map(|s| s.original_text.len()).sum();
                let mut original_text = String::with_capacity(buffer_length);

                for token in tokenizer.token_infos[remove_start_index..remove_end_index].iter() {
                    original_text.push_str(&token.original_text.to_owned());
                }

                tokenizer.token_infos.drain(remove_start_index..remove_end_index);
                tokenizer.token_infos.insert(remove_start_index, TokenInfo {
                    start: text_start_position as usize,
                    end: text_end_position as usize,
                    token_type: Some(TokenType::Variable(storage.variables.borrow()[variable_index].clone())),
                    original_text: original_text.to_owned(),
                    status: TokenInfoStatus::Active
                });
                update_tokens = true;
            }
        }
    }
}

impl core::cmp::PartialEq<TokenType> for TokenInfo {
    fn eq(&self, other: &TokenType) -> bool {
        if self.token_type.is_none() {
            return false
        }

        match &self.token_type {
            Some(l_token) => match (&l_token, &other) {
                (TokenType::Text(l_value),     TokenType::Text(r_value)) => l_value == r_value,
                (TokenType::Number(l_value),   TokenType::Number(r_value)) => l_value == r_value,
                (TokenType::Percent(l_value),  TokenType::Percent(r_value)) => l_value == r_value,
                (TokenType::Operator(l_value), TokenType::Operator(r_value)) => l_value == r_value,
                (TokenType::Date(l_value), TokenType::Date(r_value)) => l_value == r_value,
                (TokenType::Duration(l_value), TokenType::Duration(r_value)) => l_value == r_value,
                (TokenType::Month(l_value), TokenType::Month(r_value)) => l_value == r_value,
                (TokenType::Money(l_value, l_symbol), TokenType::Money(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
                (TokenType::Variable(l_value), TokenType::Variable(r_value)) => l_value == r_value,
                (TokenType::Field(l_value), _) => {
                    match (&**l_value, &other) {
                        (FieldType::Percent(_), TokenType::Percent(_)) => true,
                        (FieldType::Number(_),  TokenType::Number(_)) => true,
                        (FieldType::Text(_),    TokenType::Text(_)) => true,
                        (FieldType::Time(_),    TokenType::Time(_)) => true,
                        (FieldType::Date(_),    TokenType::Date(_)) => true,
                        (FieldType::Money(_),   TokenType::Money(_, _)) => true,
                        (FieldType::Month(_),   TokenType::Month(_)) => true,
                        (FieldType::Duration(_),   TokenType::Duration(_)) => true,
                        (FieldType::Group(_, items),    TokenType::Text(text)) => items.iter().find(|&item| item == text).is_some(),
                        (FieldType::TypeGroup(types, _), right_ast) => types.contains(&right_ast.type_name()),
                        (_, _) => false,
                    }
                },
                (_, TokenType::Field(r_value)) => {
                    match (&**r_value, &l_token) {
                        (FieldType::Percent(_), TokenType::Percent(_)) => true,
                        (FieldType::Number(_),  TokenType::Number(_)) => true,
                        (FieldType::Text(_),    TokenType::Text(_)) => true,
                        (FieldType::Time(_),    TokenType::Time(_)) => true,
                        (FieldType::Date(_),    TokenType::Date(_)) => true,
                        (FieldType::Money(_),   TokenType::Money(_, _)) => true,
                        (FieldType::Duration(_),   TokenType::Duration(_)) => true,
                        (FieldType::Month(_),   TokenType::Month(_)) => true,
                        (FieldType::Group(_, items),    TokenType::Text(text)) => items.iter().find(|&item| item == text).is_some(),
                        (FieldType::TypeGroup(types, _), right_ast) => types.contains(&right_ast.type_name()),
                        (_, _) => false
                    }
                },
                (_, _)  => false
            },
            _ => false
        }
    }
}

impl PartialEq for TokenInfo {
    fn eq(&self, other: &Self) -> bool {
        if self.token_type.is_none() || other.token_type.is_none() {
            return false
        }

        match (&self.token_type, &other.token_type) {
            (Some(l_token), Some(r_token)) => match (&l_token, &r_token) {
                (TokenType::Text(l_value),     TokenType::Text(r_value)) => l_value == r_value,
                (TokenType::Number(l_value),   TokenType::Number(r_value)) => l_value == r_value,
                (TokenType::Percent(l_value),  TokenType::Percent(r_value)) => l_value == r_value,
                (TokenType::Operator(l_value), TokenType::Operator(r_value)) => l_value == r_value,
                (TokenType::Date(l_value), TokenType::Date(r_value)) => l_value == r_value,
                (TokenType::Duration(l_value), TokenType::Duration(r_value)) => l_value == r_value,
                (TokenType::Money(l_value, l_symbol), TokenType::Money(r_value, r_symbol)) => l_value == r_value && l_symbol == r_symbol,
                (TokenType::Variable(l_value), TokenType::Variable(r_value)) => l_value == r_value,
                (TokenType::Field(l_value), _) => {
                    match (&**l_value, &r_token) {
                        (FieldType::Percent(_), TokenType::Percent(_)) => true,
                        (FieldType::Number(_),  TokenType::Number(_)) => true,
                        (FieldType::Text(_),    TokenType::Text(_)) => true,
                        (FieldType::Time(_),    TokenType::Time(_)) => true,
                        (FieldType::Date(_),    TokenType::Date(_)) => true,
                        (FieldType::Money(_),   TokenType::Money(_, _)) => true,
                        (FieldType::Duration(_), TokenType::Duration(_)) => true,
                        (FieldType::Month(_),   TokenType::Month(_)) => true,
                        (FieldType::Group(_, items),    TokenType::Text(text)) => items.iter().find(|&item| item == text).is_some(),
                        (FieldType::TypeGroup(types, _), right_ast) => types.contains(&right_ast.type_name()),
                        (_, _) => false,
                    }
                },
                (_, TokenType::Field(r_value)) => {
                    match (&**r_value, &l_token) {
                        (FieldType::Percent(_), TokenType::Percent(_)) => true,
                        (FieldType::Number(_),  TokenType::Number(_)) => true,
                        (FieldType::Text(_),    TokenType::Text(_)) => true,
                        (FieldType::Time(_),    TokenType::Time(_)) => true,
                        (FieldType::Date(_),    TokenType::Date(_)) => true,
                        (FieldType::Money(_),   TokenType::Money(_, _)) => true,
                        (FieldType::Month(_),   TokenType::Month(_)) => true,
                        (FieldType::Duration(_), TokenType::Duration(_)) => true,
                        (FieldType::Group(_, items),    TokenType::Text(text)) => items.iter().find(|&item| item == text).is_some(),
                        (FieldType::TypeGroup(types, _), right_ast) => types.contains(&right_ast.type_name()),
                        (_, _) => false
                    }
                },
                (_, _)  => false
            },
            (_, _) => false
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
    Money(f64, String),
    Month(u32),
    Date(NaiveDate),
    Duration(Duration),
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

impl BramaAstType {
    pub fn type_name(&self) -> String {
        match self {
            BramaAstType::None => "NONE".to_string(),
            BramaAstType::Number(_) => "NUMBER".to_string(),
            BramaAstType::Field(_) => "FIELD".to_string(),
            BramaAstType::Percent(_) => "PERCENT".to_string(),
            BramaAstType::Time(_) => "TIME".to_string(),
            BramaAstType::Money(_, _) => "MONEY".to_string(),
            BramaAstType::Month(_) => "MONTH".to_string(),
            BramaAstType::Date(_) => "DATE".to_string(),
            BramaAstType::Duration(_) => "DURATION".to_string(),
            BramaAstType::Binary {
                left: _,
                operator: _,
                right: _
            } => "binary".to_string(),
            BramaAstType::PrefixUnary(_, _) => "PREFIX_UNARY".to_string(),
            BramaAstType::Assignment {
                index: _,
                expression: _
            } => "ASSIGNMENT".to_string(),
            BramaAstType::Symbol(_) => "SYMBOL".to_string(),
            BramaAstType::Variable(_) => "VARIABLE".to_string()
        }
    }
}
