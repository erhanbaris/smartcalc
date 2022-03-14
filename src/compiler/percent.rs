/*
 * smartcalc v1.0.6
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::any::{Any, TypeId};
use alloc::rc::Rc;
use alloc::string::{ToString, String};
use crate::session::Session;
use crate::config::SmartCalcConfig;
use crate::types::TokenType;
use super::{DataItem, OperationType, UnaryType};
use crate::formatter::format_number;
use alloc::format;
use crate::tools::do_divition;


#[derive(Debug)]

pub struct PercentItem(pub f64);
impl DataItem for PercentItem {
    fn as_token_type(&self) -> TokenType {
        TokenType::Percent(self.0)
    }
    fn is_same(&self, other: &dyn Any) -> bool {
        match other.downcast_ref::<f64>() {
            Some(value) => (value - self.0).abs() < f64::EPSILON,
            None => false
        }
    }
    fn as_any(&self) -> &dyn Any { self }
    fn calculate(&self, _: &SmartCalcConfig, on_left: bool, other: &dyn DataItem, operation_type: OperationType) -> Option<Rc<dyn DataItem>> {
        if TypeId::of::<Self>() != other.type_id() {
            return None;
        }
        
        let number = other.get_underlying_number();
        let (left, right) = if on_left { 
            (self.0, number) 
        } else { 
            (number, self.0 ) 
        };
        
        let result = match operation_type {
            OperationType::Add => left + right,
            OperationType::Div => left / right,
            OperationType::Mul => left * right,
            OperationType::Sub => left - right
        };
        Some(Rc::new(PercentItem(result)))
    }
    
    fn get_number(&self, other: &dyn DataItem) -> f64 {
       if self.type_name() == other.type_name() {
           return self.0 
       }
       
       do_divition(other.get_underlying_number(), 100.0) * self.0
    }
    
    fn get_underlying_number(&self) -> f64 { self.0 }
    fn type_name(&self) -> &'static str { "PERCENT" }
    fn type_id(&self) -> TypeId { TypeId::of::<PercentItem>() }
    fn print(&self, config: &SmartCalcConfig, _: &Session) -> String { format!("%{:}", format_number(self.0, config.thousand_separator.to_string(), config.decimal_seperator.to_string(), 2, true, true)) }
    fn unary(&self, unary: UnaryType) -> Rc<dyn DataItem> {
        match unary {
            UnaryType::Minus => Rc::new(Self(-1.0 * self.0)),
            UnaryType::Plus => Rc::new(Self(self.0))
        }
    }
}


#[cfg(test)]
#[test]
fn format_result_test() {
    use crate::config::SmartCalcConfig;
    let config = SmartCalcConfig::default();
    let session = Session::default();

    assert_eq!(PercentItem(0.0).print(&config, &session), "%0".to_string());
    assert_eq!(PercentItem(10.0).print(&config, &session), "%10".to_string());
    assert_eq!(PercentItem(10.1).print(&config, &session), "%10,10".to_string());
       
}
