use std::vec::Vec;
use lazy_static::*;

use crate::types::{ExpressionFunc};
use std::collections::HashMap;
use crate::tokinizer::{TokenLocation};

use crate::worker::rules::date_time_rules::*;
use crate::worker::rules::percent_rules::*;

lazy_static! {
        pub static ref RULE_FUNCTIONS: HashMap<String, ExpressionFunc> = {
        let mut m = HashMap::new();
        m.insert("hour_add".to_string(),           hour_add as ExpressionFunc);
        m.insert("percent_calculator".to_string(), percent_calculator as ExpressionFunc);
        m.insert("time_for_location".to_string(),  time_for_location as ExpressionFunc);
        m
    };
}

pub type RuleItemList     = Vec<(ExpressionFunc, Vec<Vec<TokenLocation>>)>;
pub type RuleLanguage     = HashMap<String, RuleItemList>;
