use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;
use alloc::string::ToString;
use log;

use crate::tokinizer::{TokenLocation};
use crate::types::{BramaAstType};
use crate::constants::{CURRENCIES};


fn get_frac(f: f64) -> u64 {
    
    let eps = 1e-4;
    let mut f = f.abs().fract();
    if f == 0.0 { return 0; }
    
    while (f.round() - f).abs() <= eps {
        f = 10.0 * f;
    }
    
    while (f.round() - f).abs() > eps {
        f = 10.0 * f;
    }
    
    return f.round() as u64;
}

fn convert_money(price: f64, currency: &String) -> String {
    let trunc_part = price.trunc().to_string();
    let fract_part = get_frac(price).to_string();
    let trunc_size = trunc_part.len();
    let mut trunc_dot_index = 3 -  trunc_size % 3;
    let mut trunc_formated = String::new();

    for index in 0..trunc_size {
        trunc_formated.push(trunc_part.chars().nth(index).unwrap());
        trunc_dot_index += 1;
        if trunc_size != (index + 1) && trunc_dot_index % 3 == 0 {
            trunc_formated.push('.');
        }
    }
    
    log::info!("{}", trunc_formated);
    trunc_formated
}

fn format_number(price: f64, thousandsSeparator: String, decimalSeparator: String, decimalDigits: u8) -> String {
    let trunc_part = price.trunc().to_string();

    let decimal_format_divider = 10_u32.pow(decimalDigits.into());
    let fract_number : f64 = (price.fract() * decimal_format_divider as f64).round() / decimal_format_divider as f64;

    let fract_part = get_frac(fract_number);
    let trunc_size = trunc_part.len();
    let mut trunc_dot_index = 3 -  trunc_size % 3;
    let mut trunc_formated = String::new();

    for index in 0..trunc_size {
        trunc_formated.push(trunc_part.chars().nth(index).unwrap());
        trunc_dot_index += 1;
        if trunc_size != (index + 1) && trunc_dot_index % 3 == 0 {
            trunc_formated.push_str(&thousandsSeparator);
        }
    }
    
    trunc_formated.push_str(&decimalSeparator);
    let formatted_fract = format!("{:0width$}", &fract_part, width = decimalDigits.into());
    trunc_formated.push_str(&formatted_fract);
    trunc_formated
}


pub fn format_results(results: &Vec<Result<(Vec<TokenLocation>, alloc::rc::Rc<BramaAstType>), String>> ) -> Vec<String> {
    let mut response = Vec::new();
    
    for result in results {
        let result_item = match result {
            Ok((_, line_result)) => {
                match &**line_result {
                    BramaAstType::Money(price, currency) => {
                        match CURRENCIES.read().unwrap().get(currency) {
                            Some(currency_detail) => {
                                let formated_price = format_number(*price, currency_detail.thousandsSeparator.to_string(), currency_detail.decimalSeparator.to_string(), currency_detail.decimalDigits);
                                match (currency_detail.symbolOnLeft, currency_detail.spaceBetweenAmountAndSymbol) {
                                    (true, true) => format!("{} {}", currency_detail.symbol, formated_price),
                                    (true, false) => format!("{}{}", currency_detail.symbol, formated_price),
                                    (false, true) => format!("{} {}", formated_price, currency_detail.symbol),
                                    (false, false) => format!("{}{}", formated_price, currency_detail.symbol),
                                }
                            },
                            _ => format!("{} {}", format_number(*price, ".".to_string(), ",".to_string(), 2), currency)
                        }
                    },
                    BramaAstType::Number(number) => format_number(*number, ".".to_string(), ",".to_string(), 2),
                    BramaAstType::Time(time) => time.to_string(),
                    BramaAstType::Percent(percent) => format!("%{:}", percent),
                    _ => "".to_string()
                }
            },
            _ => "".to_string()
        };

        response.push(result_item);
    }

    response
}

pub fn format_result(result: &alloc::rc::Rc<BramaAstType>) -> String {
    match &**result {
        BramaAstType::Money(price, currency) => {
            match CURRENCIES.read().unwrap().get(currency) {
                Some(currency_detail) => {
                    let formated_price = format_number(*price, currency_detail.thousandsSeparator.to_string(), currency_detail.decimalSeparator.to_string(), currency_detail.decimalDigits);
                    match (currency_detail.symbolOnLeft, currency_detail.spaceBetweenAmountAndSymbol) {
                        (true, true) => format!("{} {}", currency_detail.symbol, formated_price),
                        (true, false) => format!("{}{}", currency_detail.symbol, formated_price),
                        (false, true) => format!("{} {}", formated_price, currency_detail.symbol),
                        (false, false) => format!("{}{}", formated_price, currency_detail.symbol),
                    }
                },
                _ => format!("{} {}", format_number(*price, ".".to_string(), ",".to_string(), 2), currency)
            }
        },
        BramaAstType::Number(number) => format_number(*number, ".".to_string(), ",".to_string(), 2),
        BramaAstType::Time(time) => time.to_string(),
        BramaAstType::Percent(percent) => format!("%{:}", percent),
        _ => "".to_string()
    }
}