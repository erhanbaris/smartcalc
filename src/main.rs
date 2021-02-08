extern crate libsmartcalc;

use libsmartcalc::executer::{execute, initialize};

fn main() {
    let test_data = r"7975,19 kr + 3914,00 kr + 2989,96 kr + 8955,01 kr + 2913,80 kr + 991,59 kr + 350,00 kr".to_string();
    initialize();
    println!("{:#?}", execute(&test_data, &"en".to_string()));
}

/*
cd libsmartcalc
cargo build --target wasm32-unknown-unknown --release
wasm-pack build
cd ../www/
npm run start
*/
