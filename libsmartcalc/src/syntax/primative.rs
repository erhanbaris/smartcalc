use alloc::string::ToString;
use crate::types::*;
use crate::syntax::util::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::binary::AddSubtractParser;

pub struct PrimativeParser;

impl PrimativeParser {
    pub fn parse_basic_primatives(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();

        let token = parser.peek_token();
        if token.is_err() {
            return Err(("No more token", 0, 0));
        }

        let second_index_backup  = parser.get_index();
        let mut closest_variable = usize::max_value();
        let mut variable_index   = 0;
        let mut variable_size    = 0;
        let mut found = false;
        let start = parser.index.get() as usize;

        for (index, variable) in parser.storage.variables.borrow().iter().enumerate() {
            if let Some(start_index) = TokenType::is_same(&parser.tokens[start..].to_vec(), &variable.tokens) {
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
            return Ok(BramaAstType::Variable(parser.storage.variables.borrow()[variable_index].clone()));
        }

        parser.set_index(second_index_backup);

        let result = match &token.unwrap() {
            TokenType::Text(_)  => {
                parser.consume_token();
                return Ok(BramaAstType::None);
            },
            TokenType::Money(price, currency)     => Ok(BramaAstType::Money(*price, currency.to_string())),
            TokenType::Number(double)     => Ok(BramaAstType::Number(*double)),
            TokenType::Field(field_type)  => Ok(BramaAstType::Field(field_type.clone())),
            TokenType::Percent(percent)   => Ok(BramaAstType::Percent(*percent)),
            TokenType::Time(time)         => Ok(BramaAstType::Time(*time)),
            TokenType::Date(date)         => Ok(BramaAstType::Date(*date)),
            TokenType::Duration(duration)         => Ok(BramaAstType::Duration(*duration)),
            TokenType::Variable(variable) => Ok(BramaAstType::Variable(variable.clone())),
            _ => {
                parser.consume_token();
                return Err(("No more token", 0, 0));
            }
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
        Ok(BramaAstType::None)
    }
}

impl SyntaxParserTrait for PrimativeParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        map_parser(parser, &[Self::parse_parenthesis, Self::parse_basic_primatives])
    }
}