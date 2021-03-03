extern crate libsmartcalc;

use libsmartcalc::executer::{execute, initialize};
use libsmartcalc::formatter::format_result;
use libsmartcalc::constants::{FORMATS};

fn main() {
    let test_data = r"1/1/2000 to 3/3/2021".to_string();
    initialize();
    let language = "en".to_string();
    let results = execute(&language, &test_data);
    
    for result in results {
        match result {
            Ok((tokens, ast)) => {
                println!("{:?}", tokens);
                match FORMATS.read().unwrap().get(&language) {
                    Some(formats) => println!("{}", format_result(formats, ast)),
                    _ => ()
                }
            },
            Err(error) => println!("Error : {}", error)
        };
    }
}
