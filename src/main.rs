extern crate libsmartcalc;

use libsmartcalc::executer::{initialize};
use libsmartcalc::app::SmartCalc;

#[cfg(feature="webserver")]
mod webserver;

#[cfg(feature="webserver")]
fn main() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        match webserver::start_webserver().await {
            Ok(_) => println!("Webserver stopped"),
            Err(error) => println!("Error : {:}", error),
        };
    });
}

#[cfg(not(feature="webserver"))]
fn main() {
    let test_data = r"
    12 january".to_string();
    initialize();

    let app = SmartCalc::default();
    let language = "en".to_string();
    let results = app.execute(language, test_data);
    
    for result in results.lines.iter() {
        match result {
            Some(result) => match &result.result {
                Ok(output) => {
                    println!("{:?}", result.ui_tokens);
                    println!("{}", output.output)
                },
                Err(error) => println!("Error : {}", error)
            },
            None => println!("No query")
        }
    }
}
