use crate::types::*;
use crate::syntax::{SyntaxParser};
use crate::syntax::ParseType;

pub fn map_parser(parser: &mut SyntaxParser, parser_funcs: &[ParseType]) -> AstResult {
    for parser_func in parser_funcs {
        match parser_func(parser) {
            Ok(BramaAstType::None) => (),
            Ok(ast) => return Ok(ast),
            Err(err) => return Err(err)
        }
    }

    Ok(BramaAstType::None)
}

pub fn is_ast_empty(ast: &AstResult) -> bool {
    match ast {
        Ok(BramaAstType::None) => true,
        Ok(_) => false,
        Err(_) => true
    }
}

pub fn err_or_message(ast: &AstResult, message: &'static str) -> AstResult {
    match &ast {
        Ok(BramaAstType::None) => Err((message, 0, 0,)),
        Ok(_) => Ok(BramaAstType::None),
        Err((m, l, c)) => Err((m, *l, *c))
    }
}
