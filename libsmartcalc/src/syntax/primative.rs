use crate::types::*;
use crate::syntax::util::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::binary::AddSubtractParser;
use std::rc::Rc;

pub struct PrimativeParser;

impl PrimativeParser {
    pub fn parse_basic_primatives(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();

        let token = parser.peek_token();
        if token.is_err() {
            return Ok(BramaAstType::None);
        }

        let second_index_backup  = parser.get_index();
        let mut closest_variable = usize::max_value();
        let mut variable_index   = 0;
        let mut variable_size    = 0;
        let mut found = false;
        let start = parser.index.get() as usize;

        for (index, variable) in parser.storage.variables.borrow().iter().enumerate() {
            if let Some(start_index) = Token::is_same(&parser.tokens[start..].to_vec(), &variable.tokens) {
                if start_index == closest_variable && variable_size < variable.tokens.len() {
                    closest_variable = start_index;
                    variable_index   = index;
                    variable_size    = variable.tokens.len();
                    found = true;
                }
                else if start_index < closest_variable {
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
            return Ok(BramaAstType::Variable(parser.storage.variables.borrow()[variable_index].clone()));
        }

        parser.set_index(second_index_backup);

        let result = match &token.unwrap() {
            Token::Text(text)  => {
                parser.consume_token();
                return Ok(BramaAstType::None);
            },
            Token::Number(double)     => Ok(BramaAstType::Number(*double)),
            Token::Field(field_type)  => Ok(BramaAstType::Field(field_type.clone())),
            Token::Percent(percent)   => Ok(BramaAstType::Percent(*percent)),
            Token::Time(time)         => Ok(BramaAstType::Time(*time)),
            Token::Variable(variable) => Ok(BramaAstType::Variable(variable.clone())),
            _ => Ok(BramaAstType::None)
        };

        match result {
            Ok(BramaAstType::None) => {
                parser.set_index(index_backup);
                Ok(BramaAstType::None)
            },
            Ok(ast) => {
                parser.consume_token();
                Ok(ast)
            },
            Err((message, line, column)) => Err((message, line, column))
        }
    }

    pub fn parse_parenthesis(parser: &SyntaxParser) -> AstResult {
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
        return Ok(BramaAstType::None);
    }
}

impl SyntaxParserTrait for PrimativeParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        return map_parser(parser, &[Self::parse_parenthesis, Self::parse_basic_primatives]);
    }
}