use alloc::{string::String};
use alloc::format;
use alloc::string::ToString;
use core::write;
use alloc::fmt::Write;
use chrono::{Local, Datelike};

use crate::types::{BramaAstType};
use crate::constants::{DurationFormatType, JsonFormat, MonthInfo};

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
        f = 10.0 * f;
    }
    
    while (f.round() - f).abs() > eps {
        f = 10.0 * f;
    }
    
    f.round() as u64
}

fn left_padding<'a>(number: i64, size: usize) -> String {
    format!("{:0width$}", &number, width = size.into())
}

fn format_number(number: f64, thousands_separator: String, decimal_separator: String, decimal_digits: u8, remove_fract_if_zero: bool, use_fract_rounding: bool) -> String {
    let divider      = 10_u32.pow(decimal_digits.into());
    let fract_number = (number * divider as f64).round() / divider as f64;
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
    
    if fract_part > 0 || !remove_fract_if_zero {
        trunc_formated.push_str(&decimal_separator);

        for index in (trunc_size+1)..formated_number.len() {
            trunc_formated.push(formated_number.chars().nth(index).unwrap());
        }
    }

    trunc_formated
}

fn duration_formatter<'a, 'b>(format: &'a JsonFormat, buffer: &mut String, replace_str: &'b str, duration: i64, duration_type: DurationFormatType) {

    for format_item in format.duration.iter() {
        if format_item.duration_type == duration_type {
            if format_item.count.trim().parse::<i64>().is_ok() && format_item.count.trim().parse::<i64>().unwrap() == duration {
                write!(buffer, "{} ", format_item.format.to_string().replace(replace_str, &duration.to_string())).unwrap();
                return;
            }
        }
    }

    for format_item in format.duration.iter() {
        if format_item.duration_type == duration_type && format_item.count.trim().parse::<i64>().is_err() {
            write!(buffer, "{} ", format_item.format.to_string().replace(replace_str, &duration.to_string())).unwrap();
            return;
        }
    }

    write!(buffer, "{} ", duration.to_string()).unwrap();
}

