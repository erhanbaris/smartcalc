extern crate libsmartcalc;

use libsmartcalc::executer::{execute, initialize};
use libsmartcalc::formatter::format_result;

fn main() {
    let test_data = r"aysel = 15739 dkk
erhan = 27401 dkk
nakit = erhan + aysel 
sigorta geri ödemesi = 8600DKK 

ev vergisi = 7975,19
araba kredisi = 3914
ev içi sigortarsı = 2989,96
ev sigortası = 8955,01
elektrik faturası = 2913,80
gaz faturası = 991,59
eon faturası = 350DKK


$1k earninng / 5 people asdada = $200
    
    ".to_string();
    initialize();
    let results = execute(&test_data, &"en".to_string());
    
    for result in results {
        match result {
            Ok((tokens, ast)) => {
                println!("{:?}", tokens);
                println!("{}", format_result(ast));
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
