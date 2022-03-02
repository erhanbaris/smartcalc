/*
 * smartcalc v1.0.4
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

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
