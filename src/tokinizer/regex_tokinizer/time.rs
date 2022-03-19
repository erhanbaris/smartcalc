/*
 * smartcalc v1.0.7
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
use chrono::{Utc, FixedOffset, Datelike};

use chrono::{TimeZone};

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
                if meridiem.as_str().to_lowercase() == "pm" && hour < 12 && hour >= 0 {
                    hour += 12;
                }
                end_position = meridiem.end();
            }
            
            let time_offset = TimeOffset {
                name: config.timezone.to_string(),
                offset: config.timezone_offset
            };
            
            let date = Utc::today().naive_utc();
            let datetime = FixedOffset::east(time_offset.offset * 60).ymd(date.year(), date.month(), date.day()).and_hms(hour as u32, minute as u32, second as u32);
            let date_as_utc = Utc.from_utc_datetime(&datetime.naive_utc()).naive_utc();
            
            if tokinizer.add_token_location(capture.get(0).unwrap().start(), end_position, Some(TokenType::Time(date_as_utc, time_offset)), capture.get(0).unwrap().as_str().to_string()) {
                tokinizer.add_uitoken_from_match(capture.get(0), UiTokenType::DateTime);
            }
        }
    }
}

#[cfg(test)]
#[test]
fn time_test() {
    use core::ops::Deref;
    use chrono::NaiveTime;

    use crate::tokinizer::regex_tokinizer;
    use crate::tokinizer::test::setup_tokinizer;
    use crate::config::SmartCalcConfig;
    use crate::session::Session;
    let mut session = Session::new();
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("11:30 12:00 AM 1:20 3:30 PM 9:01 1pm 1am 0pm 0am 1am GMT+10:00 12:34 pm".to_string(), &mut session, &config);

    regex_tokinizer(&mut tokinizer_mut);
    let tokens = &tokinizer_mut.token_infos;

    assert_eq!(tokens.len(), 12);
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

    assert_eq!(tokens[9].start, 49);
    assert_eq!(tokens[9].end, 52);

    let time = tokens[9].token_type.borrow().deref().clone();

    if let Some(TokenType::Time(time, timezone)) = time {
        assert_eq!(time.time(), NaiveTime::from_hms(1, 0, 0));
        assert_eq!(timezone, TimeOffset {
            name: "UTC".to_string(),
            offset: 0
        });
    } else {
        assert_eq!(false, true);
    }
    
    assert_eq!(tokens[10].start, 53);
    assert_eq!(tokens[10].end, 62);
    assert_eq!(tokens[10].token_type.borrow().deref(), &Some(TokenType::Timezone("GMT+10:00".to_string(), 600)));

    assert_eq!(tokens[11].start, 63);
    assert_eq!(tokens[11].end, 71);
    assert_eq!(tokens[11].token_type.borrow().deref(), &Some(TokenType::Time(chrono::Utc::today().and_hms(12, 34, 0).naive_utc(), config.get_time_offset())));
}

