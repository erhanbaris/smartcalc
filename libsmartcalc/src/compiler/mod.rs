use std::rc::Rc;

use crate::{types::*};
use crate::executer::Storage;

pub struct Interpreter;

impl Interpreter {
    pub fn execute(ast: Rc<BramaAstType>, storage: Rc<Storage>) -> Result<Rc<BramaAstType>, String> {
        Interpreter::execute_ast(storage.clone(), ast.clone())
    }

    fn execute_ast(storage: Rc<Storage>, ast: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        match &*ast {
            BramaAstType::Binary { left, operator, right } => Interpreter::executer_binary(storage.clone(), left.clone(), *operator, right.clone()),
            BramaAstType::Assignment { index, expression } => Interpreter::executer_assignment(storage.clone(), *index, expression.clone()),
            BramaAstType::Variable(variable)               => Interpreter::executer_variable(variable.clone()),
            BramaAstType::Percent(_)                       => Ok(ast),
            BramaAstType::Number(_)                        => Ok(ast),
            BramaAstType::Time(_)                          => Ok(ast),
            BramaAstType::PrefixUnary(ch, ast)             => Interpreter::executer_unary(storage.clone(), *ch, ast.clone()),
            BramaAstType::None                             => Ok(Rc::new(BramaAstType::None)),
            _ => {
                println!("Operation not implemented {:?}", ast);
                Ok(Rc::new(BramaAstType::None))
            }
        }
    }

    fn executer_variable(variable: Rc<VariableInfo>) -> Result<Rc<BramaAstType>, String> {
        Ok(variable.data.clone())
    }

    fn executer_assignment(storage: Rc<Storage>, index: usize, expression: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        let computed  = Interpreter::execute_ast(storage.clone(), expression)?;
        Rc::get_mut(&mut storage.variables.borrow_mut()[index]).unwrap().data = computed.clone();
        Ok(computed.clone())
    }

    fn executer_binary(storage: Rc<Storage>, left: Rc<BramaAstType>, operator: char, right: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        let computed_left  = Interpreter::execute_ast(storage.clone(), left)?;
        let computed_right = Interpreter::execute_ast(storage.clone(), right)?;

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

        Ok(Rc::new(BramaAstType::Number(result)))
    }

    fn executer_unary(storage: Rc<Storage>, operator: char, ast: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        let computed = Interpreter::execute_ast(storage.clone(), ast)?;

        let result = match operator {
            '+' => return Ok(computed.clone()),
            '-' => match &*computed {
                BramaAstType::Money(money, currency) => BramaAstType::Money(*money * -1.0, currency.to_string()),
                BramaAstType::Percent(percent)       => BramaAstType::Percent(*percent * -1.0),
                BramaAstType::Number(number)         => BramaAstType::Number(*number * -1.0),
                _ => return Err("Syntax error".to_string())
            },
            _ => return Err("Syntax error".to_string())
        };

        Ok(Rc::new(result))
    }
}
