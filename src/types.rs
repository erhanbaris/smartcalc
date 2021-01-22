use std::vec::Vec;
use std::result::Result;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::rc::Rc;
use std::collections::HashMap;

pub type ParseResult        = Result<Vec<Token>, (&'static str, u32, u32)>;
pub type ExpressionFunc     = fn(stack: &HashMap<String, String>) -> Option<()>;
pub type TokenParserResult  = Result<bool, (&'static str, u32)>;
pub type AstResult          = Result<BramaAstType, (&'static str, u32, u32)>;


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
pub enum BramaTokenType {
    Number(f64),
    Symbol(Rc<String>),
    Operator(char),
    Atom(Rc<AtomType>),
    Percent(f64)
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
pub struct Token {
    pub start    : u32,
    pub end    : u32,
    pub token_type: BramaTokenType
}

pub struct Tokinizer {
    pub line  : u32,
    pub column: u32,
    pub tokens: Vec<Token>,
    pub iter: Vec<char>,
    pub data: String,
    pub index: u32,
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

    pub fn get_indexer(&self) -> usize {
        self.indexer
    }

    pub fn set_indexer(&mut self, indexer: usize) {
        self.indexer = indexer;
    }

    pub fn add_token(&mut self, start: u32, token_type: BramaTokenType) {
        let token = Token {
            start,
            end: self.column,
            token_type
        };
        self.tokens.push(token);
    }

    pub fn increase_index(&mut self) {
        self.index  += self.get_char().len_utf8() as u32;
        self.indexer += 1;
        self.column += 1;
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

pub trait StrTrait {
    fn atom(&self) -> u64;
}

impl StrTrait for str {
    fn atom(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
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
        left: Box<BramaAstType>,
        operator: char,
        right: Box<BramaAstType>
    },
    PrefixUnary(char, Box<BramaAstType>),
    Assignment {
        variable: Box<BramaAstType>,
        expression: Box<BramaAstType>
    },
    Symbol(String),
}