fn get_month<'b>(language: &'b str, month: u8) -> Option<MonthInfo> {
    match MONTHS_REGEXES.read() {
        Ok(month_regexes) => match month_regexes.get(language) {
            Some(month_list) => month_list.get((month - 1) as usize).map(|(_, month)| month.clone()),
            None => None
        },
        Err(_) => None
    }
}
fn uppercase_first_letter<'a>(s: &'a str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn format_result<'a>(format: &'a JsonFormat, result: alloc::rc::Rc<BramaAstType>) -> String {
    match &*result {
        BramaAstType::Money(price, currency) => {
            match CURRENCIES.read().unwrap().get(&currency.to_lowercase()) {
                Some(currency_detail) => {
                    let formated_price = format_number(*price, currency_detail.thousands_separator.to_string(), currency_detail.decimal_separator.to_string(), currency_detail.decimal_digits, false, true);
                    match (currency_detail.symbol_on_left, currency_detail.space_between_amount_and_symbol) {
                        (true, true) => format!("{} {}", currency_detail.symbol, formated_price),
                        (true, false) => format!("{}{}", currency_detail.symbol, formated_price),
                        (false, true) => format!("{} {}", formated_price, currency_detail.symbol),
                        (false, false) => format!("{}{}", formated_price, currency_detail.symbol),
                    }
                },
                _ => format!("{} {}", format_number(*price, ".".to_string(), ",".to_string(), 2, false, true), currency)
            }
        },
        BramaAstType::Number(number) => format_number(*number, ".".to_string(), ",".to_string(), 3, true, true),
        BramaAstType::Time(time) => time.to_string(),
        BramaAstType::Percent(percent) => format!("%{:}", format_number(*percent, ".".to_string(), ",".to_string(), 2, true, true)),
        BramaAstType::Date(date) => {

            let date_format = match date.year() == Local::now().date().year() {
                true => format.date.get("current_year"),
                false => format.date.get("full_date")
            };

            match date_format {
                Some(data) => {
                    match get_month(&format.language, date.month() as u8) {
                        Some(month_info) => data.clone()
                            .replace("{day}", &date.day().to_string())
                            .replace("{month}", &date.month().to_string())
                            .replace("{day_pad}", &left_padding(date.day().into(), 2))
                            .replace("{month_pad}", &left_padding(date.month().into(), 2))
                            .replace("{month_long}", &uppercase_first_letter(&month_info.long))
                            .replace("{month_short}", &uppercase_first_letter(&month_info.short))
                            .replace("{year}", &date.year().to_string()),
                        None => date.to_string()
                    }
                },
                None => date.to_string()
            }
        },
        BramaAstType::Duration(duration_object) => {
            let mut buffer = String::new();

            let mut duration = duration_object.num_seconds().abs();
            if duration >= YEAR {
                duration_formatter(format, &mut buffer, "{year}", duration / YEAR, DurationFormatType::Year);
                duration %= YEAR;
            }
    
            if duration >= MONTH {
                duration_formatter(format, &mut buffer, "{month}", duration / MONTH, DurationFormatType::Month);
                duration %= MONTH;
            }
    
            if duration >= WEEK {
                duration_formatter(format, &mut buffer, "{week}", duration / WEEK, DurationFormatType::Week);
                duration %= WEEK;
            }
    
            if duration >= DAY {
                duration_formatter(format, &mut buffer, "{day}", duration / DAY, DurationFormatType::Day);
                duration %= DAY;
            }
    
            if duration >= HOUR {
                duration_formatter(format, &mut buffer, "{hour}", duration / HOUR, DurationFormatType::Hour);
                duration %= HOUR;
            }
    
            if duration >= MINUTE {
                duration_formatter(format, &mut buffer, "{minute}", duration / MINUTE, DurationFormatType::Minute);
                duration %= MINUTE;
            }
    
            if duration > 0 {
                duration_formatter(format, &mut buffer, "{second}", duration, DurationFormatType::Second);
            }
    
            
            buffer
        },
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
    use chrono::NaiveTime;
    use crate::executer::initialize;
    initialize();

    match FORMATS.read().unwrap().get("en") {
        Some(formats) => {
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(0.0, "usd".to_string()))), "$0.00".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(0.05555, "usd".to_string()))), "$0.06".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(123.05555, "usd".to_string()))), "$123.06".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(1234.05555, "usd".to_string()))), "$1,234.06".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(123456.05555, "usd".to_string()))), "$123,456.06".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(123456.0, "usd".to_string()))), "$123,456.00".to_string());

            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(0.0, "try".to_string()))), "₺0,00".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(0.05555, "try".to_string()))), "₺0,06".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(123.05555, "try".to_string()))), "₺123,06".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(1234.05555, "try".to_string()))), "₺1.234,06".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(123456.05555, "try".to_string()))), "₺123.456,06".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(123456.0, "try".to_string()))), "₺123.456,00".to_string());

            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(0.0, "UZS".to_string()))), "0,00 сўм".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(0.05555, "UZS".to_string()))), "0,06 сўм".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(123.05555, "UZS".to_string()))), "123,06 сўм".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(1234.05555, "UZS".to_string()))), "1 234,06 сўм".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(123456.05555, "UZS".to_string()))), "123 456,06 сўм".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(123456.0, "UZS".to_string()))), "123 456,00 сўм".to_string());

            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(0.0, "UYU".to_string()))), "$U 0,00".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(0.05555, "UYU".to_string()))), "$U 0,06".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(123.05555, "UYU".to_string()))), "$U 123,06".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(1234.05555, "UYU".to_string()))), "$U 1.234,06".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(123456.05555, "UYU".to_string()))), "$U 123.456,06".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Money(123456.0, "UYU".to_string()))), "$U 123.456,00".to_string());

            assert_eq!(format_result(formats, Rc::new(BramaAstType::Number(123456.123456789))), "123.456,123".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Number(1.123456789))), "1,123".to_string());
            
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Time(NaiveTime::from_hms(11, 30, 0)))), "11:30:00".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Time(NaiveTime::from_hms(0, 0, 0)))), "00:00:00".to_string());
            
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Percent(0.0))), "%0".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Percent(10.0))), "%10".to_string());
            assert_eq!(format_result(formats, Rc::new(BramaAstType::Percent(10.1))), "%10,10".to_string());
        },
        _ => assert!(false)
    }
}
