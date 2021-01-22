#[macro_use]
extern crate lazy_static;

mod types;
mod parser;

use std::vec::Vec;
use std::collections::HashMap;


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
    let mut sentence_tokens : Vec<(Vec<types::Token>, &types::Sentence)> = Vec::new();
    for (_, sentences) in SENTENCES.iter() {
        for sentence in sentences {
            let tokens = parser::Parser::parse(&sentence.text);
            println!("{:?}", tokens);
            sentence_tokens.push((tokens.unwrap(), &sentence));
        }
    }

    let test_data = "120 + 30%";
    let result = parser::Parser::parse(test_data);
    println!("{:?}", result);

}
