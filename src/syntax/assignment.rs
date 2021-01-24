use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::expression::ExpressionParser;
use std::rc::Rc;

pub struct AssignmentParser;

impl SyntaxParserTrait for AssignmentParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();

        let variable = ExpressionParser::parse(parser)?;

        match variable {
            BramaAstType::Symbol(_) => (),
            _ =>  {
                parser.set_index(index_backup);
                return Ok(BramaAstType::None);
            }
        };

        if parser.match_operator(&['=']).is_some() {
            let expression = ExpressionParser::parse(parser);
            match expression {
                Ok(BramaAstType::None) => return expression,
                Ok(_) => (),
                Err(_) => return expression
            };

            let assignment_ast = BramaAstType::Assignment {
                variable: Rc::new(variable),
                expression: Rc::new(expression.unwrap())
            };

            return Ok(assignment_ast);
        }
        parser.set_index(index_backup);
        Ok(BramaAstType::None)
    }
}
