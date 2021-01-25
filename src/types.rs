use std::vec::Vec;
use std::result::Result;
use std::rc::Rc;
use std::collections::HashMap;
use chrono::NaiveDateTime;

pub type ParseResult        = Result<Vec<Token>, (&'static str, u16, u16)>;
pub type ExpressionFunc     = fn(atoms: &HashMap<String, &Token>) -> std::result::Result<Token, String>;
pub type TokenParserResult  = Result<bool, (&'static str, u16)>;
pub type AstResult          = Result<BramaAstType, (&'static str, u16, u16)>;

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
pub enum AtomType {
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
    Time(NaiveDateTime),
    Operator(char),
    Atom(Rc<AtomType>),
    Percent(f64)
}

impl Token {
    pub fn is_same(tokens: &Vec<Token>, rule_tokens: &Vec<Token>) -> Option<usize> {
        let mut total_rule_token   = rule_tokens.len();
        let mut rule_token_index   = 0;
        let mut target_token_index = 0;
        let mut start_token_index  = 0;

        loop {
            match tokens.get(target_token_index) {
                Some(token) => {
                    if token == &rule_tokens[rule_token_index] {
                        if let Token::Atom(atom) = &rule_tokens[rule_token_index] {
                            let atom_name = match &**atom {
                                AtomType::Text(atom_name)    => atom_name,
                                AtomType::Date(atom_name)    => atom_name,
                                AtomType::Time(atom_name)    => atom_name,
                                AtomType::Money(atom_name)   => atom_name,
                                AtomType::Percent(atom_name) => atom_name,
                                AtomType::Number(atom_name)  => atom_name
                            };
                        }

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
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Text(l_value), Token::Text(r_value)) => l_value == r_value,
            (Token::Number(l_value), Token::Number(r_value)) => l_value == r_value,
            (Token::Percent(l_value), Token::Percent(r_value)) => l_value == r_value,
            (Token::Operator(l_value), Token::Operator(r_value)) => l_value == r_value,
            (Token::Atom(l_value), _) => {
                match (&**l_value, other) {
                    (AtomType::Percent(_), Token::Percent(_)) => true,
                    (AtomType::Number(_), Token::Number(_)) => true,
                    (AtomType::Text(_), Token::Text(_)) => true,
                    (AtomType::Time(_), Token::Time(_)) => true,
                    (_, _) => false,
                }
            },
            (_, Token::Atom(r_value)) => {
                match (&**r_value, self) {
                    (AtomType::Percent(_), Token::Percent(_)) => true,
                    (AtomType::Number(_), Token::Number(_)) => true,
                    (AtomType::Text(_), Token::Text(_)) => true,
                    (AtomType::Time(_), Token::Time(_)) => true,
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

pub type TokenParser = fn(tokinizer: &mut Tokinizer) -> TokenParserResult;

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
    Atom(Rc<AtomType>),
    Percent(f64),
    Binary {
        left: Rc<BramaAstType>,
        operator: char,
        right: Rc<BramaAstType>
    },
    PrefixUnary(char, Rc<BramaAstType>),
    Assignment {
        index: usize,
        variable: Vec<Token>,
        expression: Rc<BramaAstType>
    },
    Symbol(String),
    Variable(usize)
}