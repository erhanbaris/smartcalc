use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::format;
use chrono::{Duration, NaiveTime, Timelike};

use crate::{constants::ConstantType, types::*};
use crate::executer::Storage;
use crate::tools::convert_currency;

use log;

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
            BramaAstType::Money(_, _)                      => Ok(ast),
            BramaAstType::Duration(_, _, _)                => Ok(ast),
            BramaAstType::Month(_)                         => Ok(ast),
            BramaAstType::PrefixUnary(ch, ast)             => Interpreter::executer_unary(storage.clone(), *ch, ast.clone()),
            BramaAstType::None                             => Ok(Rc::new(BramaAstType::None)),
            _ => {
                log::debug!("Operation not implemented {:?}", ast);
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


    fn detect_target_currency(left: Rc<BramaAstType>, right: Rc<BramaAstType>) -> Option<String> {
        let left_currency = match &*left {
            BramaAstType::Money(_, currency) => Some(currency),
            BramaAstType::Variable(variable) => match &*variable.data {
                BramaAstType::Money(_, currency) => Some(currency),
                _ => None
            },
            _ => None
        };

        let right_currency = match &*right {
            BramaAstType::Money(_, currency) => Some(currency),
            BramaAstType::Variable(variable) => match &*variable.data {
                BramaAstType::Money(_, currency) => Some(currency),
                _ => None
            },
            _ => None
        };

        match (left_currency, right_currency) {
            (Some(_), Some(r)) => Some(r.to_string()),
            (None, Some(r)) => Some(r.to_string()),
            (Some(l), None) => Some(l.to_string()),
            _ => None
        }
    }

    fn get_first_percent(left: Rc<BramaAstType>, right: Rc<BramaAstType>) -> Option<f64> {
        match &*left {
            BramaAstType::Percent(percent) => return Some(*percent),
            BramaAstType::Variable(variable) => match &*variable.data {
                BramaAstType::Percent(percent) => return Some(*percent),
                _ => ()
            },
            _ => ()
        };

        match &*right {
            BramaAstType::Percent(percent) => Some(*percent),
            BramaAstType::Variable(variable) => match &*variable.data {
                BramaAstType::Percent(percent) => return Some(*percent),
                _ => None
            },
            _ => None
        }
    }

    fn get_first_money(left: Rc<BramaAstType>, right: Rc<BramaAstType>) -> Option<f64> {
        match &*left {
            BramaAstType::Money(money, _) => return Some(*money),
            BramaAstType::Variable(variable) => match &*variable.data {
                BramaAstType::Money(money, _) => return Some(*money),
                _ => ()
            },
            _ => ()
        };

        match &*right {
            BramaAstType::Money(money, _) => return Some(*money),
            BramaAstType::Variable(variable) => match &*variable.data {
                BramaAstType::Money(money, _) => return Some(*money),
                _ => None
            },
            _ => None
        }
    }

    fn get_first_number(left: Rc<BramaAstType>, right: Rc<BramaAstType>) -> Option<f64> {
        match &*left {
            BramaAstType::Number(number) => return Some(*number),
            BramaAstType::Variable(variable) => match &*variable.data {
                BramaAstType::Number(number) => return Some(*number),
                _ => ()
            },
            _ => ()
        };

        match &*right {
            BramaAstType::Number(number) => return Some(*number),
            BramaAstType::Variable(variable) => match &*variable.data {
                BramaAstType::Number(number) => return Some(*number),
                _ => None
            },
            _ => None
        }
    }

    fn get_moneys(left: Rc<BramaAstType>, right: Rc<BramaAstType>) -> Option<((f64, String), (f64, String))> {
        let left_money = match &*left {
            BramaAstType::Money(price, currency) => (*price, currency.to_string()),
            BramaAstType::Variable(variable) => match &*variable.data {
                BramaAstType::Money(price, currency) => (*price, currency.to_string()),
                _ => return None
            },
            _ => return None
        };

        let right_number = match &*right {
            BramaAstType::Money(price, currency) => (*price, currency.to_string()),
            BramaAstType::Variable(variable) => match &*variable.data {
                BramaAstType::Money(price, currency) => (*price, currency.to_string()),
                _ => return None
            },
            _ => return None
        };

        Some((left_money, right_number))
    }

    fn get_numbers(left: Rc<BramaAstType>, right: Rc<BramaAstType>) -> Option<(f64, f64)> {
        let left_number = match &*left {
            BramaAstType::Number(number) => *number,
            BramaAstType::Money(price, _) => *price,
            BramaAstType::Percent(percent) => *percent,
            BramaAstType::Duration(_, duration, _) => *duration as f64,
            BramaAstType::Variable(variable) => match &*variable.data {
                BramaAstType::Number(number) => *number,
                BramaAstType::Money(price, _) => *price,
                BramaAstType::Percent(percent) => *percent,
                BramaAstType::Duration(_, duration, _) => *duration as f64,
                _ => return None
            },
            _ => return None
        };

        let right_number = match &*right {
            BramaAstType::Number(number) => *number,
            BramaAstType::Money(price, _) => *price,
            BramaAstType::Percent(percent) => *percent,
            BramaAstType::Duration(_, duration, _) => *duration as f64,
            BramaAstType::Variable(variable) => match &*variable.data {
                BramaAstType::Number(number) => *number,
                BramaAstType::Money(price, _) => *price,
                BramaAstType::Percent(percent) => *percent,
                BramaAstType::Duration(_, duration, _) => *duration as f64,
                _ => return None
            },
            _ => return None
        };

        Some((left_number, right_number))
    }

    fn duration_to_time(duration: i32, duration_type: ConstantType) -> NaiveTime {
        match duration_type {
            ConstantType::Second => NaiveTime::from_hms(0, 0, duration as u32),
            ConstantType::Minute => NaiveTime::from_hms(0, duration as u32, 0),
            ConstantType::Hour   => NaiveTime::from_hms(duration.abs() as u32, 0, 0),
            _ => NaiveTime::from_hms(0, 0, 0), 
        }
    }

    fn get_times(left: Rc<BramaAstType>, right: Rc<BramaAstType>) -> Option<(NaiveTime, NaiveTime, bool)> {
        let left_time = match &*left {
            BramaAstType::Time(time) => *time,
            BramaAstType::Duration(_, duration, duration_type) => Interpreter::duration_to_time(*duration as i32, duration_type.clone()),
            BramaAstType::Variable(variable) => match &*variable.data {
                BramaAstType::Time(time) => *time,
                BramaAstType::Duration(_, duration, duration_type) => Interpreter::duration_to_time(*duration as i32, duration_type.clone()),
                _ => return None
            },
            _ => return None
        };

        let (right_time, is_negative) = match &*right {
            BramaAstType::Time(time) => (*time, false),
            BramaAstType::Duration(_, duration, duration_type) => (Interpreter::duration_to_time(*duration as i32, duration_type.clone()), duration.is_negative()),
            BramaAstType::Variable(variable) => match &*variable.data {
                BramaAstType::Time(time) => (*time, false),
                BramaAstType::Duration(_, duration, duration_type) => (Interpreter::duration_to_time(*duration as i32, duration_type.clone()), duration.is_negative()),
                _ => return None
            },
            _ => return None
        };

        Some((left_time, right_time, is_negative))
    }

    fn calculate_number(operator: char, left: Rc<BramaAstType>, right: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        /* Percent operation */
        match Interpreter::get_first_percent(left.clone(), right.clone()) {
            Some(percent) => {
                let number = match Interpreter::get_first_number(left.clone(), right.clone()) {
                    Some(num) => num,
                    None => return Err("Number information not valid".to_string())
                };

                return match operator {
                    '+' => Ok(Rc::new(BramaAstType::Number(number + ((number * percent) / 100.0)))),
                    '-' => Ok(Rc::new(BramaAstType::Number(number - ((number * percent) / 100.0)))),
                    '*' => Ok(Rc::new(BramaAstType::Number(number * ((number * percent) / 100.0)))),
                    '/' => Ok(Rc::new(BramaAstType::Number(Interpreter::do_divition(number, Interpreter::do_divition(number * percent, 100.0))))),
                    _ => Err(format!("Unknown operator. ({})", operator).to_string())
                };
            },
            _ => ()
        };
        
        match Interpreter::get_numbers(left.clone(), right.clone()) {
            Some((left_number, right_number)) => {
                match operator {
                    '+' => Ok(Rc::new(BramaAstType::Number(left_number + right_number))),
                    '-' => Ok(Rc::new(BramaAstType::Number(left_number - right_number))),
                    '/' => Ok(Rc::new(BramaAstType::Number(Interpreter::do_divition(left_number, right_number)))),
                    '*' => Ok(Rc::new(BramaAstType::Number(left_number * right_number))),
                    _ => Err(format!("Unknown operator. ({})", operator).to_string())
                }
            },
            None => Err("Unknown calculation".to_string())
        }
    }

    fn calculate_time(operator: char, left: Rc<BramaAstType>, right: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {

        /* Time calculation operation */
        match Interpreter::get_times(left.clone(), right.clone()) {
            Some((left_time, right_time, is_negative)) => {
                let calculated_right = Duration::seconds(right_time.num_seconds_from_midnight() as i64);

                if is_negative {
                    return Ok(Rc::new(BramaAstType::Time(left_time - calculated_right)));
                }
                
                return match operator {
                    '+' => Ok(Rc::new(BramaAstType::Time(left_time + calculated_right))),
                    '-' => Ok(Rc::new(BramaAstType::Time(left_time - calculated_right))),
                    _ => return Err(format!("Unknown operator. ({})", operator).to_string())
                };
            },
            None => Err(format!("Unknown operator. ({})", operator).to_string())
        }
    }

    fn calculate_money(operator: char, left: Rc<BramaAstType>, right: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        let to_currency = match Interpreter::detect_target_currency(left.clone(), right.clone()) {
            Some(currency) => currency,
            None => return Err("Currency information not valid".to_string())
        };
        
        /* Percent operation */
        match Interpreter::get_first_percent(left.clone(), right.clone()) {
            Some(percent) => {
                let price = match Interpreter::get_first_money(left.clone(), right.clone()) {
                    Some(currency) => currency,
                    None => return Err("Price information not valid".to_string())
                };

                return match operator {
                    '+' => Ok(Rc::new(BramaAstType::Money(price + ((price * percent) / 100.0), to_currency.to_string()))),
                    '-' => Ok(Rc::new(BramaAstType::Money(price - ((price * percent) / 100.0), to_currency.to_string()))),
                    '*' => Ok(Rc::new(BramaAstType::Money(price * ((price * percent) / 100.0), to_currency.to_string()))),
                    '/' => Ok(Rc::new(BramaAstType::Money(Interpreter::do_divition(price, Interpreter::do_divition(price * percent, 100.0)), to_currency.to_string()))),
                    _ => Err(format!("Unknown operator. ({})", operator).to_string())
                };
            },
            _ => ()
        };
        
        /* Money calculation operation */
        match Interpreter::get_moneys(left.clone(), right.clone()) {
            Some(((left_price, left_currency), (right_price, right_currency))) => {
                let left_price = convert_currency(left_price, &left_currency, &right_currency);
                return match operator {
                    '+' => Ok(Rc::new(BramaAstType::Money(left_price + right_price, right_currency.to_string()))),
                    '-' => Ok(Rc::new(BramaAstType::Money(left_price - right_price, right_currency.to_string()))),
                    '/' => Ok(Rc::new(BramaAstType::Money(Interpreter::do_divition(left_price, right_price), right_currency.to_string()))),
                    '*' => Ok(Rc::new(BramaAstType::Money(left_price * right_price, right_currency.to_string()))),
                    _ => Err(format!("Unknown operator. ({})", operator).to_string())
                };
            },
            None => ()
        }
        
        /* Normal operation */
        match Interpreter::get_numbers(left.clone(), right.clone()) {
            Some((left_number, right_number)) => {
                match operator {
                    '+' => Ok(Rc::new(BramaAstType::Money(left_number + right_number, to_currency.to_string()))),
                    '-' => Ok(Rc::new(BramaAstType::Money(left_number - right_number, to_currency.to_string()))),
                    '/' => Ok(Rc::new(BramaAstType::Money(Interpreter::do_divition(left_number, right_number), to_currency.to_string()))),
                    '*' => Ok(Rc::new(BramaAstType::Money(left_number * right_number, to_currency.to_string()))),
                    _ => Err(format!("Unknown operator. ({})", operator).to_string())
                }
            },
            None => Err("Unknown calculation".to_string())
        }
    }

    fn do_divition(left: f64, right: f64) -> f64 {
        let mut calculation = left / right;
        if calculation.is_infinite() || calculation.is_nan() {
            calculation = 0.0;
        }
        calculation
    }

    fn executer_binary(storage: Rc<Storage>, left: Rc<BramaAstType>, operator: char, right: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        let computed_left  = Interpreter::execute_ast(storage.clone(), left)?;
        let computed_right = Interpreter::execute_ast(storage.clone(), right)?;

        match (&*computed_left, &*computed_right) {
            (BramaAstType::Money(_, _), _)       | (_, BramaAstType::Money(_, _))       => Interpreter::calculate_money(operator, computed_left.clone(), computed_right.clone()),
            (BramaAstType::Time(_), _)           | (_, BramaAstType::Time(_))           => Interpreter::calculate_time(operator, computed_left.clone(), computed_right.clone()),
            (BramaAstType::Duration(_, _, _), _) | (_, BramaAstType::Duration(_, _, _)) => Interpreter::calculate_time(operator, computed_left.clone(), computed_right.clone()),
            (BramaAstType::Number(_), _)         | (_, BramaAstType::Number(_))         => Interpreter::calculate_number(operator, computed_left.clone(), computed_right.clone()),
            _ => Err("Uknown calculation result".to_string())
        }
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
