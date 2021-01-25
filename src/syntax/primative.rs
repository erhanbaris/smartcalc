use crate::types::*;
use crate::syntax::util::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::expression::ExpressionParser;

pub struct PrimativeParser;

impl PrimativeParser {
    pub fn parse_basic_primatives(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();

        let token = parser.peek_token();
        if token.is_err() {
            return Ok(BramaAstType::None);
        }

        let result = match &token.unwrap() {
            Token::Number(double)  => Ok(BramaAstType::Number(*double)),
            Token::Atom(atom_type) => Ok(BramaAstType::Atom(atom_type.clone())),
            Token::Percent(percent) => Ok(BramaAstType::Percent(*percent)),
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

    pub fn parse_symbol(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        let token = parser.peek_token();
        if token.is_err() {
            return Ok(BramaAstType::None);
        }

        if let Token::Text(_) = &token.unwrap() {
            let second_index_backup = parser.get_index();
            for (variable_index, variable) in parser.variables.borrow().iter().enumerate() {
                let mut not_same = false;
                for (index, variable_item) in variable.iter().enumerate() {
                    if variable_item != parser.peek_token().unwrap() {
                        not_same = true;
                        break
                    }

                    match parser.consume_token() {
                        Some(_) => (),
                        _ => {
                            if variable.len()-1 != index {
                                not_same = true;
                            }
                            break
                        }
                    }
                }

                if !not_same {
                    println!("# Found");
                    return Ok(BramaAstType::Variable(variable_index));
                }
            }

            parser.set_index(second_index_backup);
            parser.consume_token();
            return Ok(BramaAstType::None);
        }

        parser.set_index(index_backup);
        return Ok(BramaAstType::None);
    }

    pub fn parse_parenthesis(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        if parser.match_operator(&['(']).is_some() {
            
            let ast = ExpressionParser::parse(parser);
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
        return map_parser(parser, &[Self::parse_parenthesis, Self::parse_symbol, Self::parse_basic_primatives]);
    }
}