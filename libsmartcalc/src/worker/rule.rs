use std::vec::Vec;
use lazy_static::*;

use crate::worker::{WorkerTrait, TypeItem, LanguageItem};
use crate::types::{Token, TokenType, ExpressionFunc};
use std::collections::HashMap;
use crate::tokinizer::{Tokinizer, TokenLocation};

use crate::worker::rules::date_time_rules::*;
use crate::worker::rules::percent_rules::*;
use crate::executer::{Storage};
use std::rc::Rc;

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

pub struct RuleWorker {
    rules: RuleLanguage
}

impl RuleWorker {
    pub fn new(language_item: &LanguageItem) -> RuleWorker {
        let mut rule_worker = RuleWorker {
            rules: RuleLanguage::new()
        };

        for (language_key, language_items) in language_item.iter() {
            if let Some(collection) = language_items.get("rules") {

                let mut rule_item = RuleItemList::new();
                for (function_name, items) in collection.iter() {

                    if let Some(function_ref) = RULE_FUNCTIONS.get(function_name) {
                        let mut function_items = Vec::new();

                        for item in items {
                            match Tokinizer::token_locations(&item) {
                                Some(tokens) => function_items.push(tokens),
                                _ => println!("Error : token_locations not working")
                            }
                        }

                        rule_item.push((*function_ref, function_items));
                    }
                    else {
                        println!("Function not found : {}", function_name);
                    }
                }

                rule_worker.rules.insert(language_key.to_string(), rule_item);
            }
        }

        rule_worker
    }
}

impl WorkerTrait for RuleWorker {
    fn process(&self, _: &TypeItem, tokens: &mut Vec<Token>, _: Rc<Storage>) {

    }
}