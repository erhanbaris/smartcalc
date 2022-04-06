/*
 * smartcalc v1.0.8
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::cell::RefCell;
use alloc::string::String;
use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::variable::VariableInfo;
use alloc::rc::Rc;
use crate::syntax::binary::AddSubtractParser;
use core::ops::Deref;
use crate::alloc::string::ToString;

pub struct AssignmentParser;

impl SyntaxParserTrait for AssignmentParser {
    fn parse(parser: &mut SyntaxParser) -> AstResult {
        let index_backup      = parser.get_index();
        let mut assignment_index: Option<usize> = None;

        for (index, token) in parser.tokinizer.tokens.iter().enumerate() {
            if let TokenType::Operator('=') = token.deref() {
                assignment_index = Some(index);
                break;
            }
        }

        if assignment_index.is_some() {
            let start = parser.get_index();
            let end;
            let mut variable_name = String::new();
            variable_name.push_str(&parser.peek_token().unwrap().to_string().to_lowercase()[..]);
            
            while let Some(token) = parser.consume_token() {
                match token.deref() {
                    TokenType::Operator(operator) => {
                        if *operator == '=' {
                            parser.consume_token();
                            break;
                        }
                    }
                    _ => variable_name.push_str(&token.to_string().to_lowercase()[..])
                };
            }

            end = parser.get_index() - 1;

            let expression = AddSubtractParser::parse(parser);
            match expression {
                Ok(SmartCalcAstType::None) => return expression,
                Ok(_)  => (),
                Err(_) => return expression
            };

            let variable_exist = parser.session.variables.borrow().contains_key(&variable_name);
            
            let variable = match variable_exist {
                true => parser.session.variables.borrow().get(&variable_name).unwrap().clone(),
                false => {
                    let variable = Rc::new(VariableInfo {
                        tokens: parser.tokinizer.tokens[start..end].to_vec(),
                        data: RefCell::new(Rc::new(SmartCalcAstType::None))
                    });
        
                    parser.session.add_variable(variable.clone());
                    variable
                }
            };
            
            let assignment_ast = SmartCalcAstType::Assignment {
                variable,
                expression: Rc::new(expression.unwrap())
            };


            return Ok(assignment_ast);
        }
        parser.set_index(index_backup);
        Ok(SmartCalcAstType::None)
    }
}
