/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */


use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use regex::Regex;
use lazy_static::*;

mod number;
mod operator;
mod text;
mod whitespace;
mod field;
mod percent;
mod atom;
mod time;
mod money;
mod comment;
mod month;
mod timezone;

use crate::SmartCalcConfig;

pub use self::time::time_regex_parser;
pub use self::number::number_regex_parser;
pub use self::percent::percent_regex_parser;
pub use self::money::money_regex_parser;
pub use self::text::text_regex_parser;
pub use self::field::field_regex_parser;
pub use self::atom::{atom_regex_parser, get_atom};
pub use self::whitespace::whitespace_regex_parser;
pub use self::comment::comment_regex_parser;
pub use self::timezone::timezone_regex_parser;
pub use self::month::month_parser;
pub use self::operator::operator_regex_parser;

use super::Tokinizer;


pub type RegexParser = fn(config: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]);
pub type Parser      = fn(config: &SmartCalcConfig, tokinizer: &mut Tokinizer, data: &str);


lazy_static! {
    pub static ref TOKEN_REGEX_PARSER: Vec<(&'static str, RegexParser)> = {
        let m = vec![
        ("comment",    comment_regex_parser    as RegexParser),
        ("field",      field_regex_parser      as RegexParser),
        ("money",      money_regex_parser      as RegexParser),
        ("atom",       atom_regex_parser       as RegexParser),
        ("percent",    percent_regex_parser    as RegexParser),
        ("timezone",   timezone_regex_parser   as RegexParser),
        ("time",       time_regex_parser       as RegexParser),
        ("number",     number_regex_parser     as RegexParser),
        ("text",       text_regex_parser       as RegexParser),
        ("whitespace", whitespace_regex_parser as RegexParser),
        ("operator",   operator_regex_parser   as RegexParser)];
        m
    };
}

lazy_static! {
    pub static ref LANGUAGE_BASED_TOKEN_PARSER: Vec<Parser> = {
        let m = vec![month_parser as Parser];
        m
    };
}

pub fn regex_tokinizer(tokinizer: &mut Tokinizer) {
    /* Token parser with regex */
    for (key, func) in TOKEN_REGEX_PARSER.iter() {
        if let Some(items) = tokinizer.config.token_parse_regex.get(&key.to_string()) { 
            func(tokinizer.config, tokinizer, items) 
        }
    }
    
    tokinizer.session.borrow_mut().cleanup_token_infos();
}

pub fn language_tokinizer(tokinizer: &mut Tokinizer) {
    let lowercase_data = tokinizer.data.to_lowercase();
    for func in LANGUAGE_BASED_TOKEN_PARSER.iter() {
        func(tokinizer.config, tokinizer, &lowercase_data);
    }

    tokinizer.session.borrow_mut().cleanup_token_infos();
}
