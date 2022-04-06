/*
 * smartcalc v1.0.8
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::vec::Vec;
use alloc::string::ToString;

use crate::types::TokenType;

use super::{Tokinizer, regex_tokinizer::get_atom};

 

pub fn alias_tokinizer(tokinizer: &mut Tokinizer) {
    for token in tokinizer.token_infos.iter() {
        for (re, data) in tokinizer.config.alias_regex.iter() {
            if re.is_match(&token.original_text.to_lowercase()) {
                let new_values = match tokinizer.config.token_parse_regex.get("atom") {
                    Some(items) => get_atom(tokinizer.config, data, items),
                    _ => Vec::new()
                };

                match new_values.len() {
                    1 => {
                        if let Some(token_type) = &new_values[0].2 {
                            *token.token_type.borrow_mut() = Some(token_type.clone());
                            break;
                        }
                    },
                    0 => {
                        *token.token_type.borrow_mut() = Some(TokenType::Text(data.to_string()));
                        break;
                    },
                    _ => log::warn!("{} has multiple atoms. It is not allowed", data)
                };
            }
        }
    }

    for token in tokinizer.token_infos.iter() {
        for (re, data) in tokinizer.config.language_alias_regex.get(&tokinizer.language).unwrap().iter() {
            if re.is_match(&token.original_text.to_lowercase()) {
                let new_values = match tokinizer.config.token_parse_regex.get("atom") {
                    Some(items) => get_atom(tokinizer.config, data, items),
                    _ => Vec::new()
                };

                match new_values.len() {
                    1 => {
                        if let Some(token_type) = &new_values[0].2 {
                            *token.token_type.borrow_mut() = Some(token_type.clone());
                            break;
                        }
                    },
                    0 => {
                        *token.token_type.borrow_mut() = Some(TokenType::Text(data.to_string()));
                        break;
                    },
                    _ => log::warn!("{} has multiple atoms. It is not allowed", data)
                };
            }
        }
    }
}
