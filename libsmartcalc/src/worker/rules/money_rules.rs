use std::collections::HashMap;
use std::fs;

use serde_json::{from_str, Result, Value};

use chrono::{Utc, Duration};
use chrono_tz::Tz;

use crate::types::{TokenType, BramaAstType};
use crate::tokinizer::{TokenLocation, TokenLocationStatus};

pub fn convert_money(fields: &HashMap<String, &TokenLocation>) -> std::result::Result<TokenType, String> {
    if fields.contains_key("money") && fields.contains_key("curency") {
        
    }

    Err("Time format not valid".to_string())
}
