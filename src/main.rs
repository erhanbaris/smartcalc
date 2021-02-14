extern crate libsmartcalc;

use libsmartcalc::executer::{execute, initialize};
use libsmartcalc::formatter::format_results;

fn main() {
    let test_data = r"1230,1234 tl as usd".to_string();
    initialize();
    let results = execute(&test_data, &"en".to_string());
    let formated_results = format_results(&results);
    println!("{:#?}", formated_results);
    //println!("{:#?}", execute(&test_data, &"en".to_string()));
}

/*
cd libsmartcalc
cargo build --target wasm32-unknown-unknown --release
wasm-pack build
cd ../www/
npm run start
*/
