use crate::types::*;

pub fn atom_parser(tokinizer: &mut Tokinizer) -> TokenParserResult {
    if tokinizer.get_char() != '{' {
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
            atom_type = Some(tokinizer.data[start..end].to_string());
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

    if atom_type.is_none() {
        return Err(("Atom type not found", tokinizer.column))
    }

    start += 1;
    end   += 1;

    let atom = match atom_type.unwrap().as_str() {
        "DATE" => BramaTokenType::Atom(AtomType::Date(tokinizer.data[start..end].to_string())),
        "TIME" => BramaTokenType::Atom(AtomType::Time(tokinizer.data[start..end].to_string())),
        "NUMBER" => BramaTokenType::Atom(AtomType::Number(tokinizer.data[start..end].to_string())),
        "TEXT" => BramaTokenType::Atom(AtomType::Text(tokinizer.data[start..end].to_string())),
        "MONEY" => BramaTokenType::Atom(AtomType::Money(tokinizer.data[start..end].to_string())),
        "PERCENT" => BramaTokenType::Atom(AtomType::Percent(tokinizer.data[start..end].to_string())),
        _ => return Err(("Atom type not found", tokinizer.column))
    };

    tokinizer.add_token(start_column, atom);
    Ok(true)
}