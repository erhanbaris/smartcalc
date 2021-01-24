use std::vec::Vec;
use std::rc::Rc;
use std::cell::RefCell;

use std::ptr;

use crate::{types::*};

pub struct Executer;
impl Executer {
    pub fn execute(asts: &Vec<BramaAstType>) -> Vec<Result<f64, String>> {
        let mut results = Vec::new();

        for ast in asts {
            results.push(Executer::execute_ast(ast));
        }
        results
    }

    fn execute_ast(ast: &BramaAstType) -> Result<f64, String> {
        match ast {
            BramaAstType::Binary { left, operator, right } => Executer::executer_binary(left, *operator, right),
            _ => Err("Operation not validated".to_string())
        }
    }

    fn executer_binary(left: &BramaAstType, operator: char, right: &BramaAstType) -> Result<f64, String> {
        let result = match operator {
            '+' => {
                match (left, right) {
                    (BramaAstType::Percent(percent), BramaAstType::Number(number)) => number + ((number * percent) / 100.0),
                    (BramaAstType::Number(number), BramaAstType::Percent(percent)) => number + ((number * percent) / 100.0),
                    (BramaAstType::Number(l_num), BramaAstType::Number(r_num)) => l_num + r_num,
                    _ => -1.0
                }
            },
            '-' => {
                match (left, right) {
                    (BramaAstType::Percent(percent), BramaAstType::Number(number)) => ((number * percent) / 100.0) - number,
                    (BramaAstType::Number(number), BramaAstType::Percent(percent)) => number - ((number * percent) / 100.0),
                    (BramaAstType::Number(l_num), BramaAstType::Number(r_num)) => l_num - r_num,
                    _ => -1.0
                }
            },
            '*' => {
                match (left, right) {
                    (BramaAstType::Percent(percent), BramaAstType::Number(number)) => ((number * percent) / 100.0) * number,
                    (BramaAstType::Number(number), BramaAstType::Percent(percent)) => number * ((number * percent) / 100.0),
                    (BramaAstType::Number(l_num), BramaAstType::Number(r_num)) => l_num * r_num,
                    _ => -1.0
                }
            },
            '/' => {
                match (left, right) {
                    (BramaAstType::Percent(percent), BramaAstType::Number(number)) => ((number * percent) / 100.0) / number,
                    (BramaAstType::Number(number), BramaAstType::Percent(percent)) => number / ((number * percent) / 100.0),
                    (BramaAstType::Number(l_num), BramaAstType::Number(r_num)) => l_num / r_num,
                    _ => -1.0
                }
            },
            _ => 0.0
        };

        println!("{}", result);

        return Ok(result);
    }
}
