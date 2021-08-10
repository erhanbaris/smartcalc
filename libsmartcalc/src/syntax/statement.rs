use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::util::map_parser;
use crate::syntax::assignment::AssignmentParser;

pub struct StatementParser;

impl SyntaxParserTrait for StatementParser {
    fn parse(parser: &mut SyntaxParser) -> AstResult {
        map_parser(parser, &[AssignmentParser::parse])
    }
}
