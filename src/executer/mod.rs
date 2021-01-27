use crate::worker::WorkerExecuter;
use crate::tokinizer::Tokinizer;
use crate::syntax::SyntaxParser;
use std::rc::Rc;
use crate::types::{Token, BramaAstType, VariableInfo};
use crate::compiler::Interpreter;
use std::cell::RefCell;

pub struct Storage {
    pub asts: RefCell<Vec<Rc<BramaAstType>>>,
    pub variables: RefCell<Vec<Rc<VariableInfo>>>
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            asts: RefCell::new(Vec::new()),
            variables: RefCell::new(Vec::new())
        }
    }
}

pub fn execute(data: &String, language: &String) -> Vec<Result<Rc<BramaAstType>, String>> {
    let mut results     = Vec::new();
    let storage         = Rc::new(Storage::new());
    let worker_executer = WorkerExecuter::new();

    for text in data.lines() {
        if text.len() == 0 {
            storage.asts.borrow_mut().push(Rc::new(BramaAstType::None));
            continue;
        }

        let result = Tokinizer::tokinize(&text.to_string());
        match result {
            Ok(mut tokens) => {
                Token::update_for_variable(&mut tokens, storage.clone());
                worker_executer.process(&language, &mut tokens, storage.clone());
                let syntax = SyntaxParser::new(Rc::new(tokens), storage.clone());
                match syntax.parse() {
                    Ok(ast) => {
                        let ast_rc = Rc::new(ast);
                        storage.asts.borrow_mut().push(ast_rc.clone());
                        results.push(Interpreter::execute(ast_rc.clone(), storage.clone()));
                    },
                    Err((error, _, _)) => println!("error, {}", error)
                }
            },
            _ => {
                storage.asts.borrow_mut().push(Rc::new(BramaAstType::None));
            }
        };
    }

    results
}