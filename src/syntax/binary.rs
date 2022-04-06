/*
 * smartcalc v1.0.8
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::unary::UnaryParser;
use alloc::rc::Rc;

pub struct ModuloParser;
pub struct MultiplyDivideParser;
pub struct AddSubtractParser;

impl SyntaxParserTrait for ModuloParser {
    fn parse(parser: &mut SyntaxParser) -> AstResult {
        parse_binary::<MultiplyDivideParser>(parser, &['%'])
    }
}

impl SyntaxParserTrait for MultiplyDivideParser {
    fn parse(parser: &mut SyntaxParser) -> AstResult {
        parse_binary::<UnaryParser>(parser, &['*', '/'])
    }
}

impl SyntaxParserTrait for AddSubtractParser {
    fn parse(parser: &mut SyntaxParser) -> AstResult {
        parse_binary::<ModuloParser>(parser, &['+', '-'])
    }
}

pub fn parse_binary<T: SyntaxParserTrait>(parser: &mut SyntaxParser, operators: &[char]) -> AstResult {
    let mut left_expr = T::parse(parser)?;
    
    if let SmartCalcAstType::None = left_expr {
        return Ok(left_expr)
    }

    loop {
        let index_backup = parser.get_index();

        if let Some(operator) = parser.match_operator(operators) {
            loop {
                let right_expr = T::parse(parser);
                match right_expr {
                    Ok(SmartCalcAstType::None) => (),
                    Ok(_) => {
                        left_expr = SmartCalcAstType::Binary {
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
