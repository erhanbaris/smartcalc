use crate::worker::WorkerExecuter;
use crate::tokinizer::Tokinizer;
use crate::syntax::SyntaxParser;
use std::rc::Rc;
use crate::types::{Token, BramaAstType};
use crate::compiler::Interpreter;

pub fn execute(data: &String, language: &String) -> Vec<Result<Rc<BramaAstType>, String>> {

    let worker_executer = WorkerExecuter::new();
    let mut asts = Vec::new();
    let mut variables: Vec<Vec<Token>> = Vec::new();

    for (index, text) in data.lines().enumerate() {
        if text.len() == 0 {
            asts.push(Rc::new(BramaAstType::None));
            continue;
        }

        let result = Tokinizer::tokinize(&text.to_string());
        match result {
            Ok(mut tokens) => {
                worker_executer.process(&language, &mut tokens);
                let syntax = SyntaxParser::new(Rc::new(tokens), variables.to_vec());
                match syntax.parse() {
                    Ok(ast) => {
                        asts.push(Rc::new(ast));
                        variables = syntax.variables.borrow().to_vec();
                    },
                    Err((error, _, _)) => println!("error, {}", error)
                }
            },
            _ => {
                asts.push(Rc::new(BramaAstType::None));
                println!("{:?}", result);
            }
        };
    }
    Interpreter::execute(&asts)
}