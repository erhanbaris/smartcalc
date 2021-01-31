use crate::types::*;
use crate::tokinizer::Tokinizer;
use chrono::NaiveTime;

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