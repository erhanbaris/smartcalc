use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::binary::AddSubtractParser;

pub struct ExpressionParser;

impl SyntaxParserTrait for ExpressionParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let ast = AddSubtractParser::parse(parser)?;
        Ok(ast)
    }
}
