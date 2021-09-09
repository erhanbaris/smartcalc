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
    #123
    # 111".to_string();
    initialize();

    let app = SmartCalc::default();
    let language = "en".to_string();
    let results = app.execute(language, test_data);
    
    for result in results.lines.iter() {
        match result {
            Some(result) => match result {
                Ok(line) => {
                    println!("{:?}", line.ui_tokens);
                    println!("{:?}", line.tokens);
                    println!("{}", line.output)
                },
                Err(error) => println!("Error : {}", error)
            },
            None => println!("No query")
        }
    }
}
