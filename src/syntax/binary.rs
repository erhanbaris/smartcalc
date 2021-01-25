use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::unary::UnaryParser;
use std::rc::Rc;

pub struct ModuloParser;
pub struct MultiplyDivideParser;
pub struct AddSubtractParser;

impl SyntaxParserTrait for ModuloParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_binary::<MultiplyDivideParser>(parser, &['%']);
    }
}

impl SyntaxParserTrait for MultiplyDivideParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        return parse_binary::<UnaryParser>(parser, &['*', '/']);
    }
}

impl SyntaxParserTrait for AddSubtractParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        parse_binary::<ModuloParser>(parser, &['+', '-'])
    }
}

pub fn parse_binary<T: SyntaxParserTrait>(parser: &SyntaxParser, operators: &[char]) -> AstResult {
    let mut left_expr = T::parse(parser)?;
    match left_expr {
        BramaAstType::None => return Ok(left_expr),
        _ => ()
    };

    let mut left_assignment_done = false;
    loop {
        let index_backup = parser.get_index();

        if let Some(operator) = parser.match_operator(operators) {
            loop {
                let right_expr = T::parse(parser);
                match right_expr {
                    Ok(BramaAstType::None) => (),
                    Ok(_) => {
                        left_expr = BramaAstType::Binary {
                            left: Rc::new(left_expr),
                            operator,
                            right: Rc::new(right_expr.unwrap())
                        };
                        break;
                    },
                    Err(_) => return right_expr
                };
            }
        }
        else {
            parser.set_index(index_backup);
            break;
        }
    }

    Ok(left_expr)
}
