use std::vec::Vec;
use lazy_static::*;

use crate::worker::{WorkerTrait, TypeItem, LanguageItem};
use crate::types::{Token, ExpressionFunc, FieldType, BramaAstType};
use std::collections::HashMap;
use crate::tokinizer::Tokinizer;

use crate::worker::rules::date_time_rules::*;
use crate::executer::{Storage, prepare_code};
use std::rc::Rc;

lazy_static! {
        static ref RULE_FUNCTIONS: HashMap<String, ExpressionFunc> = {
        let mut m = HashMap::new();
        m.insert("hour_add".to_string(),          hour_add as ExpressionFunc);
        m.insert("time_for_location".to_string(), time_for_location as ExpressionFunc);
        m
    };
}

pub type RuleItemList     = Vec<(ExpressionFunc, Vec<Vec<Token>>)>;
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
                            let prepared_item = prepare_code(&item.to_string());
                            match Tokinizer::tokinize(&prepared_item) {
                                Ok(tokens) => function_items.push(tokens),
                                Err((error, _, _)) => println!("Error : {}", error)
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
    fn process(&self, items: &TypeItem, tokens: &mut Vec<Token>, storage: Rc<Storage>) {
        if let Some(rules) = self.rules.get("en") {

            let mut execute_rules = true;
            while execute_rules {
                execute_rules = false;

                for (function, tokens_list) in rules.iter() {

                    for rule_tokens in tokens_list {

                        let total_rule_token       = rule_tokens.len();
                        let mut rule_token_index   = 0;
                        let mut target_token_index = 0;
                        let mut start_token_index  = 0;
                        let mut fields             = HashMap::new();

                        loop {
                            match tokens.get(target_token_index) {
                                Some(token) => {
                                    if let Token::Variable(variable) = token {
                                        let is_same = Token::variable_compare(&rule_tokens[rule_token_index], variable.data.clone());
                                        if is_same {
                                            match Token::get_field_name(&rule_tokens[rule_token_index]) {
                                                Some(field_name) => fields.insert(field_name.to_string(), token),
                                                None => None
                                            };

                                            rule_token_index   += 1;
                                            target_token_index += 1;
                                        } else {
                                            rule_token_index    = 0;
                                            target_token_index += 1;
                                            start_token_index   = target_token_index;
                                        }

                                        println!("{:?}", variable.data.clone());
                                    }
                                    else if token == &rule_tokens[rule_token_index] {
                                        match Token::get_field_name(&rule_tokens[rule_token_index]) {
                                            Some(field_name) => fields.insert(field_name.to_string(), token),
                                            None => None
                                        };

                                        rule_token_index   += 1;
                                        target_token_index += 1;
                                    }
                                    else {
                                        rule_token_index    = 0;
                                        target_token_index += 1;
                                        start_token_index   = target_token_index;
                                    }

                                    if total_rule_token == rule_token_index { break; }
                                },
                                _=> break
                            }
                        }

                        if total_rule_token == rule_token_index {
                            match function(&fields) {
                                Ok(token) => {
                                    execute_rules = true;
                                    tokens.drain(start_token_index..total_rule_token);
                                    tokens.insert(start_token_index, token);
                                },
                                Err(error) => println!("Parse issue: {}", error)
                            }
                        }
                    }
                }
            }
        }
    }
}