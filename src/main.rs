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
use smartcalc::tokinizer::Tokinizer;
use smartcalc::syntax::SyntaxParser;
use smartcalc::compiler::Interpreter;
use std::cell::RefCell;
use smartcalc::types::Token;
use smartcalc::executer::execute;

fn main() {
    let test_data = r"tarih = 11:30
    tarih add 2 hour
".to_string();
    println!("{:?}", execute(&test_data, &"en".to_string()));
}
