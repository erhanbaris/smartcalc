use core::any::{Any, TypeId};
use alloc::rc::Rc;
use alloc::string::{ToString, String};
use crate::config::SmartCalcConfig;
use super::{DataItem, OperationType};


#[derive(Debug)]
#[derive(PartialEq)]
pub struct PercentItem(pub f64);
impl DataItem for PercentItem {
    fn as_any(&self) -> &dyn Any { self }
    fn get_raw_value(&self) -> &dyn Any {
        &self.0 as &dyn Any
    }
    
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
       
       return other.get_underlying_number() * self.0
    }
    
    fn get_underlying_number(&self) -> f64 { self.0 }
    fn has_number(&self) -> bool { true }
    fn type_name(&self) -> &'static str { "PERCENT" }
    fn type_id(&self) -> TypeId { TypeId::of::<PercentItem>() }
    fn print(&self) -> String { self.0.to_string() }
}