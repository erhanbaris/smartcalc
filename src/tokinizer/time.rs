/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::string::ToString;
use alloc::borrow::ToOwned;
use regex::Regex;
use crate::config::SmartCalcConfig;
use crate::tokinizer::Tokinizer;
use crate::types::{TokenType, TimeOffset};
use crate::token::ui_token::{UiTokenType};
use chrono::{NaiveDateTime, Local, Utc, FixedOffset, Timelike, Datelike};

use chrono::{Offset, TimeZone};

pub fn time_regex_parser(config: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            let mut end_position = 0;
            let mut hour = capture.name("hour").unwrap().as_str().parse::<i32>().unwrap();
            let minute   = match capture.name("minute") {
                Some(minute) => {
                    end_position = minute.end();
                    minute.as_str().parse::<i32>().unwrap()
                },
                _ => 0
            };
            
            let second   = match capture.name("second") {
                Some(second) => {
                    end_position = second.end();
                    second.as_str().parse::<i32>().unwrap()
                },
                _ => 0
            };

            if let Some(meridiem) = capture.name("meridiem") {
                if meridiem.as_str().to_lowercase() == "pm" {
                    hour += 12;
                }
                end_position = meridiem.end();
            }

            let timezone_info = match capture.name("timezone") {
                Some(upper_tz_match) => {
                    match capture.name("timezone_1") {
                        Some(tz) => {
                            let timezone = tz.as_str().to_uppercase();
                            match config.timezones.get(&timezone) {
                                Some(offset) => {
                                    end_position = upper_tz_match.end();
                                    Some((timezone, *offset))
                                },
                                None => None
                            }
                        },
                        None => match capture.name("timezone_2") {
                            Some(tz) => {
                                end_position = upper_tz_match.end();

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
                },
                None => None
            };
            
            let (timezone, offset) = match timezone_info {
                Some((timezone, offset)) => (timezone, offset),
                None =>(config.timezone.to_string(), config.timezone_offset)
            };
            
            let time_offset = TimeOffset {
                name: timezone,
                offset
            };
            
            let date = Local::today().and_hms(hour as u32, minute as u32, second as u32);
            let datetime = FixedOffset::east(offset * 60).ymd(date.year(), date.month(), date.day()).and_hms(date.hour(), date.minute(), date.second());
            let date_as_utc = Utc.from_utc_datetime(&datetime.naive_utc()).naive_utc();            
            
            if tokinizer.add_token_location(capture.get(0).unwrap().start(), end_position, Some(TokenType::Time(date_as_utc, time_offset)), capture.get(0).unwrap().as_str().to_string()) {
                tokinizer.ui_tokens.add_from_regex_match(capture.get(0), UiTokenType::Time);
            }
        }
    }
}

#[cfg(test)]
#[test]
fn time_test() {
    use core::ops::Deref;
    use crate::tokinizer::test::setup_tokinizer;
    use core::cell::RefCell;
    use crate::config::SmartCalcConfig;
    use crate::app::Session;
    let session = RefCell::new(Session::new());
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("11:30 12:00 AM 1:20 3:30 PM 9:01 1pm 1am 0pm 0am".to_string(), &session, &config);

    tokinizer_mut.tokinize_with_regex();
    let tokens = &tokinizer_mut.session.borrow().token_infos;

    assert_eq!(tokens.len(), 9);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 5);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Time(chrono::Utc::today().and_hms(11, 30, 0).naive_utc(), config.get_time_offset())));

    assert_eq!(tokens[1].start, 6);
    assert_eq!(tokens[1].end, 14);
    assert_eq!(tokens[1].token_type.borrow().deref(), &Some(TokenType::Time(chrono::Utc::today().and_hms(12, 00, 0).naive_utc(), config.get_time_offset())));

    assert_eq!(tokens[2].start, 15);
    assert_eq!(tokens[2].end, 19);
    assert_eq!(tokens[2].token_type.borrow().deref(), &Some(TokenType::Time(chrono::Utc::today().and_hms(1, 20, 0).naive_utc(), config.get_time_offset())));

    assert_eq!(tokens[3].start, 20);
    assert_eq!(tokens[3].end, 27);
    assert_eq!(tokens[3].token_type.borrow().deref(), &Some(TokenType::Time(chrono::Utc::today().and_hms(15, 30, 0).naive_utc(), config.get_time_offset())));

    assert_eq!(tokens[4].start, 28);
    assert_eq!(tokens[4].end, 32);
    assert_eq!(tokens[4].token_type.borrow().deref(), &Some(TokenType::Time(chrono::Utc::today().and_hms(9, 1, 0).naive_utc(), config.get_time_offset())));

    assert_eq!(tokens[5].start, 33);
    assert_eq!(tokens[5].end, 36);
    assert_eq!(tokens[5].token_type.borrow().deref(), &Some(TokenType::Time(chrono::Utc::today().and_hms(13, 0, 0).naive_utc(), config.get_time_offset())));

    assert_eq!(tokens[6].start, 37);
    assert_eq!(tokens[6].end, 40);
    assert_eq!(tokens[6].token_type.borrow().deref(), &Some(TokenType::Time(chrono::Utc::today().and_hms(1, 0, 0).naive_utc(), config.get_time_offset())));

    assert_eq!(tokens[7].start, 41);
    assert_eq!(tokens[7].end, 44);
    assert_eq!(tokens[7].token_type.borrow().deref(), &Some(TokenType::Time(chrono::Utc::today().and_hms(12, 0, 0).naive_utc(), config.get_time_offset())));

    assert_eq!(tokens[8].start, 45);
    assert_eq!(tokens[8].end, 48);
    assert_eq!(tokens[8].token_type.borrow().deref(), &Some(TokenType::Time(chrono::Utc::today().and_hms(0, 0, 0).naive_utc(), config.get_time_offset())));
}

