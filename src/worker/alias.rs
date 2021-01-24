use std::vec::Vec;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::worker::WorkerTrait;
use crate::types::{Token, AliasList, AliasYamlCollection};
use crate::types::AliasCollection;
use crate::types::BramaTokenType;
use std::borrow::Borrow;

pub struct AliasWorker {
    collection: AliasCollection
}

const YAML_DATA: &str = r"---
en:
    '*':
       - times
       - multiply
       - x
    '+':
       - add
       - append
       - include
    '-':
       - exclude
       - minus
";

impl AliasWorker {
    pub fn new() -> AliasWorker {
        let mut collection = AliasCollection::new();

        let yaml_result = serde_yaml::from_str(&YAML_DATA);

        if let Ok(result) = yaml_result {
            let yaml: AliasYamlCollection = result;

            for (language_key, alias_list) in yaml {
                let mut language = AliasList::new();

                for (alias_key, alias_items) in alias_list {
                    for item in alias_items {
                        language.insert(item, alias_key.to_string());
                    }
                }

                collection.insert(language_key, language);
            }
        }

        AliasWorker {
            collection: collection
        }
    }
}

impl WorkerTrait for AliasWorker {
    fn process(&self, tokens: &mut Vec<Token>) {
        if let Some(collection) = self.collection.get("en") {
            let mut counter        = 0;
            let mut tokens_updated = true;

            while tokens_updated && counter < 25 {
                tokens_updated = false;
                counter       += 1;

                for index in 0..tokens.len() {
                    if let BramaTokenType::Text(text) = &tokens[index].token_type {
                        match collection.get(&**text) {
                            Some(alias) => {
                                tokens[index] = Token {
                                    start: tokens[index].start,
                                    end: tokens[index].end,
                                    token_type: BramaTokenType::Text(Rc::new(alias.to_string()))
                                };
                                tokens_updated = true;
                            },
                            _ => ()
                        };
                    }
                }
            }
        }
    }
}