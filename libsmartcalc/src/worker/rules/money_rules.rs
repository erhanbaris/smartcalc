use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::collections::btree_map::BTreeMap;

use serde_json::{from_str, Result, Value};

use chrono::{Utc, Duration};
use chrono_tz::Tz;

use crate::types::{TokenType, BramaAstType};
use crate::tokinizer::{TokenLocation, TokenLocationStatus};

pub fn convert_money(fields: &BTreeMap<String, &TokenLocation>) -> core::result::Result<TokenType, String> {
    if fields.contains_key("money") && fields.contains_key("curency") {
        
    }

    Err("Time format not valid".to_string())
}
