/*
 * smartcalc v1.0.8
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::any::Any;
use core::any::TypeId;
use core::ops::Deref;

use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::format;

use crate::session::Session;
use crate::config::SmartCalcConfig;
use crate::types::*;
use crate::variable::VariableInfo;

pub mod number;
pub mod percent;
pub mod money;
pub mod time;
pub mod duration;
pub mod date;
pub mod date_time;
pub mod dynamic_type;

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
    fn unary(&self, unary: UnaryType) -> Rc<dyn DataItem>;
    fn is_same(&self, other: &dyn Any) -> bool;
    fn as_token_type(&self) -> TokenType;
    fn as_any(&self) -> &dyn Any;
    fn get_number(&self, other: &dyn DataItem) -> f64;
    fn get_underlying_number(&self) -> f64;
    fn type_name(&self) -> &'static str;
    fn type_id(&self) -> TypeId;
    fn calculate(&self, config: &SmartCalcConfig, on_left: bool, other: &dyn DataItem, operation_type: OperationType) -> Option<Rc<dyn DataItem>>;
    fn print(&self, config: &SmartCalcConfig, session: &Session) -> String;
}

pub struct Interpreter;

impl Interpreter {
    pub fn execute(config: &SmartCalcConfig, ast: Rc<SmartCalcAstType>, session: &Session) -> Result<Rc<SmartCalcAstType>, String> {
        Interpreter::execute_ast(config, session, ast)
    }

    fn execute_ast(config: &SmartCalcConfig, session: &Session, ast: Rc<SmartCalcAstType>) -> Result<Rc<SmartCalcAstType>, String> {
        match ast.deref() {
            SmartCalcAstType::Binary { left, operator, right } => Interpreter::executer_binary(config, session, left.clone(), *operator, right.clone()),
            SmartCalcAstType::Assignment { variable, expression } => Interpreter::executer_assignment(config, session, variable.clone(), expression.clone()),
            SmartCalcAstType::Variable(variable)               => Ok(Interpreter::executer_variable(variable.clone())),
            SmartCalcAstType::Item(_)                          => Ok(ast),
            SmartCalcAstType::Month(_)                         => Ok(ast),
            SmartCalcAstType::PrefixUnary(ch, ast)             => Interpreter::executer_unary(config, session, *ch, ast.clone()),
            SmartCalcAstType::None                             => Ok(Rc::new(SmartCalcAstType::None)),
            _ => {
                log::debug!("Operation not implemented {:?}", ast);
                Ok(Rc::new(SmartCalcAstType::None))
            }
        }
    }

    fn executer_variable(variable: Rc<VariableInfo>) -> Rc<SmartCalcAstType> {
        variable.data.borrow().clone()
    }

    fn executer_assignment(config: &SmartCalcConfig, session: &Session, variable: Rc<VariableInfo>, expression: Rc<SmartCalcAstType>) -> Result<Rc<SmartCalcAstType>, String> {
        let computed  = Interpreter::execute_ast(config, session, expression)?;
        *variable.data.borrow_mut() = computed.clone();
        Ok(computed)
    }
    
    fn calculate_item(config: &SmartCalcConfig, operator: char, left: Rc<SmartCalcAstType>, right: Rc<SmartCalcAstType>) -> Result<Rc<SmartCalcAstType>, String> {
        let left = match left.deref() {
            SmartCalcAstType::Item(left) => left.clone(),
            _ => return Err("Unknown calculation".to_string())
        };
        
        let right = match right.deref() {
            SmartCalcAstType::Item(right) => right.clone(),
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
            Some(item) => Ok(Rc::new(SmartCalcAstType::Item(item.clone()))),
            None => Err("Unknown calculation".to_string())
        }
    }

    fn executer_binary(config: &SmartCalcConfig, session: &Session, left: Rc<SmartCalcAstType>, operator: char, right: Rc<SmartCalcAstType>) -> Result<Rc<SmartCalcAstType>, String> {
        let computed_left  = Interpreter::execute_ast(config, session, left)?;
        let computed_right = Interpreter::execute_ast(config, session, right)?;

        match (computed_left.deref(), computed_right.deref()) {
            (SmartCalcAstType::Item(_), _)           | (_, SmartCalcAstType::Item(_))           => Interpreter::calculate_item(config, operator, computed_left.clone(), computed_right.clone()),
            _ => Err("Uknown calculation result".to_string())
        }
    }

    fn executer_unary(config: &SmartCalcConfig, session: &Session, operator: char, ast: Rc<SmartCalcAstType>) -> Result<Rc<SmartCalcAstType>, String> {
        let computed = Interpreter::execute_ast(config, session, ast)?;

        let result = match operator {
            '+' => return Ok(computed),
            '-' => match computed.deref() {
                SmartCalcAstType::Item(item) => SmartCalcAstType::Item(item.unary(UnaryType::Minus)),
                _ => return Err("Syntax error".to_string())
            },
            _ => return Err("Syntax error".to_string())
        };

        Ok(Rc::new(result))
    }
}
