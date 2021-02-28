use alloc::string::String;
use alloc::string::ToString;

use alloc::collections::btree_map::BTreeMap;

use crate::{tokinizer::Tokinizer, types::{TokenType}};
use crate::tokinizer::{TokenInfo};


pub fn time_for_location(_: &Tokinizer, atoms: &BTreeMap<String, &TokenInfo>) -> core::result::Result<TokenType, String> {
    match &atoms.get("location").unwrap().token_type {
        Some(TokenType::Text(_location)) => /*{
            let json_data = fs::read_to_string("/Users/erhanbaris/ClionProjects/smartcalculator/smartcalc/src/json/city_informations.json").expect("{}");
            let json_value: Result<Value> = from_str(&json_data);

            return match json_value {
                Ok(data) => {
                    for item in data.as_array().unwrap() {
                        if let Value::String(city) = item.get("city_ascii").unwrap() {

                            if city.to_lowercase() == location.to_lowercase() {
                                let timezone = item.get("timezone").unwrap().as_str().unwrap();
                                let tz: Tz = match timezone.parse() {
                                    Ok(v) => v,
                                    Err(_) => return Err("Time not found".to_string())
                                };
                                return Ok(TokenType::Time(Utc::now().with_timezone(&tz).naive_local().time()));
                            }
                        }
                    }

                    Err("Time not found".to_string())
                },
                Err(error) => {
                    //println!("{}", error);
                    Err("Internal error".to_string())
                }
            };
        }*/ (),
        _ => ()
    };

    Err("Location not found".to_string())
}
