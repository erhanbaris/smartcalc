use std::rc::Rc;
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
            Token::Time(NaiveTime::from_num_seconds_from_midnight(seconds, 0))
        },
        _ => return Err(("Atom type not found", tokinizer.column))
    };

    tokinizer.add_token(start_column, token);
    Ok(true)
}