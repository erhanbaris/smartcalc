use core::any::{Any, TypeId};
use alloc::string::{ToString, String};
use alloc::sync::Arc;
use crate::config::SmartCalcConfig;
use crate::types::TokenType;
use super::percent::PercentItem;
use super::{DataItem, OperationType, UnaryType};
use crate::formatter::format_number;
use crate::tools::do_divition;

#[derive(Debug)]

pub struct NumberItem(pub f64);
impl DataItem for NumberItem {
    fn as_token_type(&self) -> TokenType {
        TokenType::Number(self.0)
    }
    fn is_same<'a>(&self, other: &'a dyn Any) -> bool {
        match other.downcast_ref::<f64>() {
            Some(value) => (value - self.0).abs() < f64::EPSILON,
            None => false
        }
    }
    fn as_any(&self) -> &dyn Any { self }
    fn calculate(&self, _: &SmartCalcConfig, on_left: bool, other: &dyn DataItem, operation_type: OperationType) -> Option<Arc<dyn DataItem>> {
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
            OperationType::Div => do_divition(left, right),
            OperationType::Mul => left * right,
            OperationType::Sub => left - right
        };
        Some(Arc::new(NumberItem(result)))
    }
    fn get_number(&self, _: &dyn DataItem) -> f64 { self.0 }
    fn get_underlying_number(&self) -> f64 { self.0 }
    fn type_name(&self) -> &'static str { "NUMBER" }
    fn type_id(&self) -> TypeId { TypeId::of::<NumberItem>() }
    fn print(&self, _: &SmartCalcConfig) -> String { format_number(self.0, ".".to_string(), ",".to_string(), 3, true, true) }
    fn unary(&self, unary: UnaryType) -> Arc<dyn DataItem> {
        match unary {
            UnaryType::Minus => Arc::new(Self(-1.0 * self.0)),
            UnaryType::Plus => Arc::new(Self(self.0))
        }
    }
}
