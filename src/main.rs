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
    
    /*let config = SmartCalcConfig::default();
    
    let number = Rc::new(PercentItem(10.0));
    let percent = Rc::new(MoneyItem(2000.0, config.currency.get("try").unwrap().clone()));
        
    let aa = Operation::calculate(&config, number.deref(), percent.deref(), OperationType::Add);
    println!("{}", aa.unwrap().deref().print());
    */ 
    
    let test_data = r"22250mb - 250.1mb".to_string();
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
