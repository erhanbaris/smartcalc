#[macro_use]
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

fn date_sum(_stack: &HashMap<String, String>) -> Option<()> {
    None
}

lazy_static! {
    static ref SENTENCES: HashMap<&'static str, Vec<types::Sentence>> = {
        let mut m = HashMap::new();
        m.insert("TR", vec![types::Sentence::new("{DATE:date}'e {NUMBER:day} gün ekle".to_string(), date_sum)]);
        m
    };
}

fn main() {

    let worker_executer = WorkerExecuter::new();

    /*let mut sentence_tokens : Vec<(Vec<types::Token>, &types::Sentence)> = Vec::new();
    for (_, sentences) in SENTENCES.iter() {
        for sentence in sentences {
            let tokens = tokinizer::Parser::parse(&sentence.text);
            println!("{:?}", tokens);
            sentence_tokens.push((tokens.unwrap(), &sentence));
        }
    }*/

    let test_data = "120 + 30% + 10%";
    let result = Parser::parse(test_data);
    match result {
        Ok(mut tokens) => {
            worker_executer.process(&mut tokens);
            let syntax = SyntaxParser::new(Box::new(tokens));
            match syntax.parse() {
                Ok(ast) => {
                    Executer::execute(&vec![Rc::new(ast)]);
                },
                _ => println!("error")
            }
        },
        _ => println!("{:?}", result)
    };
}
