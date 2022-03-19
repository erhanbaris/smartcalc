/*
 * smartcalc v1.0.7
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */


use alloc::rc::Rc;

use crate::compiler::date::DateItem;
use crate::compiler::date_time::DateTimeItem;
use crate::compiler::duration::DurationItem;
use crate::compiler::money::MoneyItem;
use crate::compiler::dynamic_type::DynamicTypeItem;
use crate::compiler::number::NumberItem;
use crate::compiler::percent::PercentItem;
use crate::compiler::time::TimeItem;
use crate::types::*;
use crate::syntax::util::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::binary::AddSubtractParser;
use core::ops::Deref;

pub struct PrimativeParser;

impl PrimativeParser {
    pub fn parse_basic_primatives(parser: &mut SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        let token = parser.peek_token();

        if token.is_err() {
            return Err(("No more token", 0, 0));
        }

        let result = match token.unwrap().deref() {
            TokenType::Timezone(_, _) |
            TokenType::Text(_)   => {
                parser.consume_token();
                return Ok(SmartCalcAstType::None);
            },
            TokenType::DynamicType(number, dynamic_type)     => Ok(SmartCalcAstType::Item(Rc::new(DynamicTypeItem(*number, dynamic_type.clone())))),
            TokenType::Money(price, currency)     => Ok(SmartCalcAstType::Item(Rc::new(MoneyItem(*price, currency.clone())))),
            TokenType::Number(double, number_type)     => Ok(SmartCalcAstType::Item(Rc::new(NumberItem(*double, *number_type)))),
            TokenType::Field(field_type)  => Ok(SmartCalcAstType::Field(field_type.clone())),
            TokenType::Percent(percent)   => Ok(SmartCalcAstType::Item(Rc::new(PercentItem(*percent)))),
            TokenType::Time(time, tz)         => Ok(SmartCalcAstType::Item(Rc::new(TimeItem(*time, tz.clone())))),
            TokenType::Date(date, tz)         => Ok(SmartCalcAstType::Item(Rc::new(DateItem(*date, tz.clone())))),
            TokenType::DateTime(date_time, tz)         => Ok(SmartCalcAstType::Item(Rc::new(DateTimeItem(*date_time, tz.clone())))),
            TokenType::Duration(duration)         => Ok(SmartCalcAstType::Item(Rc::new(DurationItem(*duration)))),
            TokenType::Variable(variable) => Ok(SmartCalcAstType::Variable(variable.clone())),
            _ => {
                parser.consume_token();
                return Err(("No more token", 0, 0));
            }
        };

        match result {
            Ok(SmartCalcAstType::None) => {
                parser.set_index(index_backup);
                Ok(SmartCalcAstType::None)
            },
            Ok(ast) => {
                parser.consume_token();
                Ok(ast)
            },
            Err((message, line, column)) => Err((message, line, column))
        }
    }

    pub fn parse_parenthesis(parser: &mut SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        if parser.match_operator(&['(']).is_some() {
            
            let ast = AddSubtractParser::parse(parser);
            if is_ast_empty(&ast) {
                parser.set_index(index_backup);
                return err_or_message(&ast, "Invalid expression");
            }

            if parser.match_operator(&[')']).is_none() {
                parser.set_index(index_backup);
                return Err(("Parentheses not closed", 0, 0));
            }

            return Ok(ast.unwrap());
        }

        parser.set_index(index_backup);
        Ok(SmartCalcAstType::None)
    }
}

impl SyntaxParserTrait for PrimativeParser {
    fn parse(parser: &mut SyntaxParser) -> AstResult {
        map_parser(parser, &[Self::parse_parenthesis, Self::parse_basic_primatives])
    }
}