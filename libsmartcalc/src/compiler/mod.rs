use core::any::Any;
use core::any::TypeId;
use core::cell::RefCell;
use core::ops::Deref;

use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::format;
use alloc::sync::Arc;
use chrono::{Datelike, Duration, NaiveDate, NaiveTime};

use crate::app::Session;
use crate::config::SmartCalcConfig;
use crate::{formatter::{MONTH, YEAR}, types::*};
use crate::formatter::{MINUTE, HOUR};

use self::duration::DurationItem;
use self::time::TimeItem;

pub mod number;
pub mod percent;
pub mod money;
pub mod time;
pub mod duration;

#[derive(Clone)]
#[derive(Copy)]
pub enum OperationType {
    Add,
    Div,
    Mul,
    Sub
}


#[derive(Clone)]
#[derive(Copy)]
pub enum UnaryType {
    Plus,
    Minus
}

pub trait DataItem: alloc::fmt::Debug {
    fn unary(&self, unary: UnaryType) -> Arc<dyn DataItem>;
    fn is_same<'a>(&self, other: &'a dyn Any) -> bool;
    fn as_token_type(&self) -> TokenType;
    fn as_any(&self) -> &dyn Any;
    fn get_number(&self, other: &dyn DataItem) -> f64;
    fn get_underlying_number(&self) -> f64;
    fn type_name(&self) -> &'static str;
    fn type_id(&self) -> TypeId;
    fn calculate(&self, config: &SmartCalcConfig, on_left: bool, other: &dyn DataItem, operation_type: OperationType) -> Option<Arc<dyn DataItem>>;
    fn print(&self, config: &SmartCalcConfig, session: &RefCell<Session>) -> String;
}

pub trait AsNaiveTime {
    fn as_naive_time(&self) -> NaiveTime;
}

pub struct Operation;
impl Operation {
    pub fn calculate(config: &SmartCalcConfig, left: &dyn DataItem, right: &dyn DataItem, operation_type: OperationType) -> Option<Arc<dyn DataItem>> {
        left.calculate(config, true, right, operation_type)
            .or_else(|| right.calculate(config, false, left, operation_type))
    }
}

pub struct Interpreter;

impl Interpreter {
    pub fn execute(config: &SmartCalcConfig, ast: Rc<BramaAstType>, session: &RefCell<Session>) -> Result<Rc<BramaAstType>, String> {
        Interpreter::execute_ast(config, session, ast)
    }

    fn execute_ast(config: &SmartCalcConfig, session: &RefCell<Session>, ast: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        match ast.deref() {
            BramaAstType::Binary { left, operator, right } => Interpreter::executer_binary(config, session, left.clone(), *operator, right.clone()),
            BramaAstType::Assignment { index, expression } => Interpreter::executer_assignment(config, session, *index, expression.clone()),
            BramaAstType::Variable(variable)               => Ok(Interpreter::executer_variable(variable.clone())),
            //BramaAstType::Time(_)                          => Ok(ast),
            BramaAstType::Date(_)                          => Ok(ast),
            BramaAstType::Item(_)                          => Ok(ast),
            BramaAstType::Month(_)                         => Ok(ast),
            BramaAstType::PrefixUnary(ch, ast)             => Interpreter::executer_unary(config, session, *ch, ast.clone()),
            BramaAstType::None                             => Ok(Rc::new(BramaAstType::None)),
            _ => {
                log::debug!("Operation not implemented {:?}", ast);
                Ok(Rc::new(BramaAstType::None))
            }
        }
    }

    fn executer_variable(variable: Rc<VariableInfo>) -> Rc<BramaAstType> {
        variable.data.borrow().clone()
    }

    fn executer_assignment(config: &SmartCalcConfig, session: &RefCell<Session>, index: usize, expression: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        let computed  = Interpreter::execute_ast(config, session, expression)?;
        *session.borrow_mut().variables[index].data.borrow_mut() = computed.clone();
        Ok(computed)
    }
    fn get_duration(ast: Rc<BramaAstType>) -> Option<Duration> {
        let duration = match ast.deref() {
            BramaAstType::Item(item) => match item.as_any().downcast_ref::<DurationItem>() {
                Some(number) => Some(number.get_duration()),
                _ => None
            },
            BramaAstType::Variable(variable) => match variable.data.borrow().deref().deref() {
                BramaAstType::Item(item) => match item.as_any().downcast_ref::<DurationItem>() {
                    Some(number) => Some(number.get_duration()),
                    _ => None
                },
                _ => None
            },
            _ => None
        };
        
        duration
    }

    fn get_month_from_duration(duration: Duration) -> i64 {
        duration.num_seconds().abs() / MONTH
    }

