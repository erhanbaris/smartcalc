/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::sync::Arc;

use crate::compiler::date::DateItem;
use crate::compiler::date_time::DateTimeItem;
use crate::compiler::duration::DurationItem;
use crate::compiler::memory::MemoryItem;
use crate::compiler::money::MoneyItem;
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
        
        {
            let session_mut = parser.session.borrow_mut();

            if token.is_err() {
                return Err(("No more token", 0, 0));
            }

            let second_index_backup  = parser.get_index();
            let mut closest_variable = usize::max_value();
            let mut variable_index   = 0;
            let mut variable_size    = 0;
            let mut found = false;
            let start = parser.index.get() as usize;

            for (index, variable) in session_mut.variables.iter().enumerate() {
                if let Some(start_index) = TokenType::is_same(&session_mut.tokens[start..].to_vec(), &variable.tokens) {
                    if (start_index == closest_variable && variable_size < variable.tokens.len()) || start_index < closest_variable {
                        closest_variable = start_index;
                        variable_index   = index;
                        variable_size    = variable.tokens.len();
                        found = true;
                    }
                }
            }

            if found {
                let target_index = start + closest_variable as usize + variable_size;
                parser.index.set(target_index);
                return Ok(SmartCalcAstType::Variable(session_mut.variables[variable_index].clone()));
            }

            parser.set_index(second_index_backup);
        }

        let result = match token.unwrap().deref() {
            TokenType::Text(_)   => {
                parser.consume_token();
                return Ok(SmartCalcAstType::None);
            },
            TokenType::Money(price, currency)     => Ok(SmartCalcAstType::Item(Arc::new(MoneyItem(*price, currency.clone())))),
            TokenType::Number(double)     => Ok(SmartCalcAstType::Item(Arc::new(NumberItem(*double)))),
            TokenType::Memory(memory, memory_type)     => Ok(SmartCalcAstType::Item(Arc::new(MemoryItem(*memory, memory_type.clone())))),
            TokenType::Field(field_type)  => Ok(SmartCalcAstType::Field(field_type.clone())),
            TokenType::Percent(percent)   => Ok(SmartCalcAstType::Item(Arc::new(PercentItem(*percent)))),
            TokenType::Time(time, tz)         => Ok(SmartCalcAstType::Item(Arc::new(TimeItem(*time, tz.clone())))),
            TokenType::Date(date, tz)         => Ok(SmartCalcAstType::Item(Arc::new(DateItem(*date, tz.clone())))),
            TokenType::DateTime(date_time, tz)         => Ok(SmartCalcAstType::Item(Arc::new(DateTimeItem(*date_time, tz.clone())))),
            TokenType::Duration(duration)         => Ok(SmartCalcAstType::Item(Arc::new(DurationItem(*duration)))),
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