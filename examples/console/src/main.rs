/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

 extern crate smartcalc;

fn main() {
    use smartcalc::SmartCalc;
    use chrono_tz::Tz;
    use chrono_tz::OffsetName;
    use chrono::{TimeZone, Local};
    use num_format::SystemLocale;
    let locale = SystemLocale::default().unwrap();
    let timezone = match localzone::get_local_zone() {
        Some(tz) => match tz.parse::<Tz>() {
            Ok(tz) => {
                let date_time = Local::today().naive_local();
                tz.offset_from_utc_date(&date_time).abbreviation().to_string()
            },
            Err(_) => "UTC".to_string()
        },
        None => "UTC".to_string()
    };

    let test_data = r"123 dm".to_string();
    let mut app = SmartCalc::default();
    
    app.set_decimal_seperator(locale.decimal().to_string());
    app.set_thousand_separator(locale.separator().to_string());
    app.set_timezone(timezone).unwrap();

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
