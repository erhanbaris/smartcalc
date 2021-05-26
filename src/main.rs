extern crate libsmartcalc;

use libsmartcalc::executer::{initialize};
use libsmartcalc::app::SmartCalc;

fn main() {
    let test_data = r"
date information = 11:30
date information add 1 hour 1 minute 31 second".to_string();
    initialize();

    let app = SmartCalc::default();
    let language = "en".to_string();
    let results = app.execute(&language, &test_data);
    
    for result in results {
        match result {
            Ok((tokens, ast)) => {
                println!("{:?}", tokens);
                println!("{}", app.format_result(&language, ast))              
            },
            Err(error) => println!("Error : {}", error)
        };
    }
}
