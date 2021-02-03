use std::vec::Vec;
use lazy_static::*;

use crate::worker::{WorkerTrait, TypeItem, LanguageItem};
use crate::types::{Token, TokenType, ExpressionFunc};
use std::collections::HashMap;
use crate::tokinizer::Tokinizer;

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
                            match Tokinizer::tokinize(&item) {
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
    fn process(&self, _: &TypeItem, tokens: &mut Vec<Token>, _: Rc<Storage>) {
        if let Some(rules) = self.rules.get("en") {

            let mut execute_rules = true;
            while execute_rules {
                execute_rules = false;

                for (function, tokens_list) in rules.iter() {

                    for rule_tokens in tokens_list {

                        //println!("Rules {:?}", rule_tokens);

                        let total_rule_token       = rule_tokens.len();
                        let mut rule_token_index   = 0;
                        let mut target_token_index = 0;
                        let mut start_token_index  = 0;
                        let mut fields             = HashMap::new();

                        loop {
                            match tokens.get(target_token_index) {
                                Some(token) => {
                                    if let TokenType::Variable(variable) = &token.token {
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

                                        //println!("{:?}", variable.data.clone());
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
                                    let text_start_position = tokens[start_token_index].start;
                                    let text_end_position   = tokens[total_rule_token - 1].end;
                                    execute_rules = true;
                                    tokens.drain(start_token_index..total_rule_token);
                                    tokens.insert(start_token_index, Token {
                                        start: text_start_position,
                                        end: text_end_position,
                                        token,
                                        is_temp: false
                                    });
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