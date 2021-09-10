use alloc::string::ToString;
use alloc::borrow::ToOwned;
use regex::Regex;
use crate::config::SmartCalcConfig;
use crate::tokinizer::Tokinizer;
use crate::types::{TokenType};
use crate::token::ui_token::{UiTokenType};
use chrono::NaiveTime;

pub fn time_regex_parser(_: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            let mut hour = capture.name("hour").unwrap().as_str().parse::<i32>().unwrap();
            let minute   = capture.name("minute").unwrap().as_str().parse::<i32>().unwrap();
            let second   = match capture.name("second") {
                Some(second) => second.as_str().parse::<i32>().unwrap(),
                _ => 0
            };

            if let Some(meridiem) = capture.name("meridiem") {
                if meridiem.as_str().to_lowercase() == "pm" {
                    hour += 12;
                }
            }

            let time_number: u32 = ((hour * 60 * 60) + (minute * 60) + second) as u32;
            if tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), Some(TokenType::Time(NaiveTime::from_num_seconds_from_midnight(time_number, 0))), capture.get(0).unwrap().as_str().to_string()) {
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
    let mut tokinizer_mut = setup_tokinizer("11:30 12:00 AM 1:20 3:30 PM 9:01".to_string(), &session, &config);

    tokinizer_mut.tokinize_with_regex();
    let tokens = &tokinizer_mut.session.borrow().token_infos;

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 5);
    assert_eq!(tokens[0].token_type.borrow().deref(), &Some(TokenType::Time(NaiveTime::from_hms(11, 30, 0))));

    assert_eq!(tokens[1].start, 6);
    assert_eq!(tokens[1].end, 14);
    assert_eq!(tokens[1].token_type.borrow().deref(), &Some(TokenType::Time(NaiveTime::from_hms(12, 00, 0))));

    assert_eq!(tokens[2].start, 15);
    assert_eq!(tokens[2].end, 19);
    assert_eq!(tokens[2].token_type.borrow().deref(), &Some(TokenType::Time(NaiveTime::from_hms(1, 20, 0))));

    assert_eq!(tokens[3].start, 20);
    assert_eq!(tokens[3].end, 27);
    assert_eq!(tokens[3].token_type.borrow().deref(), &Some(TokenType::Time(NaiveTime::from_hms(15, 30, 0))));

    assert_eq!(tokens[4].start, 28);
    assert_eq!(tokens[4].end, 32);
    assert_eq!(tokens[4].token_type.borrow().deref(), &Some(TokenType::Time(NaiveTime::from_hms(9, 1, 0))));
}

