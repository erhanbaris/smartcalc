use crate::types::*;
use crate::tokinizer::{Tokinizer};
use regex::Regex;

fn increase(tokinizer: &mut Tokinizer) -> char {
    tokinizer.increase_index();
    tokinizer.get_char()
}

fn get_digits(tokinizer: &mut Tokinizer) -> (u8, u64) {
    let mut number: u64    = 0;
    let mut num_count: u8  = 0;
    let mut ch :char       = tokinizer.get_char();

    while !tokinizer.is_end() && (ch.is_ascii_digit() || ch == '_') {
        if ch != '_' {
            num_count += 1;

            number *= u64::pow(10, 1);
            number += ch as u64 - '0' as u64;
        }

        ch = increase(tokinizer);
    }

    (num_count, number)
}

fn detect_number_system(tokinizer: &mut Tokinizer) -> BramaNumberSystem {
    if tokinizer.get_char() == '0' {
        return match tokinizer.get_next_char() {
            'b' | 'B' => {
                increase(tokinizer);
                increase(tokinizer);
                BramaNumberSystem::Binary
            },
            'x' | 'X' => {
                increase(tokinizer);
                increase(tokinizer);
                BramaNumberSystem::Hexadecimal
            },
            '0'..='7' => {
                increase(tokinizer);
                BramaNumberSystem::Octal
            },
            _ => BramaNumberSystem::Decimal
        };
    }

    BramaNumberSystem::Decimal
}

fn parse_hex(tokinizer: &mut Tokinizer) -> TokenType {
    let mut number :u64 = 0;
    let mut ch :char    = tokinizer.get_char();

    while !tokinizer.is_end() && ch.is_ascii_hexdigit() {
        number <<= 4;

        let tmp_ch = ch.to_digit(16);
        if let Some(num) = tmp_ch {
            number += num as u64;
        }

        ch = increase(tokinizer);
    }

    TokenType::Number(number as f64)
}

fn parse_octal(tokinizer: &mut Tokinizer) -> TokenType {
    let mut number :u64 = 0;
    let mut ch :char    = tokinizer.get_char();

    while !tokinizer.is_end() && ch >= '0' && ch <= '7' {
        number <<= 3;

        let tmp_ch = ch.to_digit(8);
        if let Some(num) = tmp_ch {
            number += num as u64;
        }

        ch = increase(tokinizer);
    }

    TokenType::Number(number as f64)
}

fn parse_binary(tokinizer: &mut Tokinizer) -> TokenType {
    let mut number :u64 = 0;
    let mut ch :char    = tokinizer.get_char();

    while !tokinizer.is_end() && ch >= '0' && ch <= '1' {
        number <<= 1;

        let tmp_ch = ch.to_digit(2);
        if let Some(num) = tmp_ch {
            number += num as u64;
        }

        ch = increase(tokinizer);
    }

    TokenType::Number(number as f64)
}

fn parse_decimal(tokinizer: &mut Tokinizer) -> TokenType {
    /*
    [NUMBER](.[NUMBER](E(-+)[NUMBER]))
    */
    let mut ch     = tokinizer.get_char();
    let multiplier = match ch {
        '-' => {
            increase(tokinizer);
            -1.0
        },

        '+' => {
            increase(tokinizer);
            1.0
        },
        _ => {
            1.0
        }
    };

    let (_, digits)  = get_digits(tokinizer);
    let before_comma = digits;
    let ch_next      = tokinizer.get_next_char();
    ch               = tokinizer.get_char();

    /* Double number */
    if !tokinizer.is_end() && ch == '.' && (ch_next >= '0' && ch_next <= '9') {
        increase(tokinizer);

        let (digit_num, digits) = get_digits(tokinizer);
        let after_comma = digits;
        let dot_place   = digit_num;
        ch          = tokinizer.get_char();

        if !tokinizer.is_end() && (ch == 'e' || ch == 'E') {
            let mut is_minus      = false;

            ch = increase(tokinizer);

            if !tokinizer.is_end() {
                match ch {
                    '-' => {
                        is_minus = true;
                        increase(tokinizer);
                    },

                    '+' => { increase(tokinizer); },
                    _ => {}
                }
            }

            let (_, digits) = get_digits(tokinizer);
            let e_after    = digits;
            increase(tokinizer);

            let num = before_comma as f64 + (after_comma as f64 * f64::powi(10.0, -1 * dot_place as i32));

            return match is_minus {
                true  => TokenType::Number(num / f64::powi(10.0, e_after as i32)),
                false => TokenType::Number(num * f64::powi(10.0, e_after as i32))
            }
        }

        let num = before_comma as f64 + (after_comma as f64 * f64::powi(10.0, -1 * dot_place as i32));
        return TokenType::Number(num * multiplier)
    }

    TokenType::Number(before_comma as f64 * multiplier)
}

