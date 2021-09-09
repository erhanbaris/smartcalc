use alloc::rc::Rc;
use alloc::vec::Vec;
use lazy_static::*;
use alloc::string::ToString;
use alloc::string::String;
use alloc::collections::btree_map::BTreeMap;

use crate::types::{ExpressionFunc};
use crate::tokinizer::{TokenInfo};

use crate::worker::rules::date_time_rules::*;
use crate::worker::rules::percent_rules::*;
use crate::worker::rules::money_rules::*;
use crate::worker::rules::number_rules::*;
use crate::worker::rules::cleanup_rules::*;
use crate::worker::rules::date_rules::*;
use crate::worker::rules::duration_rules::*;

lazy_static! {
        pub static ref RULE_FUNCTIONS: BTreeMap<String, ExpressionFunc> = {
        let mut m = BTreeMap::new();
        m.insert("percent_calculator".to_string(), percent_calculator as ExpressionFunc);
        m.insert("time_for_location".to_string(),  time_for_location as ExpressionFunc);
        m.insert("small_date".to_string(),         small_date as ExpressionFunc);
        
        m.insert("convert_money".to_string(),      convert_money as ExpressionFunc);

        m.insert("number_on".to_string(),          number_on as ExpressionFunc);
        m.insert("number_of".to_string(),          number_of as ExpressionFunc);
        m.insert("number_off".to_string(),         number_off as ExpressionFunc);

        m.insert("division_cleanup".to_string(),   division_cleanup as ExpressionFunc);
        m.insert("duration_parse".to_string(),     duration_parse as ExpressionFunc);
        m.insert("as_duration".to_string(),        as_duration as ExpressionFunc);
        m.insert("to_duration".to_string(),        to_duration as ExpressionFunc);
        m.insert("combine_durations".to_string(),  combine_durations as ExpressionFunc);

        m.insert("find_numbers_percent".to_string(),    find_numbers_percent as ExpressionFunc);
        m.insert("find_total_from_percent".to_string(), find_total_from_percent as ExpressionFunc);

        m
    };
}

pub type RuleItemList     = Vec<(String, ExpressionFunc, Vec<Vec<Rc<TokenInfo>>>)>;
