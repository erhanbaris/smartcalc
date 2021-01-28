use std::collections::HashMap;
use std::fs;

use serde_json::{from_str, Result, Value};

use chrono::{Utc, Duration};
use chrono_tz::Tz;

use crate::types::{Token, BramaAstType};

pub fn percent_calculator(fields: &HashMap<String, &Token>) -> std::result::Result<Token, String> {
    if fields.contains_key("p") && fields.contains_key("number") {
        let number = match fields.get("number").unwrap() {
            Token::Number(number) => number,
            Token::Variable(variable) => {
                match &*variable.data {
                    BramaAstType::Number(number) => number,
                    _ => return Err("Number not valid".to_string())
                }
            },
            _ => return Err("Number not valid".to_string())
        };

        let percent = match fields.get("p").unwrap() {
            Token::Percent(percent) => percent,
            Token::Variable(variable) => {
                match &*variable.data {
                    BramaAstType::Percent(percent) => percent,
                    _ => return Err("Percent not valid".to_string())
                }
            },
            _ => return Err("Percent not valid".to_string())
        };
        return Ok(Token::Number((percent * number) / 100.0));
    }

    Err("Percent not valid".to_string())
}
