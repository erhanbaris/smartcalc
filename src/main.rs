extern crate libsmartcalc;

use libsmartcalc::executer::{execute, initialize};

fn main() {
    let test_data = r"11:30 add 4 hours".to_string();
    initialize();
    println!("{:?}", execute(&test_data, &"en".to_string()));
}

/*
cd libsmartcalc
cargo build --target wasm32-unknown-unknown --release
wasm-pack build
cd ../www/
npm run start
*/
