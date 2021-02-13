use alloc::vec::Vec;
use lazy_static::*;
use alloc::string::ToString;
use alloc::string::String;
use alloc::collections::btree_map::BTreeMap;

use crate::types::{ExpressionFunc};
use crate::tokinizer::{TokenLocation};

use crate::worker::rules::date_time_rules::*;
use crate::worker::rules::percent_rules::*;
use crate::worker::rules::money_rules::*;
use crate::worker::rules::number_rules::*;

lazy_static! {
        pub static ref RULE_FUNCTIONS: BTreeMap<String, ExpressionFunc> = {
        let mut m = BTreeMap::new();
        m.insert("hour_add".to_string(),           hour_add as ExpressionFunc);
        m.insert("percent_calculator".to_string(), percent_calculator as ExpressionFunc);
        m.insert("time_for_location".to_string(),  time_for_location as ExpressionFunc);
        
        m.insert("money_on".to_string(),           money_on as ExpressionFunc);
        m.insert("money_of".to_string(),           money_of as ExpressionFunc);
        m.insert("money_off".to_string(),          money_off as ExpressionFunc);
        m.insert("convert_money".to_string(),      convert_money as ExpressionFunc);

        m.insert("number_on".to_string(),          number_on as ExpressionFunc);
        m.insert("number_of".to_string(),          number_of as ExpressionFunc);
        m.insert("number_off".to_string(),         number_off as ExpressionFunc);
        m
    };
}

pub type RuleItemList     = Vec<(ExpressionFunc, Vec<Vec<TokenLocation>>)>;
pub type RuleLanguage     = BTreeMap<String, RuleItemList>;
