extern crate libsmartcalc;

use libsmartcalc::executer::{execute, initialize};

fn main() {
    let test_data = r"tarih = 11:30
tarih add -1 hour".to_string();
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