    fn get_year_from_duration(duration: Duration) -> i64 {
        duration.num_seconds().abs() / YEAR
    }

    fn get_durations(left: Rc<BramaAstType>, right: Rc<BramaAstType>) -> Option<(Duration, Duration)> {
        let left_item = match left.deref() {
            BramaAstType::Item(item) => item.clone(),
            BramaAstType::Variable(variable) => match variable.data.borrow().deref().deref() {
                BramaAstType::Item(item) => item.clone(),
                _ => return None
            },
            _ => return None
        };
        
        let left_time = match left_item.type_name() {
            "DURATION" => left_item.as_any().downcast_ref::<DurationItem>().unwrap().get_duration(),
            _ => return None
        };
        
        let right_item = match right.deref() {
            BramaAstType::Item(item) => item.clone(),
            BramaAstType::Variable(variable) => match variable.data.borrow().deref().deref() {
                BramaAstType::Item(item) => item.clone(),
                _ => return None
            },
            _ => return None
        };
        
        let right_time = match right_item.type_name() {
            "DURATION" => right_item.as_any().downcast_ref::<DurationItem>().unwrap().get_duration(),
            _ => return None
        };

        Some((left_time, right_time))
    }
    
    fn get_date(ast: Rc<BramaAstType>) -> Option<NaiveDate> {
        match ast.deref() {
            BramaAstType::Date(date) => Some(*date),
            BramaAstType::Variable(variable) => match **variable.data.borrow() {
                BramaAstType::Date(date) => Some(date),
                _ => None
            },
            _ => None
        }
    }

    fn get_times(left: Rc<BramaAstType>, right: Rc<BramaAstType>) -> Option<(NaiveTime, NaiveTime, bool)> {
        let left_item = match left.deref() {
            BramaAstType::Item(item) => item.clone(),
            BramaAstType::Variable(variable) => match variable.data.borrow().deref().deref() {
                BramaAstType::Item(item) => item.clone(),
                _ => return None
            },
            _ => return None
        };
        
        let left_time = match left_item.type_name() {
            "DURATION" => left_item.as_any().downcast_ref::<DurationItem>().unwrap().as_naive_time(),
            "TIME" => left_item.as_any().downcast_ref::<TimeItem>().unwrap().as_naive_time(),
            _ => return None
        };

        let right_item = match right.deref() {
            BramaAstType::Item(item) => item.clone(),
            BramaAstType::Variable(variable) => match variable.data.borrow().deref().deref() {
                BramaAstType::Item(item) => item.clone(),
                _ => return None
            },
            _ => return None
        };
        
        let (right_time, is_negative) = match right_item.type_name() {
            "DURATION" => {
                let duration = right_item.as_any().downcast_ref::<DurationItem>().unwrap();
                (duration.as_naive_time(), duration.get_duration().num_seconds().is_negative())
            },
            "TIME" => (right_item.as_any().downcast_ref::<TimeItem>().unwrap().as_naive_time(), false),
            _ => return None
        };

        Some((left_time, right_time, is_negative))
    }

    fn calculate_item(config: &SmartCalcConfig, operator: char, left: Rc<BramaAstType>, right: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        let left = match left.deref() {
            BramaAstType::Item(left) => left.clone(),
            _ => return Err("Unknown calculation".to_string())
        };
        
        let right = match right.deref() {
            BramaAstType::Item(right) => right.clone(),
            _ => return Err("Unknown calculation".to_string())
        };
        
        let result = match operator {
            '+' => left.calculate(config, true, right.deref(), OperationType::Add),
            '-' => left.calculate(config, true, right.deref(), OperationType::Sub),
            '*' => left.calculate(config, true, right.deref(), OperationType::Mul),
            '/' => left.calculate(config, true, right.deref(), OperationType::Div),
            _ => return Err(format!("Unknown operator. ({})", operator))
        };
        
        match result {
            Some(item) => Ok(Rc::new(BramaAstType::Item(item.clone()))),
            None => Err("Unknown calculation".to_string())
        }
    }
    
