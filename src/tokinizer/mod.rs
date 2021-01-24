mod number;
mod operator;
mod text;
mod whitespace;
mod atom;
mod percent;

use std::str;

use crate::types::*;
use self::number::number_parser;
use self::operator::operator_parser;
use self::text::text_parser;
use self::whitespace::whitespace_parser;
use self::atom::atom_parser;
use self::percent::percent_parser;

pub struct Parser {
    tokinizer: Tokinizer
}

impl Parser {

    pub fn parse(data: &'static str) -> ParseResult {

        let a: Vec<char> = data.chars().collect();
        let aa = a.get(0);

        let mut parser = Parser {
            tokinizer: Tokinizer {
                column: 0,
                line: 0,
                tokens: Vec::new(),
                iter: data.chars().collect(),
                data: data.to_string(),
                index: 0,
                indexer: 0,
                total: data.chars().count()
            }
        };

        let token_parses : Vec<TokenParser> = vec![atom_parser, percent_parser, whitespace_parser, text_parser, number_parser, operator_parser];

        while !parser.tokinizer.is_end() {
            for parse in &token_parses {
                let status = parse(&mut parser.tokinizer);
                match status {
                    Ok(true) => break,
                    Ok(false) => continue,
                    Err((message, column)) => return Err((message, 0, column))
                }
            }
        }

        Ok(parser.tokinizer.tokens)
    }
}