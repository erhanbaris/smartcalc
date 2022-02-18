extern crate libsmartcalc;

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
    use libsmartcalc::app::SmartCalc;
    use libsmartcalc::executer::initialize;
    use num_format::SystemLocale;

    let locale = SystemLocale::default().unwrap();

    let test_data = r"15.5
15,5".to_string();
    initialize();

    let mut app = SmartCalc::default();
    app.config.decimal_seperator = locale.decimal().to_string();
    app.config.thousand_separator = locale.separator().to_string();
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
