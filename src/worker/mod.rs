use std::vec::Vec;
use std::rc::Rc;

use crate::types::Token;

mod alias;

pub trait WorkerTrait {
    fn process(&self, tokens: &mut Vec<Token>);
}

pub struct WorkerExecuter {
    workers: Vec<Rc<WorkerTrait>>
}

impl WorkerExecuter {
    pub fn new() -> WorkerExecuter {
        WorkerExecuter {
            workers: vec![Rc::new(alias::AliasWorker::new())]
        }
    }

    pub fn process(&self, tokens: &mut Vec<Token>) {
        for worker in &self.workers {
            worker.process(tokens);
        }
    }
}