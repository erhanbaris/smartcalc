use crate::types::*;
use crate::tokinizer::Tokinizer;

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

fn parse_hex(tokinizer: &mut Tokinizer) -> Token {
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

    Token::Number(number as f64)
}

fn parse_octal(tokinizer: &mut Tokinizer) -> Token {
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

    Token::Number(number as f64)
}

fn parse_binary(tokinizer: &mut Tokinizer) -> Token {
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

    Token::Number(number as f64)
}

fn parse_decimal(tokinizer: &mut Tokinizer) -> Token {
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
                true  => Token::Number(num / f64::powi(10.0, e_after as i32)),
                false => Token::Number(num * f64::powi(10.0, e_after as i32))
            }
        }

        let num = before_comma as f64 + (after_comma as f64 * f64::powi(10.0, -1 * dot_place as i32));
        return Token::Number(num * multiplier)
    }

    Token::Number(before_comma as f64 * multiplier)
}

pub fn is_number(ch: char, ch_next: char) -> bool {
    (ch == '.' && (ch_next >= '0' && ch_next <= '9')) || (ch >= '0' && ch <= '9') || (( ch == '-' || ch == '+') && (ch_next >= '0' && ch_next <= '9'))
}

pub fn get_number_token(tokinizer: &mut Tokinizer) -> Option<Token> {
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