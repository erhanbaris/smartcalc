/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::string::ToString;
use chrono::{TimeZone, Offset, Local};
use chrono_tz::OffsetName;

use crate::{worker::tools::get_timezone, types::TimeOffset};

pub fn do_divition(left: f64, right: f64) -> f64 {
    let mut calculation = left / right;
    if calculation.is_infinite() || calculation.is_nan() {
        calculation = 0.0;
    }
    calculation
}

pub fn get_time_offset() -> TimeOffset {
    let date_time = Local::today().naive_local();
    let name = get_timezone().offset_from_utc_date(&date_time).abbreviation().to_string();
    let offset = get_timezone().offset_from_utc_date(&date_time).fix().utc_minus_local();
    TimeOffset {
        name,
        offset       
    }
}