use std::rc::Rc;
use crate::types::*;
use crate::tokinizer::Tokinizer;

pub fn field_parser(tokinizer: &mut Tokinizer) -> TokenParserResult {
    if tokinizer.get_char() != '{' {
        return Ok(false);
    }

    tokinizer.increase_index();

    let mut ch: char      = '\0';
    let mut ch_next: char;
    let mut start                     = tokinizer.index as usize;
    let start_column                  = tokinizer.column;
    let mut end                       = start;
    let mut field_type:Option<String> = None;

    while !tokinizer.is_end() {
        ch      = tokinizer.get_char();
        ch_next = tokinizer.get_next_char();

        if (ch == '\\' && ch_next == '}') || (ch == '\\' && ch_next == '{') {
            end += ch.len_utf8();
            tokinizer.increase_index();
        }
        else if ch == '\\' && ch_next == ':' {
            end += ch.len_utf8();
            tokinizer.increase_index();
        }
        else if ch == '}' {
            tokinizer.increase_index();
            break;
        }
        else if ch == ':' {
            field_type = Some(tokinizer.data[start..end].to_string());
            start = tokinizer.index as usize;
        }
        else {
            end += ch.len_utf8();
        }

        tokinizer.increase_index();
    }

    if ch != '}' {
        return Err(("Missing '}' deliminator", tokinizer.column));
    }

    if field_type.is_none() {
        return Err(("Field type not found", tokinizer.column))
    }

    start += 1;
    end   += 1;

    let field = match field_type.unwrap().as_str() {
        "DATE" => FieldType::Date(tokinizer.data[start..end].to_string()),
        "TIME" => FieldType::Time(tokinizer.data[start..end].to_string()),
        "NUMBER" => FieldType::Number(tokinizer.data[start..end].to_string()),
        "TEXT" => FieldType::Text(tokinizer.data[start..end].to_string()),
        "MONEY" => FieldType::Money(tokinizer.data[start..end].to_string()),
        "PERCENT" => FieldType::Percent(tokinizer.data[start..end].to_string()),
        _ => return Err(("Field type not found", tokinizer.column))
    };

    tokinizer.add_token(start_column, TokenType::Field(Rc::new(field)));
    Ok(true)
}