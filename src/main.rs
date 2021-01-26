#[macro_use(lazy_static)]
extern crate lazy_static;

mod types;
mod tokinizer;
mod syntax;
mod worker;

use std::vec::Vec;
use std::rc::Rc;
use std::collections::HashMap;

use smartcalc::worker::WorkerExecuter;
use smartcalc::tokinizer::Parser;
use smartcalc::syntax::SyntaxParser;
use smartcalc::compiler::Executer;
use std::cell::RefCell;
use smartcalc::types::Token;

fn date_sum(_stack: &HashMap<String, String>) -> Option<()> {
    None
}

fn main() {

    let worker_executer = WorkerExecuter::new();

    let test_data = r"aysel = 10324
erhan = 5890
nakit = erhan + aysel
erhan maaş = 25965.25
aysel maaş = 3500
sigorta geri ödemesi = 8600
toplam nakit = nakit + erhan maaş + aysel maaş + sigorta geri ödemesi";


    //let test_data = r"erhan time in tokyo";
    let mut asts = Vec::new();
    let mut variables: Vec<Vec<Token>> = Vec::new();

    for (index, text) in test_data.lines().enumerate() {
        let result = Parser::parse(&text.to_string());
        match result {
            Ok(mut tokens) => {
                worker_executer.process(&"en".to_string(), &mut tokens);
                let syntax = SyntaxParser::new(Rc::new(tokens), variables.to_vec());
                match syntax.parse() {
                    Ok(ast) => {
                        asts.push(Rc::new(ast));
                        variables = syntax.variables.borrow().to_vec();
                    },
                    Err((error, _, _)) => println!("error, {}", error)
                }
            },
            _ => println!("{:?}", result)
        };
    }
    println!("{:?}", asts);
    println!("{:?}", Executer::execute(&asts));
}
