/*
 * smartcalc v1.0.2
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

pub mod primative;
pub mod unary;
pub mod util;
pub mod binary;
pub mod assignment;
pub mod statement;

use core::cell::{Cell, RefCell};

use crate::syntax::util::map_parser;

use crate::types::*;
use alloc::rc::Rc;
use crate::app::Session;
use crate::syntax::assignment::AssignmentParser;
use crate::syntax::binary::AddSubtractParser;
use core::ops::Deref;

pub type ParseType = fn(parser: &mut SyntaxParser) -> AstResult;

pub struct SyntaxParser<'a> {
    pub index: Cell<usize>,
    pub session: &'a RefCell<Session>
}

pub trait SyntaxParserTrait {
    fn parse(parser: &mut SyntaxParser) -> AstResult;
}

impl<'a> SyntaxParser<'a> {
    pub fn new(session: &'a RefCell<Session>) -> SyntaxParser {
        SyntaxParser {
            index: Cell::new(0),
            session
        }
    }

    pub fn parse(&mut self) -> AstResult {
        let ast = map_parser(self, &[AssignmentParser::parse, AddSubtractParser::parse])?;
        Ok(ast)
    }

    pub fn set_index(&self, index: usize) {
        self.index.set(index);
    }

    pub fn get_index(&self) -> usize {
        self.index.get()
    }

    #[allow(clippy::result_unit_err)]
    pub fn peek_token(&self) -> Result<Rc<TokenType>, ()> {
        match self.session.borrow().tokens.get(self.index.get()) {
            Some(token) => Ok(token.clone()),
            None => Err(())
        }
    }

    pub fn consume_token(&self) -> Option<Rc<TokenType>> {
        self.index.set(self.index.get() + 1);
        match self.session.borrow().tokens.get(self.index.get()) {
            Some(token) => Some(token.clone()),
            None => None
        }
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
                match token.deref() {
                    TokenType::Operator(token_operator) => operator == *token_operator,
                    _ => false
                }
            },
            _ => false
        }
    }
}