    fn calculate_date(operator: char, left: Rc<BramaAstType>, right: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        match (Interpreter::get_date(left), Interpreter::get_duration(right)) {
            (Some(date), Some(duration)) => {
                let mut date     = date;
                let mut duration = duration;

                return match operator {
                    '+' => {
                        match Interpreter::get_year_from_duration(duration) {
                            0 => (),
                            n => {
                                let years_diff = date.year() + n as i32;
                                date     = NaiveDate::from_ymd(years_diff as i32, date.month() as u32, date.day());
                                duration = Duration::seconds(duration.num_seconds() - (YEAR * n))
                            }
                        };

                        match Interpreter::get_month_from_duration(duration) {
                            0 => (),
                            n => {
                                let years_diff = (date.month() + n as u32) / 12;
                                let month = (date.month() + n as u32) % 12;
                                date     = NaiveDate::from_ymd(date.year() + years_diff as i32, month as u32, date.day());
                                duration = Duration::seconds(duration.num_seconds() - (MONTH * n))
                            }
                        };
                        Ok(Rc::new(BramaAstType::Date(date + duration)))
                    },

                    '-' => {
                        match Interpreter::get_year_from_duration(duration) {
                            0 => (),
                            n => {
                                let years_diff = date.year() - n as i32;
                                date     = NaiveDate::from_ymd(years_diff as i32, date.month() as u32, date.day());
                                duration = Duration::seconds(duration.num_seconds() - (YEAR * n))
                            }
                        };

                        match Interpreter::get_month_from_duration(duration) {
                            0 => (),
                            n => {
                                let years = date.year() - (n as i32 / 12);
                                let mut months = date.month() as i32 - (n as i32 % 12);
                                if months < 0 {
                                    months += 12;
                                }

                                date = NaiveDate::from_ymd(years as i32, months as u32, date.day());
                                duration = Duration::seconds(duration.num_seconds() - (MONTH * n))
                            }
                        };
                        Ok(Rc::new(BramaAstType::Date(date - duration)))
                    },
                    _ => Err(format!("Unknown operator. ({})", operator))
                };
                
            },
            _ => Err(format!("Unknown operator. ({})", operator))
        }
    }

    /*fn calculate_time(operator: char, left: Rc<BramaAstType>, right: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {

        /* Time calculation operation */
        match Interpreter::get_times(left, right) {
            Some((left_time, right_time, is_negative)) => {
                let calculated_right = Duration::seconds(right_time.num_seconds_from_midnight() as i64);

                if is_negative {
                    return Ok(Rc::new(BramaAstType::Time(left_time - calculated_right)));
                }
                
                return match operator {
                    '+' => Ok(Rc::new(BramaAstType::Time(left_time + calculated_right))),
                    '-' => Ok(Rc::new(BramaAstType::Time(left_time - calculated_right))),
                    _ => return Err(format!("Unknown operator. ({})", operator))
                };
            },
            None => Err(format!("Unknown operator. ({})", operator))
        }
    }

    fn calculate_duration(operator: char, left: Rc<BramaAstType>, right: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        /* Time calculation operation */
        match Interpreter::get_durations(left, right) {
            Some((left_time, right_time)) => {
                
                return match operator {
                    '+' => Ok(Rc::new(BramaAstType::Duration(left_time + right_time))),
                    '-' => Ok(Rc::new(BramaAstType::Duration(left_time - right_time))),
                    _ => return Err(format!("Unknown operator. ({})", operator))
                };
            },
            None => Err(format!("Unknown operator. ({})", operator))
        }
    }*/

    fn executer_binary(config: &SmartCalcConfig, session: &RefCell<Session>, left: Rc<BramaAstType>, operator: char, right: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        let computed_left  = Interpreter::execute_ast(config, session, left)?;
        let computed_right = Interpreter::execute_ast(config, session, right)?;

        match (computed_left.deref(), computed_right.deref()) {
            (BramaAstType::Date(_), _)           | (_, BramaAstType::Date(_))           => Interpreter::calculate_date(operator, computed_left.clone(), computed_right.clone()),
            //(BramaAstType::Time(_), _)           | (_, BramaAstType::Time(_))           => Interpreter::calculate_time(operator, computed_left.clone(), computed_right.clone()),
            //(BramaAstType::Duration(_), _)       | (_, BramaAstType::Duration(_))       => Interpreter::calculate_duration(operator, computed_left.clone(), computed_right.clone()),
            (BramaAstType::Item(_), _)           | (_, BramaAstType::Item(_))           => Interpreter::calculate_item(config, operator, computed_left.clone(), computed_right.clone()),
            _ => Err("Uknown calculation result".to_string())
        }
    }

    fn executer_unary(config: &SmartCalcConfig, session: &RefCell<Session>, operator: char, ast: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        let computed = Interpreter::execute_ast(config, session, ast)?;

        let result = match operator {
            '+' => return Ok(computed),
            '-' => match computed.deref() {
                BramaAstType::Item(item) => BramaAstType::Item(item.unary(UnaryType::Minus)),
                _ => return Err("Syntax error".to_string())
            },
            _ => return Err("Syntax error".to_string())
        };

        Ok(Rc::new(result))
    }
}
