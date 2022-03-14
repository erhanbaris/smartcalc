/*
 * smartcalc v1.0.6
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::{collections::BTreeMap, rc::Rc};
use core::{ops::Deref, cell::{RefCell, Cell}};
use alloc::string::ToString;

use crate::{types::TokenType, UiTokenType};

use super::{Tokinizer, TokenInfoStatus, TokenInfo, get_number};

pub fn dynamic_type_tokinizer(tokinizer: &mut Tokinizer) {    
    let mut execute_rules = true;
    while execute_rules {
        execute_rules = false;

        for (type_name, type_items) in tokinizer.config.types.iter() {
            for (_, dynamic_type) in type_items.iter() {
                for rule_tokens in dynamic_type.parse.iter() {
                    let total_rule_token       = rule_tokens.len();
                    let mut rule_token_index   = 0;
                    let mut target_token_index = 0;
                    let mut start_token_index  = 0;
                    let mut fields             = BTreeMap::new();

                    while let Some(token) = tokinizer.token_infos.get(target_token_index) {
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
                            log::debug!(" --------- {} found", type_name);
                        }
                        
                        let text_start_position = tokinizer.token_infos[start_token_index].start;
                        let text_end_position   = tokinizer.token_infos[target_token_index - 1].end;
                        execute_rules = true;

                        for index in start_token_index..target_token_index {
                            tokinizer.token_infos[index].status.set(TokenInfoStatus::Removed);
                        }
                        
                        let value = get_number("value", &fields).unwrap();
                        if let Some(data) = fields.get("type") {
                            tokinizer.ui_tokens.update_tokens(data.start, data.end, UiTokenType::Symbol2)
                        }

                        tokinizer.token_infos.insert(start_token_index, Rc::new(TokenInfo {
                            start: text_start_position,
                            end: text_end_position,
                            token_type: RefCell::new(Some(TokenType::DynamicType(value, dynamic_type.clone()))),
                            original_text: "".to_string(),
                            status: Cell::new(TokenInfoStatus::Active)
                        }));
                        break;
                    }
                }
            }
        }
    }

    if cfg!(feature="debug-rules") {
        log::debug!("Updated token_infos: {:?}", tokinizer.token_infos);
    }
}