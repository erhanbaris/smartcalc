use core::any::{Any, TypeId};
use alloc::string::ToString;
use alloc::string::String;
use alloc::sync::Arc;
use chrono::{Duration, NaiveTime, Timelike};
use crate::config::SmartCalcConfig;
use crate::types::TokenType;

use super::{DataItem, OperationType, UnaryType};

#[derive(Debug)]

pub struct TimeItem(pub NaiveTime);

impl TimeItem {
    pub fn get_time(&self) -> NaiveTime {
        self.0.clone()
    }
}

impl DataItem for TimeItem {
    fn as_token_type(&self) -> TokenType {
        TokenType::Time(self.0)
    }
    fn is_same<'a>(&self, other: &'a dyn Any) -> bool {
        match other.downcast_ref::<NaiveTime>() {
            Some(l_value) => l_value == &self.0,
            None => false
        }
    }
    fn as_any(&self) -> &dyn Any { self }
    
    fn calculate(&self, _: &SmartCalcConfig, on_left: bool, other: &dyn DataItem, operation_type: OperationType) -> Option<Arc<dyn DataItem>> {
        /* If both item is money and current money is on left side, skip calculation */
        if TypeId::of::<Self>() != other.type_id() && on_left {
            return None;
        }
        
        let right = match other.as_any().downcast_ref::<Self>() {
            Some(time) => Duration::seconds(time.get_time().num_seconds_from_midnight() as i64),
            _ => return None
        };
        
        let result = match operation_type {
            OperationType::Add => self.0 + right,
            OperationType::Sub => self.0 - right,
            _ => return None
        };
        
        Some(Arc::new(TimeItem(NaiveTime::from_hms(result.hour(), result.minute(), result.second()))))
    }
    
    fn get_number(&self, _: &dyn DataItem) -> f64 {
       self.get_underlying_number()
    }
    
    fn get_underlying_number(&self) -> f64 { self.0.nanosecond() as f64 }
    fn type_name(&self) -> &'static str { "TIME" }
    fn type_id(&self) -> TypeId { TypeId::of::<TimeItem>() }
    fn print(&self, _: &SmartCalcConfig) -> String {
        self.0.to_string()
    }
    fn unary(&self, _: UnaryType) -> Arc<dyn DataItem> {
        Arc::new(Self(self.0))
    }
}


#[cfg(test)]
#[test]
fn time_test() {
    use core::ops::Deref;
    use crate::executer::initialize;
    use crate::compiler::time::TimeItem;
    initialize();
    use crate::config::SmartCalcConfig;
    let config = SmartCalcConfig::default();

    assert_eq!(TimeItem(NaiveTime::from_hms(15, 25, 35)).print(&config), "15:25:35".to_string());
    let left = TimeItem(NaiveTime::from_hms(15, 25, 35));
    let right = TimeItem(NaiveTime::from_hms(1, 25, 1));
    let result = left.calculate(&config, true, &right, OperationType::Add);
    
    assert!(result.is_some());
    assert_eq!(result.unwrap().deref().print(&config), "16:50:36".to_string());
    
    let left = TimeItem(NaiveTime::from_hms(15, 25, 35));
    let right = TimeItem(NaiveTime::from_hms(1, 25, 1));
    let result = left.calculate(&config, true, &right, OperationType::Sub);
    
    assert!(result.is_some());
    assert_eq!(result.unwrap().deref().print(&config), "14:00:34".to_string());
}