extern crate libsmartcalc;

use libsmartcalc::executer::{execute, initialize};
use libsmartcalc::formatter::format_result;
use libsmartcalc::constants::{FORMATS};

fn main() {
    let test_data = r"10 weeks".to_string();
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

/*
cd libsmartcalc
cargo build --target wasm32-unknown-unknown --release
wasm-pack build
cd ../www/
npm run start
*/
