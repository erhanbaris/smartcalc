use std::vec::Vec;
use std::rc::Rc;

use serde_json::{from_str, Result};
use serde::{Deserialize, Serialize};

use crate::types::Token;
use std::collections::HashMap;

mod alias;
mod rule;
mod rules;

const JSON_DATA: &str = r#"{
    "en": {
        "rules": {
            "date_sum": ["{DATE:date}\"e {NUMBER:day} gün ekle", "{TIME:time} + {NUMBER:hours} hour"],
            "time_for_location": ["time in {TEXT:location}", "time at {TEXT:location}", "time for {TEXT:location}"]
        },
        "aliases": {
            "x": ["X", "times", "multiply"],
            "+": ["add", "sum", "append"],
            "-": ["exclude", "minus"],
            "%": ["percent", "percentage"]
        }
    }
}"#;

pub type ItemList     = HashMap<String, Vec<String>>;
pub type TypeItem     = HashMap<String, ItemList>;
pub type LanguageItem = HashMap<String, TypeItem>;

pub trait WorkerTrait {
    fn process(&self, items: &TypeItem, tokens: &mut Vec<Token>);
}

pub struct WorkerExecuter {
    workers: Vec<Rc<dyn WorkerTrait>>,
    data: LanguageItem
}

impl WorkerExecuter {
    pub fn new() -> WorkerExecuter {
        let json_value: Result<LanguageItem> = from_str(JSON_DATA);
        let executer = match json_value {
            Ok(data) => WorkerExecuter {
                workers: vec![Rc::new(alias::AliasWorker::new()), Rc::new(rule::RuleWorker::new(&data))],
                data: data
            },
            Err(error) => panic!(format!("Worker json not parsed. Error: {}", error))
        };

        executer
    }

    pub fn process(&self, language_key: &String, tokens: &mut Vec<Token>) {
        for worker in &self.workers {
            worker.process(&self.data.get(language_key).unwrap(), tokens);
        }
    }
}