use std::vec::Vec;
use std::result::Result;
use std::rc::Rc;
use std::collections::HashMap;

pub type ParseResult        = Result<Vec<Token>, (&'static str, u16, u16)>;
pub type ExpressionFunc     = fn(stack: &HashMap<String, String>) -> Option<()>;
pub type TokenParserResult  = Result<bool, (&'static str, u16)>;
pub type AstResult          = Result<BramaAstType, (&'static str, u16, u16)>;
pub type AliasYaml          = HashMap<String, Vec<String>>;
pub type AliasYamlCollection= HashMap<String, AliasYaml>;

pub type AliasList          = HashMap<String, String>;
pub type AliasCollection    = HashMap<String, AliasList>;


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
#[derive(PartialEq)]
pub enum Token {
    Number(f64),
    Text(Rc<String>),
    Operator(char),
    Atom(Rc<AtomType>),
    Percent(f64)
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