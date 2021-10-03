use core::any::Any;
use core::any::TypeId;
use core::cell::RefCell;
use core::ops::Deref;

use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::format;
use alloc::sync::Arc;
use chrono::NaiveTime;

use crate::app::Session;
use crate::config::SmartCalcConfig;
use crate::types::*;

pub mod number;
pub mod percent;
pub mod money;
pub mod time;
pub mod duration;
pub mod date;
pub mod memory;
pub mod date_time;

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

    fn executer_binary(config: &SmartCalcConfig, session: &RefCell<Session>, left: Rc<BramaAstType>, operator: char, right: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        let computed_left  = Interpreter::execute_ast(config, session, left)?;
        let computed_right = Interpreter::execute_ast(config, session, right)?;

        match (computed_left.deref(), computed_right.deref()) {
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
