use std::vec::Vec;
use std::rc::Rc;

use crate::{types::*};

pub struct Executer;
impl Executer {
    pub fn execute(asts: &Vec<Rc<BramaAstType>>) -> Vec<Result<Rc<BramaAstType>, String>> {
        let mut results = Vec::new();

        for ast in asts {
            results.push(Executer::execute_ast(ast.clone()));
        }
        results
    }

    fn execute_ast(ast: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        match &*ast.clone() {
            BramaAstType::Binary { left, operator, right } => Executer::executer_binary(left.clone(), *operator, right.clone()),
            BramaAstType::Percent(_) => Ok(ast.clone()),
            BramaAstType::Number(_) => Ok(ast.clone()),
            _ => Err("Operation not validated".to_string())
        }
    }

    fn executer_binary(left: Rc<BramaAstType>, operator: char, right: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        let computed_left  = Executer::execute_ast(left.clone())?;
        let computed_right = Executer::execute_ast(right.clone())?;

        let result = match operator {
            '+' => {
                match (&*computed_left, &*computed_right) {
                    (BramaAstType::Percent(percent), BramaAstType::Number(number)) => number + ((number * percent) / 100.0),
                    (BramaAstType::Number(number), BramaAstType::Percent(percent)) => number + ((number * percent) / 100.0),
                    (BramaAstType::Number(l_num), BramaAstType::Number(r_num)) => l_num + r_num,
                    _ => return Err("Syntax error".to_string())
                }
            },
            '-' => {
                match (&*computed_left, &*computed_right) {
                    (BramaAstType::Percent(percent), BramaAstType::Number(number)) => ((number * percent) / 100.0) - number,
                    (BramaAstType::Number(number), BramaAstType::Percent(percent)) => number - ((number * percent) / 100.0),
                    (BramaAstType::Number(l_num), BramaAstType::Number(r_num)) => l_num - r_num,
                    _ => return Err("Syntax error".to_string())
                }
            },
            '*' => {
                match (&*computed_left, &*computed_right) {
                    (BramaAstType::Percent(percent), BramaAstType::Number(number)) => ((number * percent) / 100.0) * number,
                    (BramaAstType::Number(number), BramaAstType::Percent(percent)) => number * ((number * percent) / 100.0),
                    (BramaAstType::Number(l_num), BramaAstType::Number(r_num)) => l_num * r_num,
                    _ => return Err("Syntax error".to_string())
                }
            },
            '/' => {
                match (&*computed_left, &*computed_right) {
                    (BramaAstType::Percent(percent), BramaAstType::Number(number)) => ((number * percent) / 100.0) / number,
                    (BramaAstType::Number(number), BramaAstType::Percent(percent)) => number / ((number * percent) / 100.0),
                    (BramaAstType::Number(l_num), BramaAstType::Number(r_num)) => l_num / r_num,
                    _ => return Err("Syntax error".to_string())
                }
            },
            _ => return Err("Syntax error".to_string())
        };

        return Ok(Rc::new(BramaAstType::Number(result)));
    }
}
