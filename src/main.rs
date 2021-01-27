#[macro_use]
extern crate lazy_static;

mod types;
mod tokinizer;
mod syntax;
mod worker;
mod executer;
mod compiler;

use smartcalc::executer::execute;

fn main() {
    let test_data = r"tarih = 11:30
tarih add 12 hour
".to_string();

    println!("{:?}", execute(&test_data, &"en".to_string()));
}
