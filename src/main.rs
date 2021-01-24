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
use std::cell::RefCell;
use smartcalc::types::Token;

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

    let test_data = r"
erhan barış = 120
aysel barış = 200
toplam = erhan barış + aysel barış";
    let mut asts = Vec::new();
    let mut variables: Vec<Vec<Token>> = Vec::new();

    for text in test_data.lines() {
        let result = Parser::parse(text);
        match result {
            Ok(mut tokens) => {
                worker_executer.process(&mut tokens);
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
    Executer::execute(&asts);
}
