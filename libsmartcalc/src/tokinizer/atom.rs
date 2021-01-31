use crate::types::*;
use crate::tokinizer::Tokinizer;
use chrono::NaiveTime;
use regex::Regex;

pub fn atom_parser(tokinizer: &mut Tokinizer) -> TokenParserResult {
    if tokinizer.get_char() != '[' {
        return Ok(false);
    }

    tokinizer.increase_index();

    let mut ch: char      = '\0';
    let mut ch_next: char;
    let mut start                    = tokinizer.index as usize;
    let start_column                 = tokinizer.column;
    let mut end                      = start;
    let mut atom_type:Option<String> = None;

    while !tokinizer.is_end() {
        ch      = tokinizer.get_char();
        ch_next = tokinizer.get_next_char();

        if (ch == '\\' && ch_next == ']') || (ch == '\\' && ch_next == '[') {
            end += ch.len_utf8();
            tokinizer.increase_index();
        }
        else if ch == '\\' && ch_next == ':' {
            end += ch.len_utf8();
            tokinizer.increase_index();
        }
        else if ch == ']' {
            tokinizer.increase_index();
            break;
        }
        else if ch == ':' {
            atom_type = Some(tokinizer.data[start..end].to_string());
            start = tokinizer.index as usize;
        }
        else {
            end += ch.len_utf8();
        }

        tokinizer.increase_index();
    }

    if ch != ']' {
        return Err(("Missing ']' deliminator", tokinizer.column));
    }

    if atom_type.is_none() {
        return Err(("Atom type not found", tokinizer.column))
    }

    start += 1;
    end   += 1;

    let token = match atom_type.unwrap().as_str() {
        "TIME" => {
            let seconds = tokinizer.data[start..end].to_string().parse::<u32>().unwrap();
            TokenType::Time(NaiveTime::from_num_seconds_from_midnight(seconds, 0))
        },
        "MONEY" => {
            let content = tokinizer.data[start..end].to_string();
            let splited_data: Vec<&str> = content.split(";").collect();
            TokenType::Money(splited_data[0].parse::<f64>().unwrap(), splited_data[1].to_string())
        },
        "NUMBER" => {
            let number = tokinizer.data[start..end].to_string().parse::<f64>().unwrap();
            TokenType::Number(number)
        },
        "PERCENT" => {
            let number = tokinizer.data[start..end].to_string().parse::<f64>().unwrap();
            TokenType::Percent(number)
        },
        "OPERATOR" => TokenType::Operator(tokinizer.data.chars().nth(start).unwrap()),
        _ => return Err(("Atom type not found", tokinizer.column))
    };

    tokinizer.add_token(start_column, token);
    Ok(true)
}

pub fn atom_regex_parser(tokinizer: &mut Tokinizer, data: &mut String, group_item: &Vec<Regex>) -> String {
    let mut data_str = data.to_string();

    for re in group_item.iter() {
        for capture in re.captures_iter(data) {
            let atom_type = capture.name("ATOM").unwrap().as_str();
            let data      = capture.name("DATA").unwrap().as_str();

            let token_type = match atom_type {
                "TIME" => {
                    let seconds = data.parse::<u32>().unwrap();
                    TokenType::Time(NaiveTime::from_num_seconds_from_midnight(seconds, 0))
                },
                "MONEY" => {
                    let splited_data: Vec<&str> = data.split(";").collect();
                    TokenType::Money(splited_data[0].parse::<f64>().unwrap(), splited_data[1].to_string())
                },
                "NUMBER" => {
                    let number = data.parse::<f64>().unwrap();
                    TokenType::Number(number)
                },
                "PERCENT" => {
                    let number = data.parse::<f64>().unwrap();
                    TokenType::Percent(number)
                },
                "OPERATOR" => TokenType::Operator(data.chars().nth(0).unwrap()),
                _ => {
                    println!("Type not found, {}", atom_type);
                    return data_str;
                }
            };

            tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), token_type);
        }
    }

    data_str
}