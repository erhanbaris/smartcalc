pub mod primative;
pub mod unary;
pub mod util;
pub mod binary;
pub mod assignment;
pub mod statement;

use std::vec::Vec;
use std::cell::{Cell};

use crate::syntax::util::map_parser;

use crate::types::*;
use std::rc::Rc;
use crate::executer::Storage;
use crate::syntax::assignment::AssignmentParser;
use crate::syntax::binary::AddSubtractParser;

pub type ParseType = fn(parser: &SyntaxParser) -> AstResult;

pub struct SyntaxParser {
    pub tokens: Rc<Vec<Token>>,
    pub index: Cell<usize>,
    pub storage: Rc<Storage>
}

pub trait SyntaxParserTrait {
    fn parse(parser: &SyntaxParser) -> AstResult;
}

impl SyntaxParser {
    pub fn new(tokens: Rc<Vec<Token>>, storage: Rc<Storage>) -> SyntaxParser {
        SyntaxParser {
            tokens,
            index: Cell::new(0),
            storage
        }
    }

    pub fn parse(&self) -> AstResult {
        let ast = map_parser(self, &[AssignmentParser::parse, AddSubtractParser::parse])?;
        return Ok(ast);
    }

    pub fn set_index(&self, index: usize) {
        self.index.set(index);
    }

    pub fn get_index(&self) -> usize {
        self.index.get()
    }

    pub fn peek_token(&self) -> Result<&Token, ()> {
        match self.tokens.get(self.index.get()) {
            Some(token) => Ok(token),
            None => Err(())
        }
    }

    #[allow(dead_code)]
    pub fn next_token(&self) -> Result<&Token, ()> {
        match self.tokens.get(self.index.get() + 1) {
            Some(token) => Ok(token),
            None => Err(())
        }
    }
    
    pub fn consume_token(&self) -> Option<&Token> {
        self.index.set(self.index.get() + 1);
        self.tokens.get(self.index.get())
    }

    fn match_operator(&self, operators: &[char]) -> Option<char> {
        for operator in operators {
            if self.check_operator(*operator) {
                self.consume_token();
                return Some(*operator);
            }
        }

        None
    }

    fn check_operator(&self, operator: char) -> bool {
        match self.peek_token() {
            Ok(token) => {
                match token.token {
                    TokenType::Operator(token_operator) => {
                        operator == token_operator
                    },
                    _ => false
                }
            },
            _ => false
        }
    }
}