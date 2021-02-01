use regex::Regex;
use crate::tokinizer::Tokinizer;
use crate::types::TokenType;
use chrono::NaiveTime;

pub fn time_regex_parser(tokinizer: &mut Tokinizer, group_item: &Vec<Regex>) {
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
            tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), Some(TokenType::Time(NaiveTime::from_num_seconds_from_midnight(time_number, 0))));
        }
    }
}

#[cfg(test)]
#[test]
fn time_test() {
    use crate::tokinizer::test::setup;
    let tokinizer_mut = setup("11:30 12:00 AM 1:20 3:30 PM 9:01".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    let tokens = &tokinizer_mut.borrow().token_locations;

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 5);
    assert_eq!(tokens[0].token_type, Some(TokenType::Time(NaiveTime::from_hms(11, 30, 0))));

    assert_eq!(tokens[1].start, 6);
    assert_eq!(tokens[1].end, 14);
    assert_eq!(tokens[1].token_type, Some(TokenType::Time(NaiveTime::from_hms(12, 00, 0))));

    assert_eq!(tokens[2].start, 15);
    assert_eq!(tokens[2].end, 19);
    assert_eq!(tokens[2].token_type, Some(TokenType::Time(NaiveTime::from_hms(1, 20, 0))));

    assert_eq!(tokens[3].start, 20);
    assert_eq!(tokens[3].end, 27);
    assert_eq!(tokens[3].token_type, Some(TokenType::Time(NaiveTime::from_hms(15, 30, 0))));

    assert_eq!(tokens[4].start, 28);
    assert_eq!(tokens[4].end, 32);
    assert_eq!(tokens[4].token_type, Some(TokenType::Time(NaiveTime::from_hms(9, 1, 0))));
}

