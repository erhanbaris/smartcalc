/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::{string::String};
use alloc::format;
use alloc::string::ToString;
use crate::session::Session;
use crate::tools::do_divition;
use core::ops::Deref;

use crate::config::SmartCalcConfig;
use crate::types::{SmartCalcAstType};
use crate::constants::MonthInfo;

pub const MINUTE: i64 = 60;
pub const HOUR: i64 = MINUTE * 60;
pub const DAY: i64 = HOUR * 24;
pub const WEEK: i64 = DAY * 7;
pub const MONTH: i64 = DAY * 30;
pub const YEAR: i64 = DAY * 365;

fn fract_information(f: f64) -> u64 {
    let eps = 1e-4;
    let mut f = f.abs().fract();
    if f == 0.0 { return 0; }
    
    while (f.round() - f).abs() <= eps {
        f *= 10.0;
    }
    
    while (f.round() - f).abs() > eps {
        f *= 10.0;
    }
    
    f.round() as u64
}

pub fn left_padding(number: i64, size: usize) -> String {
    format!("{:0width$}", &number, width = size)
}

pub fn format_number(number: f64, thousands_separator: String, decimal_separator: String, decimal_digits: u8, remove_fract_if_zero: bool, use_fract_rounding: bool) -> String {
    let divider      = 10_u32.pow(decimal_digits.into());
    let fract_number = do_divition((number * divider as f64).round(), divider as f64);
    let trunc_part   = fract_number.trunc().abs().to_string();

    let formated_number = match use_fract_rounding {
        true => format!("{:.width$}", &number.abs(), width = decimal_digits.into()),
        false => format!("{}", &number.abs())
    };

    let fract_part = fract_information(fract_number.fract());
    let trunc_size = trunc_part.len();
    let mut trunc_dot_index = 3 - (trunc_part.len() % 3);
    let mut trunc_formated = String::new();


    if number < 0.0 {
        trunc_formated.push('-');
    }

    for index in 0..trunc_size {
        trunc_formated.push(formated_number.chars().nth(index).unwrap());
        trunc_dot_index += 1;
        if trunc_size != (index + 1) && trunc_dot_index % 3 == 0 {
            trunc_formated.push_str(&thousands_separator);
        }
    }
    
    if (fract_part > 0 || !remove_fract_if_zero) && trunc_size != formated_number.len() {
        trunc_formated.push_str(&decimal_separator);

        for index in (trunc_size+1)..formated_number.len() {
            trunc_formated.push(formated_number.chars().nth(index).unwrap());
        }
    }

    trunc_formated
}

pub fn get_month_info(config: &SmartCalcConfig, language: &'_ str, month: u8) -> Option<MonthInfo> {
    match config.month_regex.get(language) {
        Some(month_list) => month_list.get((month - 1) as usize).map(|(_, month)| month.clone()),
        None => None
    }
}

pub fn uppercase_first_letter(s: &'_ str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn format_result(config: &SmartCalcConfig, session: &Session, result: alloc::rc::Rc<SmartCalcAstType>) -> String {
    match result.deref() {
        SmartCalcAstType::Item(item) => item.print(config, session),
        _ => "".to_string()
    }
}

#[cfg(test)]
#[test]
fn get_frac_test() {
    assert_eq!(fract_information(0.1234567), 1234567);
    assert_eq!(fract_information(987654321.987), 987);
}

#[cfg(test)]
#[test]
fn format_number_test() {
    assert_eq!(format_number(123.0, ",".to_string(), ".".to_string(), 2, false, true), "123.00".to_string());
    assert_eq!(format_number(123.1, ",".to_string(), ".".to_string(), 2, false, true), "123.10".to_string());
    assert_eq!(format_number(123.01, ",".to_string(), ".".to_string(), 2, false, true), "123.01".to_string());
    assert_eq!(format_number(1234.01, ",".to_string(), ".".to_string(), 2, false, true), "1,234.01".to_string());
    assert_eq!(format_number(123456.01, ",".to_string(), ".".to_string(), 2, false, true), "123,456.01".to_string());
    assert_eq!(format_number(123456.123456789, ",".to_string(), ".".to_string(), 2, false, true), "123,456.12".to_string());
    assert_eq!(format_number(123456.1, ",".to_string(), ".".to_string(), 2, false, true), "123,456.10".to_string());
    assert_eq!(format_number(-123456.1, ",".to_string(), ".".to_string(), 2, false, true), "-123,456.10".to_string());

    assert_eq!(format_number(123.0, ",".to_string(), ".".to_string(), 2, true, false), "123".to_string());
    assert_eq!(format_number(123.0000, ",".to_string(), ".".to_string(), 2, true, false), "123".to_string());
    assert_eq!(format_number(123.1, ",".to_string(), ".".to_string(), 2, false, false), "123.1".to_string());
    assert_eq!(format_number(123.01, ",".to_string(), ".".to_string(), 2, false, false), "123.01".to_string());
    assert_eq!(format_number(1234.01, ",".to_string(), ".".to_string(), 2, false, false), "1,234.01".to_string());
    assert_eq!(format_number(123456.01, ",".to_string(), ".".to_string(), 2, false, false), "123,456.01".to_string());
    assert_eq!(format_number(123456.123456789, ",".to_string(), ".".to_string(), 2, false, false), "123,456.123456789".to_string());
    assert_eq!(format_number(123456.1, ",".to_string(), ".".to_string(), 2, false, false), "123,456.1".to_string());
    assert_eq!(format_number(-123456.1, ",".to_string(), ".".to_string(), 2, false, false), "-123,456.1".to_string());
}

#[cfg(test)]
#[test]
fn format_result_test() {
    use alloc::rc::Rc;
    use crate::compiler::DataItem;
    use crate::compiler::number::NumberItem;
    use crate::compiler::time::TimeItem;
    use crate::config::SmartCalcConfig;
    use crate::types::NumberType;
    let config = SmartCalcConfig::default();

    let mut session = Session::default();
    session.set_language("en".to_string());
    assert_eq!(NumberItem(123456.123456789, NumberType::Decimal).print(&config, &session), "123.456,12".to_string());
    assert_eq!(NumberItem(1.123456789, NumberType::Decimal).print(&config, &session), "1,12".to_string());
    assert_eq!(NumberItem(2.0, NumberType::Hexadecimal).print(&config, &session), "0x2".to_string());
            
    assert_eq!(format_result(&config, &session, Rc::new(SmartCalcAstType::Item(Rc::new(TimeItem(chrono::Utc::today().and_hms(11, 30, 0).naive_utc(), config.get_time_offset()))))), "11:30:00 UTC".to_string());
    assert_eq!(format_result(&config, &session, Rc::new(SmartCalcAstType::Item(Rc::new(TimeItem(chrono::Utc::today().and_hms(0, 0, 0).naive_utc(), config.get_time_offset()))))), "00:00:00 UTC".to_string());
}
