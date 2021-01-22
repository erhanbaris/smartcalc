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

        let result = match &token.unwrap().token_type {
            BramaTokenType::Number(double)  => Ok(BramaAstType::Number(*double)),
            BramaTokenType::Atom(atom_type) => Ok(BramaAstType::Atom(atom_type.clone())),
            BramaTokenType::Percent(percent) => Ok(BramaAstType::Percent(*percent)),
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

        if let BramaTokenType::Symbol(symbol) = &token.unwrap().token_type {
            parser.consume_token();
            return Ok(BramaAstType::Symbol(symbol.to_string()));
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