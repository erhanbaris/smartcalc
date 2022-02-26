extern crate smartcalc;

fn main() {
    use smartcalc::SmartCalc;
    use num_format::SystemLocale;
    let locale = SystemLocale::default().unwrap();

    let test_data = r"11:50".to_string();
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
