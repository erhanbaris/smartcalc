extern crate libsmartcalc;

use libsmartcalc::executer::{execute, initialize};
use libsmartcalc::formatter::format_result;
use libsmartcalc::constants::{FORMATS};

fn main() {
    let test_data = r"22 dec 1985".to_string();
    initialize();
    let results = execute(&test_data, &"en".to_string());
    
    for result in results {
        match result {
            Ok((tokens, ast)) => {
                println!("{:?}", tokens);
                match FORMATS.read().unwrap().get("en") {
                    Some(formats) => println!("{}", format_result(formats, ast)),
                    _ => ()
                }
            },
            Err(error) => println!("Error : {}", error)
        };
    }
}
