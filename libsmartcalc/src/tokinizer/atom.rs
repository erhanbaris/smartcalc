use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::borrow::ToOwned;

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

pub fn get_atom(data: &String, group_item: &Vec<Regex>) -> Vec<(usize, usize, Option<TokenType>, String)> {
    let mut atoms = Vec::new();

    for re in group_item.iter() {
        for capture in re.captures_iter(&data) {
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
                    //println!("Type not found, {}", atom_type);
                    return Vec::new()
                }
            };

            atoms.push((capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), Some(token_type), capture.get(0).unwrap().as_str().to_string()))
        }
    }
    atoms
}


pub fn atom_regex_parser(tokinizer: &mut Tokinizer, group_item: &Vec<Regex>) {
    let atoms =  get_atom(&tokinizer.data.to_owned(), group_item);
    for (start, end, token_type, text) in atoms {
        tokinizer.add_token_location(start, end, token_type, text);
    }
}

#[cfg(test)]
#[test]
fn operator_test() {
    use crate::tokinizer::test::setup;
    let tokinizer_mut = setup("[OPERATOR:+] [PERCENT:-29.1] [TIME:44100]  [NUMBER:-222.333] [MONEY:200;try]".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    let tokens = &tokinizer_mut.borrow().token_locations;

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 12);
    assert_eq!(tokens[0].token_type, Some(TokenType::Operator('+')));

    assert_eq!(tokens[1].start, 13);
    assert_eq!(tokens[1].end, 28);
    assert_eq!(tokens[1].token_type, Some(TokenType::Percent(-29.1)));

    assert_eq!(tokens[2].start, 29);
    assert_eq!(tokens[2].end, 41);
    assert_eq!(tokens[2].token_type, Some(TokenType::Time(NaiveTime::from_hms(12, 15, 0))));

    assert_eq!(tokens[3].start, 43);
    assert_eq!(tokens[3].end, 60);
    assert_eq!(tokens[3].token_type, Some(TokenType::Number(-222.333)));

    assert_eq!(tokens[4].start, 61);
    assert_eq!(tokens[4].end, 76);
    assert_eq!(tokens[4].token_type, Some(TokenType::Money(200.0, "try".to_string())));
}
