extern crate libsmartcalc;
extern crate any_ascii;
use any_ascii::any_ascii;

use libsmartcalc::executer::{execute, initialize};

fn main() {

    let s = any_ascii(&"erhan barış aysel barış test".to_string());

    let test_data = r"add hours hour 1024 percent".to_string();
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
