use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::util::map_parser;
use crate::syntax::primative::PrimativeParser;
use alloc::rc::Rc;

pub struct UnaryParser;

impl SyntaxParserTrait for UnaryParser {
    fn parse(parser: &mut SyntaxParser) -> AstResult {
        let ast = map_parser(parser, &[Self::parse_prefix_unary, PrimativeParser::parse])?;
        
        let index_backup = parser.get_index();
        parser.set_index(index_backup);
        Ok(ast)
    }
}

impl UnaryParser {
    fn parse_prefix_unary(parser: &mut SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();

        if let Some(operator) = parser.match_operator(&['-', '+']) {
            match parser.peek_token() {
                Ok(token) => {
                    let opt = match operator {
                        '+' => 1_f64,
                        '-' => -1_f64,
                        _   => 1_f64
                    };

                    match &*token {
                        TokenType::Number(double)         => return Ok(BramaAstType::Number(double * opt)),
                        TokenType::Variable(variable)     => return Ok(BramaAstType::PrefixUnary(operator, Rc::new(BramaAstType::Variable(variable.clone())))),
                        TokenType::Percent(percent)       => return Ok(BramaAstType::PrefixUnary(operator, Rc::new(BramaAstType::Percent(*percent)))),
                        //TokenType::Money(money, currency) => return Ok(BramaAstType::PrefixUnary(operator, Rc::new(BramaAstType::Money(*money, currency.clone())))),
                        _ => {
                            parser.set_index(index_backup);
                            return Err(("Unary works with number", 0, 0));
                        }
                    };
                },
                 _=> return Ok(BramaAstType::None)
            }
        }

        Ok(BramaAstType::None)
    }
}