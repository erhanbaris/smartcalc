/*
 * smartcalc v1.0.5
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
                        TokenType::Number(double, number_type)         => return Ok(SmartCalcAstType::Item(Rc::new(NumberItem(double * opt, *number_type)))),
                        TokenType::Variable(variable)     => return Ok(SmartCalcAstType::PrefixUnary(operator, Rc::new(SmartCalcAstType::Variable(variable.clone())))),
                        TokenType::Percent(percent)       => return Ok(SmartCalcAstType::PrefixUnary(operator, Rc::new(SmartCalcAstType::Item(Rc::new(PercentItem(*percent)))))),
                        TokenType::Money(money, currency) => return Ok(SmartCalcAstType::PrefixUnary(operator, Rc::new(SmartCalcAstType::PrefixUnary(operator, Rc::new(SmartCalcAstType::Item(Rc::new(MoneyItem(*money, currency.clone())))))))),
                        _ => {
                            parser.set_index(index_backup);
                            return Err(("Unary works with number", 0, 0));
                        }
                    };
                },
                 _=> return Ok(SmartCalcAstType::None)
            }
        }

        Ok(SmartCalcAstType::None)
    }
}