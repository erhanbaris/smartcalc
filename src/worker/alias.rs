use std::vec::Vec;
use std::rc::Rc;

use crate::worker::{WorkerTrait, TypeItem};
use crate::types::{Token};

pub struct AliasWorker;

impl AliasWorker {
    pub fn new() -> AliasWorker {
        AliasWorker { }
    }
}

impl WorkerTrait for AliasWorker {
    fn process(&self, items: &TypeItem, tokens: &mut Vec<Token>) {
        if let Some(collection) = items.get("aliases") {
            let mut counter        = 0;
            let mut tokens_updated = true;

            while tokens_updated && counter < 25 {
                tokens_updated = false;
                counter       += 1;

                for index in 0..tokens.len() {
                    if let Token::Text(text) = &tokens[index] {
                        let key_data = collection.iter().find(|&(key, value)| (value.iter().find(| &x| x == &**text)).is_some());

                        if let Some(key) = key_data {
                            /* Parse numbers */
                            if let Ok(number) = key.0.parse::<f64>() {
                                tokens[index] = Token::Number(number);
                            }

                            /* Parse operators */
                            else if key.0.len() == 1 && !key.0.chars().nth(0).unwrap().is_alphabetic() {
                                tokens[index] = Token::Operator(key.0.chars().nth(0).unwrap());
                            }
                            else {

                                /* Normal text  */
                                tokens[index] = Token::Text(Rc::new(key.0.to_string()));
                            }

                            tokens_updated = true;
                        }
                    }
                }
            }
        }
    }
}