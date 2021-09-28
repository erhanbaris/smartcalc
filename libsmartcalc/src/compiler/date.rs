use core::any::{Any, TypeId};
use core::cell::RefCell;
use alloc::string::ToString;
use alloc::string::String;
use alloc::sync::Arc;
use chrono::{Duration, NaiveDate, NaiveTime, Timelike};
use crate::app::Session;
use crate::config::SmartCalcConfig;
use crate::types::TokenType;

use super::duration::DurationItem;
use super::{DataItem, OperationType, UnaryType};

#[derive(Debug)]

pub struct DateItem(pub NaiveDate);

impl DateItem {
    pub fn get_date(&self) -> NaiveDate {
        self.0.clone()
    }
}

impl DataItem for DateItem {
    fn as_token_type(&self) -> TokenType {
        TokenType::Date(self.0)
    }
    fn is_same<'a>(&self, other: &'a dyn Any) -> bool {
        match other.downcast_ref::<NaiveDate>() {
            Some(l_value) => l_value == &self.0,
            None => false
        }
    }
    fn as_any(&self) -> &dyn Any { self }
    
    fn calculate(&self, _: &SmartCalcConfig, on_left: bool, other: &dyn DataItem, operation_type: OperationType) -> Option<Arc<dyn DataItem>> {
        None
    }
    
    fn get_number(&self, _: &dyn DataItem) -> f64 {
       self.get_underlying_number()
    }
    
    fn get_underlying_number(&self) -> f64 { 0.0 }
    fn type_name(&self) -> &'static str { "DATE" }
    fn type_id(&self) -> TypeId { TypeId::of::<DateItem>() }
    fn print(&self, _: &SmartCalcConfig, _: &RefCell<Session>) -> String {
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
    use crate::compiler::time::DateItem;
    initialize();
    use crate::config::SmartCalcConfig;
    let config = SmartCalcConfig::default();
    let session = RefCell::new(Session::default());

    assert_eq!(DateItem(NaiveTime::from_hms(15, 25, 35)).print(&config, &session), "15:25:35".to_string());
    let left = DateItem(NaiveTime::from_hms(15, 25, 35));
    let right = DateItem(NaiveTime::from_hms(1, 25, 1));
    let result = left.calculate(&config, true, &right, OperationType::Add);
    
    assert!(result.is_some());
    assert_eq!(result.unwrap().deref().print(&config, &session), "16:50:36".to_string());
    
    let left = DateItem(NaiveTime::from_hms(15, 25, 35));
    let right = DateItem(NaiveTime::from_hms(1, 25, 1));
    let result = left.calculate(&config, true, &right, OperationType::Sub);
    
    assert!(result.is_some());
    assert_eq!(result.unwrap().deref().print(&config, &session), "14:00:34".to_string());
}