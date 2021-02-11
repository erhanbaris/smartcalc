use alloc::string::String;
use alloc::string::ToString;
use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use alloc::rc::Rc;
use crate::syntax::binary::AddSubtractParser;

pub struct AssignmentParser;

impl SyntaxParserTrait for AssignmentParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let index_backup      = parser.get_index();
        let mut assignment_index: Option<usize> = None;

        for (index, token) in parser.tokens.iter().enumerate() {
            match token.token {
                TokenType::Operator('=') => {
                    assignment_index = Some(index);
                    break;
                },
                _ => ()
            };
        }

        if assignment_index.is_some() {
            let start = parser.get_index();
            let end;
            let mut variable_name = String::new();
            variable_name.push_str(&parser.peek_token().unwrap().to_string()[..]);

            loop {
                match parser.consume_token() {
                    Some(token) => {
                        match token.token {
                            TokenType::Operator(operator) => {
                                if operator == '=' {
                                    parser.consume_token();
                                    break;
                                }
                            }
                            _ => variable_name.push_str(&token.to_string()[..])
                        };
                    },
                     _ => break
                };
            }

            end = parser.get_index() - 1;

            let expression = AddSubtractParser::parse(parser);
            match expression {
                Ok(BramaAstType::None) => return expression,
                Ok(_)  => (),
                Err(_) => return expression
            };

            let mut index = parser.storage.variables.borrow().len();
            let mut new_variable = true;

            if let Some(data) = parser.storage.variables.borrow().iter().find(|&s| s.name == variable_name) {
                index = data.index;
                new_variable = true;
            }

            let variable_info = VariableInfo {
                tokens: parser.tokens[start..end].to_vec(),
                index: index,
                data: Rc::new(BramaAstType::None),
                name: variable_name.to_string()
            };

            let assignment_ast = BramaAstType::Assignment {
                index: variable_info.index,
                expression: Rc::new(expression.unwrap())
            };

            if new_variable {
                parser.storage.variables.borrow_mut().push(Rc::new(variable_info));
            }

            return Ok(assignment_ast);
        }
        parser.set_index(index_backup);
        Ok(BramaAstType::None)
    }
}
