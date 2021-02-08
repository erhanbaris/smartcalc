use std::collections::HashMap;
use std::fs;

use serde_json::{from_str, Result, Value};

use chrono::{Utc, Duration};
use chrono_tz::Tz;

use crate::types::{TokenType, BramaAstType};
use crate::tokinizer::{TokenLocation, TokenLocationStatus};
use crate::tools::convert_currency;

pub fn convert_money(fields: &HashMap<String, &TokenLocation>) -> std::result::Result<TokenType, String> {
    if fields.contains_key("money") && fields.contains_key("curency") {
        let (price, currency) = match &fields.get("money").unwrap().token_type {
            Some(TokenType::Money(price, currency)) => (*price, currency.to_string()),
            Some(TokenType::Variable(variable)) => {
                match &*variable.data {
                    BramaAstType::Money(price, currency) => (*price, currency.to_string()),
                    _ => return Err("Money type not valid".to_string())
                }
            },
            _ => return Err("Money type not valid".to_string())
        };

        let to_currency = match &fields.get("curency").unwrap().token_type {
            Some(TokenType::Text(currency)) => currency.to_lowercase(),
            _ => return Err("Money type not valid".to_string())
        };

        let calculated_price = convert_currency(price, &currency, &to_currency);
        return Ok(TokenType::Money(calculated_price, to_currency));
    }

    Err("Money type not valid".to_string())
}
