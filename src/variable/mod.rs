/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::cell::{RefCell, Cell};
use core::ops::Deref;
use alloc::{string::{String, ToString}, vec::Vec, rc::Rc};
use crate::types::find_location;
use crate::{types::TokenType, SmartCalcAstType, tokinizer::{Tokinizer, TokenInfoStatus, TokenInfo}, UiTokenType};

#[derive(Debug)]
pub struct VariableInfo {
    pub tokens: Vec<Rc<TokenType>>,
    pub data: RefCell<Rc<SmartCalcAstType>>
}

impl PartialEq for VariableInfo {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

unsafe impl Send for VariableInfo {}
unsafe impl Sync for VariableInfo {}


impl ToString for VariableInfo {
    fn to_string(&self) -> String {
        self.tokens.iter().map(|item| item.to_string().to_lowercase()).collect::<String>()
    }
}

pub fn update_token_variables(tokenizer: &mut Tokinizer) {
    let session = tokenizer.session;
    let mut token_start_index = 0;
    tokenizer.ui_tokens.sort();

    for (index, token) in tokenizer.token_infos.iter().enumerate().skip(1) {
        if let Some(TokenType::Operator('=')) = &token.token_type.borrow().deref() {
            token_start_index = index as usize + 1;

            tokenizer.ui_tokens.update_tokens(0, tokenizer.token_infos[index - 1].end, UiTokenType::VariableDefination);                        
            break;
        }
    }

   let mut update_tokens = true;

    while update_tokens {
        let mut found = false;
        let mut closest_variable = usize::max_value();
        let mut name = String::new();
        let mut variable_size = 0;

        update_tokens            = false;

        for (variable_name, variable) in session.variables.borrow().iter() {
            if let Some(start_index) = find_location(&tokenizer.token_infos[token_start_index..].to_vec(), &variable.tokens) {
                if (start_index == closest_variable && variable_size < variable.tokens.len()) || (start_index < closest_variable) {
                    closest_variable = start_index;
                    name = variable_name.to_string();
                    variable_size = variable.tokens.len();
                    found = true;
                }
            }
        }

        if found {
            let remove_start_index  = token_start_index + closest_variable;
            let remove_end_index    = remove_start_index + variable_size;
            let text_start_position = tokenizer.token_infos[remove_start_index].start;
            let text_end_position   = tokenizer.token_infos[remove_end_index - 1].end;

            tokenizer.ui_tokens.update_tokens(text_start_position, text_end_position, UiTokenType::VariableUse);

            let buffer_length: usize = tokenizer.token_infos[remove_start_index..remove_end_index].iter().map(|s| s.original_text.len()).sum();
            let mut original_text = String::with_capacity(buffer_length);

            for token in tokenizer.token_infos[remove_start_index..remove_end_index].iter() {
                original_text.push_str(&token.original_text.to_string());
            }

            tokenizer.token_infos.drain(remove_start_index..remove_end_index);
            
            let token_type = RefCell::new(Some(TokenType::Variable(session.variables.borrow()[&name].clone())));
            
            tokenizer.token_infos.insert(remove_start_index, Rc::new(TokenInfo {
                start: text_start_position as usize,
                end: text_end_position as usize,
                token_type,
                original_text: original_text.to_string(),
                status: Cell::new(TokenInfoStatus::Active)
            }));
            update_tokens = true;
        }
    }
}
