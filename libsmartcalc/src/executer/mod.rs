use std::rc::Rc;
use std::cell::RefCell;

use crate::worker::WorkerExecuter;
use crate::tokinizer::Tokinizer;
use crate::syntax::SyntaxParser;
use crate::types::{Token, TokenType, BramaAstType, VariableInfo};
use crate::compiler::Interpreter;

use serde_json::{Value, from_str};
use regex::{Regex};

pub struct Storage {
    pub asts: RefCell<Vec<Rc<BramaAstType>>>,
    pub variables: RefCell<Vec<Rc<VariableInfo>>>
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            asts: RefCell::new(Vec::new()),
            variables: RefCell::new(Vec::new())
        }
    }
}

pub fn token_cleaner(tokens: &mut Vec<Token>) {
    let mut index = 0;
    for (token_index, token) in tokens.iter().enumerate() {
        match token.token {
            TokenType::Operator('=') => {
                index = token_index as usize + 1;
                break;
            },
            _ => ()
        };
    }

    while index < tokens.len() {
        if let TokenType::Text(_) = tokens[index].token {
            tokens.remove(index);
        }
        else {
            index += 1;
        }
    }
}

pub fn missing_token_adder(tokens: &mut Vec<Token>) {
    let mut index = 0;
    for (token_index, token) in tokens.iter().enumerate() {
        match token.token {
            TokenType::Operator('=') => {
                index = token_index as usize + 1;
                break;
            },
            _ => ()
        };
    }

    if tokens.len() == 0 {
        return;
    }

    if index + 1 >= tokens.len() {
        return;
    }

    if let TokenType::Operator(_) = tokens[index].token {
        tokens.insert(index, Token {
            start: 0,
            end: 1,
            token: TokenType::Number(0.0),
            is_temp: true
        });
    }

    index += 1;
    while index < tokens.len() {
        match tokens[index].token {
            TokenType::Operator(_) => index += 2,
            _ => {
                tokens.insert(index, Token {
                    start: 0,
                    end: 1,
                    token: TokenType::Operator('+'),
                    is_temp: true
                });
                index += 2;
            }
        };
    }

    if let TokenType::Operator(_) = tokens[tokens.len()-1].token {
        tokens.insert(tokens.len()-1, Token {
            start: 0,
            end: 1,
            token: TokenType::Number(0.0),
            is_temp: true
        });
    }
}

fn time_parse(data: &mut String, group_item: &Value) -> String {
    let mut data_str = data.to_string();

    for time_pattern in group_item.as_array().unwrap() {
        let re = Regex::new(time_pattern.as_str().unwrap()).unwrap();
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
            data_str = data_str.replace(capture.get(0).unwrap().as_str(), &format!("[TIME:{}]", time_number)[..]);
        }
    }

    data_str
}

const json_data: &str = r#"{
  "parse":  {
    "time": [
      "(?P<hour>1[0-2]|0?[1-9]):(?P<minute>[0-5][0-9]):(?P<second>[0-5][0-9]) ?(?P<meridiem>[AaPp][Mm])",
      "(?P<hour>1[0-2]|0?[1-9]):(?P<minute>[0-5][0-9]) ?(?P<meridiem>[AaPp][Mm])",
      "(?P<hour>[0-1]?[0-9]|2[0-3]):(?P<minute>[0-5][0-9]):(?P<second>[0-5][0-9])",
      "(?P<hour>[0-1]?[0-9]|2[0-3]):(?P<minute>[0-5][0-9])"
    ],
    "number": []
  },

  "rules": {
    "percent_calculator": ["{PERCENT:percent} {NUMBER:number}", "{NUMBER:number} {PERCENT:percent}"],
    "hour_add": ["{TIME:time} add {NUMBER:hours} hour"],
    "date_add": ["{DATE:date}\"e {NUMBER:day} gün ekle"],
    "time_for_location": ["time in {TEXT:location}", "time at {TEXT:location}", "time for {TEXT:location}"]
  },

  "alias": {
    "at": "in",
    "for": "in",
    "hours": "hour",
    "günler": "gün",

    ",": "",
    "_": "",
    ";": "",
    "!": "",
    "?": "",
    "'": "",
    "&": "",
    "^": "",

    "times": "[OPERATOR:*]",
    "multiply": "[OPERATOR:*]",
    "x": "[OPERATOR:*]",

    "add": "[OPERATOR:+]",
    "sum": "[OPERATOR:+]",
    "append": "[OPERATOR:+]",

    "exclude": "[OPERATOR:-]",
    "minus": "[OPERATOR:-]",

    "percent": "[OPERATOR:%]",
    "percentage": "[OPERATOR:%]"
  }
}"#;

pub fn prepare_code(data: &String) -> String {
    let mut data_str = data.to_string();
    let json_value: serde_json::Result<Value> = from_str(&json_data);
    match json_value {
        Ok(json) => {
            if let Some(group) = json.get("alias").unwrap().as_object() {
                for (key, value) in group.iter() {
                    let re = Regex::new(&format!(r"\b{}\b", key.as_str())[..]).unwrap();
                    data_str = re.replace_all(&data_str, value.as_str().unwrap()).to_string();
                }
            }

            if let Some(group) = json.get("parse").unwrap().as_object() {
                for (group, group_item) in group.iter() {
                    data_str = match group.as_str() {
                        "time" => time_parse(&mut data_str, group_item),
                        _ => data_str
                    };
                }
            }
        },
        _ => ()
    }
    data_str
}

pub fn execute(data: &String, language: &String) -> Vec<Result<(Rc<Vec<Token>>, Rc<BramaAstType>), String>> {
    let mut results     = Vec::new();
    let storage         = Rc::new(Storage::new());
    let worker_executer = WorkerExecuter::new();

    for text in data.lines() {
        let prepared_text = prepare_code(&text.to_string());
        println!("> {}", prepared_text);

        if prepared_text.len() == 0 {
            storage.asts.borrow_mut().push(Rc::new(BramaAstType::None));
            results.push(Ok((Rc::new(Vec::new()), Rc::new(BramaAstType::None))));
            continue;
        }

        let result = Tokinizer::tokinize(&prepared_text.to_string());
        match result {
            Ok(mut tokens) => {
                println!("tokens {:?}", tokens);
                Token::update_for_variable(&mut tokens, storage.clone());
                worker_executer.process(&language, &mut tokens, storage.clone());
                token_cleaner(&mut tokens);
                missing_token_adder(&mut tokens);
                println!("tokens {:?}", tokens);

                let tokens_rc = Rc::new(tokens);
                let syntax = SyntaxParser::new(tokens_rc.clone(), storage.clone());
                match syntax.parse() {
                    Ok(ast) => {
                        let ast_rc = Rc::new(ast);
                        storage.asts.borrow_mut().push(ast_rc.clone());

                        match Interpreter::execute(ast_rc.clone(), storage.clone()) {
                            Ok(ast) => results.push(Ok((tokens_rc.clone(), ast.clone()))),
                            Err(error) => results.push(Err(error))
                        };

                        println!("Ast {:?}", ast_rc.clone());
                    },
                    Err((error, _, _)) => println!("error, {}", error)
                }
            },
            Err((error, _, _)) => {
                results.push(Err(error.to_string()));
                storage.asts.borrow_mut().push(Rc::new(BramaAstType::None));
            }
        };
    }

    results
}