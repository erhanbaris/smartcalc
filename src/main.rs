extern crate libsmartcalc;

use libsmartcalc::executer::{execute, initialize};
use libsmartcalc::formatter::format_result;
use libsmartcalc::constants::{FORMATS};

fn main() {
    let test_data = r"today + 3 weeks".to_string();
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
//[UiToken { start: 0, end: 2, ui_type: Number }, UiToken { start: 2, end: 6, ui_type: Text }]
//[UiToken { start: 0, end: 2, ui_type: Number }, UiToken { start: 3, end: 7, ui_type: Text }]
