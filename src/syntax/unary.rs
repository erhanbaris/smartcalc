/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use crate::compiler::money::MoneyItem;
use crate::compiler::number::NumberItem;
use crate::compiler::percent::PercentItem;
use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::util::map_parser;
use crate::syntax::primative::PrimativeParser;
use core::ops::Deref;
use alloc::rc::Rc;
use alloc::sync::Arc;

pub struct UnaryParser;

impl SyntaxParserTrait for UnaryParser {
    fn parse(parser: &mut SyntaxParser) -> AstResult {
        let ast = map_parser(parser, &[Self::parse_prefix_unary, PrimativeParser::parse])?;
        
        let index_backup = parser.get_index();
        parser.set_index(index_backup);
        Ok(ast)
    }
}

impl UnaryParser {
    fn parse_prefix_unary(parser: &mut SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();

        if let Some(operator) = parser.match_operator(&['-', '+']) {
            match parser.peek_token() {
                Ok(token) => {
                    let opt = match operator {
                        '+' => 1_f64,
                        '-' => -1_f64,
                        _   => 1_f64
                    };

                    match token.deref() {
                        TokenType::Number(double)         => return Ok(BramaAstType::Item(Arc::new(NumberItem(double * opt)))),
                        TokenType::Variable(variable)     => return Ok(BramaAstType::PrefixUnary(operator, Rc::new(BramaAstType::Variable(variable.clone())))),
                        TokenType::Percent(percent)       => return Ok(BramaAstType::PrefixUnary(operator, Rc::new(BramaAstType::Item(Arc::new(PercentItem(*percent)))))),
                        TokenType::Money(money, currency) => return Ok(BramaAstType::PrefixUnary(operator, Rc::new(BramaAstType::PrefixUnary(operator, Rc::new(BramaAstType::Item(Arc::new(MoneyItem(*money, currency.clone())))))))),
                        _ => {
                            parser.set_index(index_backup);
                            return Err(("Unary works with number", 0, 0));
                        }
                    };
                },
                 _=> return Ok(BramaAstType::None)
            }
        }

        Ok(BramaAstType::None)
    }
}