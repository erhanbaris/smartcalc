/*
 * smartcalc v1.0.7
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use crate::types::*;
use crate::syntax::{SyntaxParser};
use crate::syntax::ParseType;

pub fn map_parser(parser: &mut SyntaxParser, parser_funcs: &[ParseType]) -> AstResult {
    for parser_func in parser_funcs {
        match parser_func(parser) {
            Ok(SmartCalcAstType::None) => (),
            Ok(ast) => return Ok(ast),
            Err(err) => return Err(err)
        }
    }

    Ok(SmartCalcAstType::None)
}

pub fn is_ast_empty(ast: &AstResult) -> bool {
    match ast {
        Ok(SmartCalcAstType::None) => true,
        Ok(_) => false,
        Err(_) => true
    }
}

pub fn err_or_message(ast: &AstResult, message: &'static str) -> AstResult {
    match &ast {
        Ok(SmartCalcAstType::None) => Err((message, 0, 0,)),
        Ok(_) => Ok(SmartCalcAstType::None),
        Err((m, l, c)) => Err((m, *l, *c))
    }
}
