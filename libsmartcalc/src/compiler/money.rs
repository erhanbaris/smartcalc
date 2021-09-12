use core::any::{Any, TypeId};
use alloc::format;
use alloc::string::ToString;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use crate::config::SmartCalcConfig;
use crate::types::CurrencyInfo;

use super::number::NumberItem;
use super::percent::PercentItem;
use super::{DataItem, OperationType};


#[derive(Debug)]
#[derive(PartialEq)]
pub struct MoneyItem(pub f64, pub Arc<CurrencyInfo>);

impl MoneyItem {
    fn get_currency(&self) -> Arc<CurrencyInfo> {
        self.1.clone()
    }
    
    fn get_price(&self) -> f64 {
        self.0
    }
    
    fn convert_currency(&self, config: &SmartCalcConfig, left: &MoneyItem) -> f64 {
        let as_usd = match config.currency_rate.get(&left.get_currency()) {
            Some(l_rate) => left.get_price() / l_rate,
            _ => 0.0
        };
    
        match config.currency_rate.get(&self.get_currency()) {
            Some(r_rate) => as_usd * r_rate,
            _ => 0.0
        }
    }
}

impl DataItem for MoneyItem {
    fn as_any(&self) -> &dyn Any { self }
    fn get_raw_value(&self) -> &dyn Any {
        self as &dyn Any
    }
    
    fn calculate(&self, config: &SmartCalcConfig, on_left: bool, other: &dyn DataItem, operation_type: OperationType) -> Option<Rc<dyn DataItem>> {
        /* If both item is money and current money is on left side, skip calculation */
        if TypeId::of::<MoneyItem>() == other.type_id() && on_left {
            return None;
        }
        
        let (other_amount, target_curreny)  = if TypeId::of::<NumberItem>() == other.type_id() { 
            (other.get_underlying_number(), self.1.clone())
            
        } else if TypeId::of::<MoneyItem>() == other.type_id() { 
            (self.convert_currency(config, other.as_any().downcast_ref::<MoneyItem>().unwrap()), self.1.clone())
            
        } else if TypeId::of::<PercentItem>() == other.type_id() { 
            (other.get_number(self), self.1.clone())
            
        } else {
            return None;
        };
        
        let (left, right) = if on_left { 
            (self.0, other_amount) 
        } else { 
            (other_amount, self.0 ) 
        };
        
        let result = match operation_type {
            OperationType::Add => left + right,
            OperationType::Div => left / right,
            OperationType::Mul => left * right,
            OperationType::Sub => left - right
        };
        Some(Rc::new(MoneyItem(result, target_curreny)))
    }
    
    fn get_number(&self, other: &dyn DataItem) -> f64 {
       if self.type_name() == other.type_name() {
           return self.0 
       }
       
       return other.get_underlying_number() * self.0
    }
    
    fn get_underlying_number(&self) -> f64 { self.0 }
    fn has_number(&self) -> bool { true }
    fn type_name(&self) -> &'static str { "MONEY" }
    fn type_id(&self) -> TypeId { TypeId::of::<MoneyItem>() }
    fn print(&self) -> String { 
        format!("{} {}", self.0.to_string(), self.1.to_string())
    }
}
