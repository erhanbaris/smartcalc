pub mod primative;
pub mod unary;
pub mod util;
pub mod binary;
pub mod assignment;
pub mod statement;
pub mod expression;

use std::vec::Vec;
use std::cell::Cell;

use crate::syntax::statement::StatementParser;
use crate::syntax::expression::ExpressionParser;
use crate::syntax::util::map_parser;

use crate::types::*;

pub type ParseType = fn(parser: &SyntaxParser) -> AstResult;

pub struct SyntaxParser {
    pub tokens: Box<Vec<Token>>,
    pub index: Cell<usize>
}

pub trait SyntaxParserTrait {
    fn parse(parser: &SyntaxParser) -> AstResult;
}

pub trait ExtensionSyntaxParser: Sized {
    fn parsable    (parser: &SyntaxParser) -> bool;
    fn parse_suffix(ast: Box<BramaAstType>, parser: &SyntaxParser) -> AstResult;
}

impl SyntaxParser {
    pub fn new(tokens: Box<Vec<Token>>) -> SyntaxParser {
        SyntaxParser {
            tokens,
            index: Cell::new(0)
        }
    }

    pub fn parse(&self) -> AstResult {
        let ast = map_parser(self, &[StatementParser::parse, ExpressionParser::parse])?;
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

        return None;
    }

    fn check_operator(&self, operator: char) -> bool {
        match self.peek_token() {
            Ok(token) => {
                match token.token_type {
                    BramaTokenType::Operator(token_operator) => {
                        println!("{} == {} {}", operator, token_operator, operator == token_operator);
                        operator == token_operator
                    },
                    _ => false
                }
            },
            _ => false
        }
    }
}