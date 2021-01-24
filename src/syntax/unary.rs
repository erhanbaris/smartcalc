use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::util::map_parser;
use crate::syntax::primative::PrimativeParser;

pub struct UnaryParser;

impl SyntaxParserTrait for UnaryParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let ast = map_parser(parser, &[Self::parse_prefix_unary, PrimativeParser::parse])?;
        
        let index_backup = parser.get_index();
        parser.set_index(index_backup);
        return Ok(ast);
    }
}

impl UnaryParser {
    fn parse_prefix_unary(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();

        if let Some(operator) = parser.match_operator(&['-', '+']) {
            let token = &parser.peek_token().unwrap();

            let opt = match operator {
                '+'    => 1 as f64,
                '-' => -1 as f64,
                _ => 1 as f64
            };

            parser.consume_token();
            match token {
                Token::Number(double) => return Ok(BramaAstType::Number(double * opt)),
                _ => {
                    parser.set_index(index_backup);
                    return Err(("Unary works with number", 0, 0));
                }
            };
        }

        return Ok(BramaAstType::None);
    }
}