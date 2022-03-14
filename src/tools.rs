/*
 * smartcalc v1.0.6
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::string::{ToString, String};
use crate::config::SmartCalcConfig;
 
pub fn do_divition(left: f64, right: f64) -> f64 {
    let mut calculation = left / right;
    if calculation.is_infinite() || calculation.is_nan() {
        calculation = 0.0;
    }
    calculation
}

pub fn parse_timezone<'t>(config: &SmartCalcConfig, capture: &regex::Captures<'t>) -> Option<(String, i32)> {
    match capture.name("timezone_1") {
        Some(tz) => {
            let timezone = tz.as_str().to_uppercase();
            config.timezones.get(&timezone).map(|offset| (timezone, *offset))
        },
        None => match capture.name("timezone_2") {
            Some(tz) => {
                let hour = capture.name("timezone_hour").unwrap().as_str().parse::<i32>().unwrap();
                let minute   = match capture.name("timezone_minute") {
                    Some(minute) => {
                        minute.as_str().parse::<i32>().unwrap()
                    },
                    _ => 0
                };

                let timezone_type = match capture.name("timezone_type") {
                    Some(timezone_type) => match timezone_type.as_str() {
                        "-" => -1,
                        _ => 1
                    },
                    None => 1
                };

                Some((tz.as_str().to_string(), (hour * 60 + minute) * timezone_type))
            },
            None => None
        }
    }
}