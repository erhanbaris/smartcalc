use std::collections::HashMap;
use crate::types::{Token, TokenType, BramaAstType};

pub fn percent_calculator(fields: &HashMap<String, &Token>) -> std::result::Result<TokenType, String> {
    if fields.contains_key("p") && fields.contains_key("number") {
        let number = match &fields.get("number").unwrap().token {
            TokenType::Number(number) => number,
            TokenType::Variable(variable) => {
                match &*variable.data {
                    BramaAstType::Number(number) => number,
                    _ => return Err("Number not valid".to_string())
                }
            },
            _ => return Err("Number not valid".to_string())
        };

        let percent = match &fields.get("p").unwrap().token {
            TokenType::Percent(percent) => percent,
            TokenType::Variable(variable) => {
                match &*variable.data {
                    BramaAstType::Percent(percent) => percent,
                    _ => return Err("Percent not valid".to_string())
                }
            },
            _ => return Err("Percent not valid".to_string())
        };
        return Ok(TokenType::Number((percent * number) / 100.0));
    }

    Err("Percent not valid".to_string())
}