pub fn is_number(ch: char, ch_next: char) -> bool {
    (ch == '.' && (ch_next >= '0' && ch_next <= '9')) || (ch >= '0' && ch <= '9') || (( ch == '-' || ch == '+') && (ch_next >= '0' && ch_next <= '9'))
}

pub fn get_number_token(tokinizer: &mut Tokinizer) -> Option<TokenType> {
    let ch      = tokinizer.get_char();
    let ch_next = tokinizer.get_next_char();

    if !is_number(ch, ch_next) {
        return None;
    }

    let number_system = detect_number_system(tokinizer);

    let token_type = match number_system {
        BramaNumberSystem::Binary      => parse_binary(tokinizer),
        BramaNumberSystem::Octal       => parse_octal(tokinizer),
        BramaNumberSystem::Decimal     => parse_decimal(tokinizer),
        BramaNumberSystem::Hexadecimal => parse_hex(tokinizer)
    };

    Some(token_type)
}

pub fn number_parser(mut tokinizer: &mut Tokinizer) -> TokenParserResult {
    let start_column = tokinizer.column;
    let token_type = match get_number_token(&mut tokinizer) {
        Some(token_type) => token_type,
        None => return Ok(false)
    };

    tokinizer.add_token(start_column, token_type);

    if tokinizer.get_char().is_alphabetic() && !tokinizer.get_char().is_whitespace() {
        return Err(("Number parser error", tokinizer.column));
    }
    Ok(true)
}

pub fn number_regex_parser(tokinizer: &mut Tokinizer, group_item: &Vec<Regex>) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            /* Check price value */
            let mut number = 0.0;

            if let Some(binary) = capture.name("BINARY") {
                number = i64::from_str_radix(binary.as_str(), 2).unwrap() as f64;
            }
            else if let Some(hex) = capture.name("HEX") { 
                number = i64::from_str_radix(hex.as_str(), 16).unwrap() as f64;
            }
            else if let Some(hex) = capture.name("OCTAL") { 
                number = i64::from_str_radix(hex.as_str(), 8).unwrap() as f64;
            }
            else if let Some(decimal) = capture.name("DECIMAL") { 
                number = match decimal.as_str().replace(",", ".").parse::<f64>() {
                    Ok(num) => num,
                    _ => return
                };
            }

            tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), Some(TokenType::Number(number)), capture.get(0).unwrap().as_str().to_string());
        }
    }
}


#[cfg(test)]
#[test]
fn number_test_1() {
    use crate::tokinizer::test::setup;
    let tokinizer_mut = setup("1024 -1024 1024.1 -1024.1".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    let tokens = &tokinizer_mut.borrow().token_locations;

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 4);
    assert_eq!(tokens[0].token_type, Some(TokenType::Number(1024.0)));
    
    assert_eq!(tokens[1].start, 5);
    assert_eq!(tokens[1].end, 10);
    assert_eq!(tokens[1].token_type, Some(TokenType::Number(-1024.0)));
    
    assert_eq!(tokens[2].start, 11);
    assert_eq!(tokens[2].end, 17);
    assert_eq!(tokens[2].token_type, Some(TokenType::Number(1024.1)));
    
    assert_eq!(tokens[3].start, 18);
    assert_eq!(tokens[3].end, 25);
    assert_eq!(tokens[3].token_type, Some(TokenType::Number(-1024.1)));
}

