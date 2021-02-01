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