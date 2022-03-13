/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

pub mod rules;

use core::cell::Cell;
use core::cell::RefCell;

use alloc::rc::Rc;
use alloc::vec::Vec;
use lazy_static::*;
use alloc::string::ToString;
use alloc::string::String;
use alloc::collections::btree_map::BTreeMap;

use crate::UiTokenType;
use crate::types::TokenType;
use crate::types::{ExpressionFunc};
use crate::tokinizer::{TokenInfo};

use self::rules::date_time_rules::*;
use self::rules::percent_rules::*;
use self::rules::money_rules::*;
use self::rules::number_rules::*;
use self::rules::cleanup_rules::*;
use self::rules::date_rules::*;
use self::rules::duration_rules::*;
use self::rules::dynamic_type_rules::*;

use super::TokenInfoStatus;
use super::Tokinizer;

pub type RuleItemList = Vec<(String, ExpressionFunc, Vec<Vec<Rc<TokenInfo>>>)>;

lazy_static! {
        pub static ref RULE_FUNCTIONS: BTreeMap<String, ExpressionFunc> = {
        let mut m = BTreeMap::new();
        m.insert("percent_calculator".to_string(), percent_calculator as ExpressionFunc);
        m.insert("time_for_location".to_string(),  time_for_location as ExpressionFunc);
        m.insert("convert_timezone".to_string(),   convert_timezone as ExpressionFunc);
        m.insert("time_with_timezone".to_string(), time_with_timezone as ExpressionFunc);
        m.insert("to_unixtime".to_string(),        to_unixtime as ExpressionFunc);
        m.insert("from_unixtime".to_string(),      from_unixtime as ExpressionFunc);
        m.insert("small_date".to_string(),         small_date as ExpressionFunc);
        
        m.insert("convert_money".to_string(),      convert_money as ExpressionFunc);

        m.insert("number_on".to_string(),          number_on as ExpressionFunc);
        m.insert("number_of".to_string(),          number_of as ExpressionFunc);
        m.insert("number_off".to_string(),         number_off as ExpressionFunc);

        m.insert("division_cleanup".to_string(),   division_cleanup as ExpressionFunc);
        m.insert("duration_parse".to_string(),     duration_parse as ExpressionFunc);
        m.insert("as_duration".to_string(),        as_duration as ExpressionFunc);
        m.insert("to_duration".to_string(),        to_duration as ExpressionFunc);
        m.insert("at_date".to_string(),            at_date as ExpressionFunc);
        
        m.insert("combine_durations".to_string(),  combine_durations as ExpressionFunc);

        m.insert("find_numbers_percent".to_string(),    find_numbers_percent as ExpressionFunc);
        m.insert("find_total_from_percent".to_string(), find_total_from_percent as ExpressionFunc);

        m.insert("number_type_convert".to_string(),     number_type_convert as ExpressionFunc);
        
        m.insert("dynamic_type_convert".to_string(),     dynamic_type_convert as ExpressionFunc);

        m
    };
}

pub fn rule_tokinizer(tokinizer: &mut Tokinizer) {
    let mut session_mut = tokinizer.session.borrow_mut();
    let language = session_mut.get_language();
    
    if let Some(language) = tokinizer.config.rule.get(&language) {

        let mut execute_rules = true;
        while execute_rules {
            execute_rules = false;

            for (function_name, function, tokens_list) in language.iter() {
                if cfg!(feature="debug-rules") {
                    log::debug!("# Checking for '{}'", function_name);
                }

                for rule_tokens in tokens_list {

                    let total_rule_token       = rule_tokens.len();
                    let mut rule_token_index   = 0;
                    let mut target_token_index = 0;
                    let mut start_token_index  = 0;
                    let mut fields             = BTreeMap::new();

                    while let Some(token) = session_mut.token_infos.get(target_token_index) {
                        target_token_index += 1;
                        if token.status.get() == TokenInfoStatus::Removed {
                            continue;
                        }

                        if let Some(token_type) = &token.token_type.borrow().deref() {

                            if let TokenType::Variable(variable) = &token_type {
                                let is_same = TokenType::variable_compare(&rule_tokens[rule_token_index], variable.data.borrow().clone());
                                if is_same {
                                    match TokenType::get_field_name(&rule_tokens[rule_token_index]) {
                                        Some(field_name) => fields.insert(field_name.to_string(), token.clone()),
                                        None => None
                                    };

                                    rule_token_index   += 1;
                                } else {
                                    rule_token_index    = 0;
                                    start_token_index   = target_token_index;
                                }
                            }
                            else if token == &rule_tokens[rule_token_index] {
                                match TokenType::get_field_name(&rule_tokens[rule_token_index]) {
                                    Some(field_name) => fields.insert(field_name.to_string(), token.clone()),
                                    None => None
                                };

                                if cfg!(feature="debug-rules") {
                                    log::debug!("Ok, {:?} == {:?}", token.token_type, &rule_tokens[rule_token_index].token_type);
                                }

                                rule_token_index   += 1;
                            }
                            else {
                                if cfg!(feature="debug-rules") {
                                    log::debug!("No, {:?} == {:?}", token.token_type, &rule_tokens[rule_token_index].token_type);
                                }
                                rule_token_index    = 0;
                                start_token_index   = target_token_index;
                            }

                            if total_rule_token == rule_token_index { break; }
                        }
                    }

                    if total_rule_token == rule_token_index {
                        if cfg!(feature="debug-rules") {
                            log::debug!(" --------- {} executing", function_name);
                        }

                        match function(tokinizer.config, tokinizer, &fields) {
                            Ok(token) => {
                                if cfg!(feature="debug-rules") {
                                    log::debug!("Rule function success with new token: {:?}", token);
                                }

                                let text_start_position = session_mut.token_infos[start_token_index].start;
                                let text_end_position   = session_mut.token_infos[target_token_index - 1].end;
                                execute_rules = true;

                                for index in start_token_index..target_token_index {
                                    session_mut.token_infos[index].status.set(TokenInfoStatus::Removed);
                                }

                                if let Some(data) = fields.get("type") {
                                    tokinizer.ui_tokens.update_tokens(data.start, data.end, UiTokenType::Symbol2)
                                }

                                session_mut.token_infos.insert(start_token_index, Rc::new(TokenInfo {
                                    start: text_start_position,
                                    end: text_end_position,
                                    token_type: RefCell::new(Some(token)),
                                    original_text: "".to_string(),
                                    status: Cell::new(TokenInfoStatus::Active)
                                }));
                                break;
                            },
                            Err(error) => log::info!("Rule execution error, {}", error)
                        }
                    }
                }
            }
        }
    }

    if cfg!(feature="debug-rules") {
        log::debug!("Updated token_infos: {:?}", session_mut.token_infos);
    }
}