#[cfg(test)]
#[test]
fn number_test_2() {
    use crate::tokinizer::test::setup;
    let tokinizer_mut = setup("0x100 0X100 0x1 0X1 0x0 0X0".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    let tokens = &tokinizer_mut.borrow().token_locations;

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 5);
    assert_eq!(tokens[0].token_type, Some(TokenType::Number(256.0)));
    
    assert_eq!(tokens[1].start, 6);
    assert_eq!(tokens[1].end, 11);
    assert_eq!(tokens[1].token_type, Some(TokenType::Number(256.0)));
    
    assert_eq!(tokens[2].start, 12);
    assert_eq!(tokens[2].end, 15);
    assert_eq!(tokens[2].token_type, Some(TokenType::Number(1.0)));
    
    assert_eq!(tokens[3].start, 16);
    assert_eq!(tokens[3].end, 19);
    assert_eq!(tokens[3].token_type, Some(TokenType::Number(1.0)));
    
    assert_eq!(tokens[4].start, 20);
    assert_eq!(tokens[4].end, 23);
    assert_eq!(tokens[4].token_type, Some(TokenType::Number(0.0)));
    
    assert_eq!(tokens[5].start, 24);
    assert_eq!(tokens[5].end, 27);
    assert_eq!(tokens[5].token_type, Some(TokenType::Number(0.0)));
}

#[cfg(test)]
#[test]
fn number_test_3() {
    use crate::tokinizer::test::setup;
    let tokinizer_mut = setup("0b100 0B100 0b1 0B1 0b0 0B0".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    let tokens = &tokinizer_mut.borrow().token_locations;

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 5);
    assert_eq!(tokens[0].token_type, Some(TokenType::Number(4.0)));
    
    assert_eq!(tokens[1].start, 6);
    assert_eq!(tokens[1].end, 11);
    assert_eq!(tokens[1].token_type, Some(TokenType::Number(4.0)));
    
    assert_eq!(tokens[2].start, 12);
    assert_eq!(tokens[2].end, 15);
    assert_eq!(tokens[2].token_type, Some(TokenType::Number(1.0)));
    
    assert_eq!(tokens[3].start, 16);
    assert_eq!(tokens[3].end, 19);
    assert_eq!(tokens[3].token_type, Some(TokenType::Number(1.0)));
    
    assert_eq!(tokens[4].start, 20);
    assert_eq!(tokens[4].end, 23);
    assert_eq!(tokens[4].token_type, Some(TokenType::Number(0.0)));
    
    assert_eq!(tokens[5].start, 24);
    assert_eq!(tokens[5].end, 27);
    assert_eq!(tokens[5].token_type, Some(TokenType::Number(0.0)));
}


#[cfg(test)]
#[test]
fn number_test_4() {
    use crate::tokinizer::test::setup;
    let tokinizer_mut = setup("0o100 0O100 0o1 0O1 0o0 0O0".to_string());

    tokinizer_mut.borrow_mut().tokinize_with_regex();
    let tokens = &tokinizer_mut.borrow().token_locations;

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 5);
    assert_eq!(tokens[0].token_type, Some(TokenType::Number(64.0)));
    
    assert_eq!(tokens[1].start, 6);
    assert_eq!(tokens[1].end, 11);
    assert_eq!(tokens[1].token_type, Some(TokenType::Number(64.0)));
    
    assert_eq!(tokens[2].start, 12);
    assert_eq!(tokens[2].end, 15);
    assert_eq!(tokens[2].token_type, Some(TokenType::Number(1.0)));
    
    assert_eq!(tokens[3].start, 16);
    assert_eq!(tokens[3].end, 19);
    assert_eq!(tokens[3].token_type, Some(TokenType::Number(1.0)));
    
    assert_eq!(tokens[4].start, 20);
    assert_eq!(tokens[4].end, 23);
    assert_eq!(tokens[4].token_type, Some(TokenType::Number(0.0)));
    
    assert_eq!(tokens[5].start, 24);
    assert_eq!(tokens[5].end, 27);
    assert_eq!(tokens[5].token_type, Some(TokenType::Number(0.0)));
}
