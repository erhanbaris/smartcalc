use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::expression::ExpressionParser;
use std::rc::Rc;

pub struct AssignmentParser;

impl SyntaxParserTrait for AssignmentParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let index_backup      = parser.get_index();
        let mut assignment_index: Option<usize> = None;

        for (index, token) in parser.tokens.iter().enumerate() {
            match token {
                Token::Operator('=') => {
                    assignment_index = Some(index);
                    break;
                },
                _ => ()
            };
        }

        if assignment_index.is_some() {
            let start = parser.get_index();
            let end;

            loop {
                match parser.consume_token() {
                    Some(token) => {
                        match token {
                            Token::Operator(operator) => {
                                if *operator == '=' {
                                    parser.consume_token();
                                    break;
                                }
                            }
                            _ => ()
                        };
                    },
                     _ => break
                };
            }

            end = parser.get_index() - 1;

            let expression = ExpressionParser::parse(parser);
            match expression {
                Ok(BramaAstType::None) => return expression,
                Ok(_)  => (),
                Err(_) => return expression
            };

            let variable       = parser.tokens[start..end].to_vec();
            let assignment_ast = BramaAstType::Assignment {
                index: parser.variables.borrow().len(),
                variable: variable.to_vec(),
                expression: Rc::new(expression.unwrap())
            };

            parser.variables.borrow_mut().push(variable);
            return Ok(assignment_ast);
        }
        parser.set_index(index_backup);
        Ok(BramaAstType::None)
    }
}
