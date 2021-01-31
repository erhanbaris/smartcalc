use regex::Regex;
use crate::tokinizer::Tokinizer;
use crate::types::TokenType;
use chrono::NaiveTime;

pub fn time_regex_parser(tokinizer: &mut Tokinizer, data: &mut String, group_item: &Vec<Regex>) -> String {
    let mut data_str = data.to_string();

    for re in group_item.iter() {
        for capture in re.captures_iter(data) {
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
            if tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), TokenType::Time(NaiveTime::from_num_seconds_from_midnight(time_number, 0))) {
                data_str = data_str.replace(capture.get(0).unwrap().as_str(), &format!("[TIME:{}]", time_number)[..]);
            }
        }
    }

    data_str
}