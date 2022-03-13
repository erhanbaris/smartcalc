/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::cell::RefCell;

use alloc::string::String;
use alloc::string::ToString;
use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::variable::VariableInfo;
use alloc::rc::Rc;
use crate::syntax::binary::AddSubtractParser;
use core::ops::Deref;

pub struct AssignmentParser;

impl SyntaxParserTrait for AssignmentParser {
    fn parse(parser: &mut SyntaxParser) -> AstResult {
        let index_backup      = parser.get_index();
        let mut assignment_index: Option<usize> = None;

        for (index, token) in parser.session.borrow().tokens.iter().enumerate() {
            if let TokenType::Operator('=') = token.deref() {
                assignment_index = Some(index);
                break;
            }
        }

        if assignment_index.is_some() {
            let start = parser.get_index();
            let end;
            let mut variable_name = String::new();
            variable_name.push_str(&parser.peek_token().unwrap().to_string()[..]);
            
            while let Some(token) = parser.consume_token() {
                match token.deref() {
                    TokenType::Operator(operator) => {
                        if *operator == '=' {
                            parser.consume_token();
                            break;
                        }
                    }
                    _ => variable_name.push_str(&token.to_string()[..])
                };
            }

            end = parser.get_index() - 1;

            let expression = AddSubtractParser::parse(parser);
            match expression {
                Ok(SmartCalcAstType::None) => return expression,
                Ok(_)  => (),
                Err(_) => return expression
            };
            
            let mut session_mut = parser.session.borrow_mut();
            let mut index = session_mut.variables.len();
            let mut new_variable = true;

            if let Some(data) = session_mut.variables.iter().find(|&s| s.name == variable_name) {
                index = data.index;
                new_variable = true;
            }

            let variable_info = VariableInfo {
                tokens: session_mut.tokens[start..end].to_vec(),
                index,
                data: RefCell::new(Rc::new(SmartCalcAstType::None)),
                name: variable_name
            };

            let assignment_ast = SmartCalcAstType::Assignment {
                index: variable_info.index,
                expression: Rc::new(expression.unwrap())
            };

            if new_variable {
                session_mut.add_variable(Rc::new(variable_info));
            }

            return Ok(assignment_ast);
        }
        parser.set_index(index_backup);
        Ok(SmartCalcAstType::None)
    }
}
