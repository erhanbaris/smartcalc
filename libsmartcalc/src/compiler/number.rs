use core::any::{Any, TypeId};
use alloc::rc::Rc;
use alloc::string::{ToString, String};
use crate::config::SmartCalcConfig;
use super::percent::PercentItem;
use super::{DataItem, OperationType};

#[derive(Debug)]
#[derive(PartialEq)]
pub struct NumberItem(pub f64);
impl DataItem for NumberItem {
    fn as_any(&self) -> &dyn Any { self }
    fn get_raw_value(&self) -> &dyn Any {
        &self.0 as &dyn Any
    }
    fn calculate(&self, _: &SmartCalcConfig, on_left: bool, other: &dyn DataItem, operation_type: OperationType) -> Option<Rc<dyn DataItem>> {
        let other_number  = if TypeId::of::<NumberItem>() == other.type_id() { 
            other.get_underlying_number()
            
        } else if TypeId::of::<PercentItem>() == other.type_id() { 
            other.get_number(self)
            
        } else {
            return None;
        };
        
        let (left, right) = if on_left { 
            (self.0, other_number) 
        } else { 
            (other_number, self.0 ) 
        };
        
        let result = match operation_type {
            OperationType::Add => left + right,
            OperationType::Div => left / right,
            OperationType::Mul => left * right,
            OperationType::Sub => left - right
        };
        Some(Rc::new(NumberItem(result)))
    }
    fn get_number(&self, _: &dyn DataItem) -> f64 { self.0 }
    fn get_underlying_number(&self) -> f64 { self.0 }
    fn has_number(&self) -> bool { true }
    fn type_name(&self) -> &'static str { "NUMBER" }
    fn type_id(&self) -> TypeId { TypeId::of::<NumberItem>() }
    fn print(&self) -> String { self.0.to_string() }
}