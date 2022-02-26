/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::any::{Any, TypeId};
use core::cell::RefCell;
use alloc::format;
use alloc::string::ToString;
use alloc::string::String;
use alloc::sync::Arc;
use chrono::{Duration, Timelike, NaiveDateTime};
use chrono_tz::Tz;
use chrono::TimeZone;
use crate::app::Session;
use crate::config::SmartCalcConfig;
use crate::types::TokenType;

use super::duration::DurationItem;
use super::{DataItem, OperationType, UnaryType};

#[derive(Debug)]

pub struct TimeItem(pub NaiveDateTime, pub Tz);

impl TimeItem {
    pub fn get_time(&self) -> NaiveDateTime {
        self.0.clone()
    }
    
    pub fn get_tz(&self) -> Tz {
        self.1.clone()
    }
}

impl DataItem for TimeItem {
    fn as_token_type(&self) -> TokenType {
        TokenType::Time(self.0, self.1)
    }
    fn is_same<'a>(&self, other: &'a dyn Any) -> bool {
        match other.downcast_ref::<NaiveDateTime>() {
            Some(l_value) => l_value == &self.0,
            None => false
        }
    }
    fn as_any(&self) -> &dyn Any { self }
    
    fn calculate(&self, _: &SmartCalcConfig, on_left: bool, other: &dyn DataItem, operation_type: OperationType) -> Option<Arc<dyn DataItem>> {
        /* If both item is money and current money is on left side, skip calculation */
        if TypeId::of::<Self>() == other.type_id() && !on_left {
            return None;
        }
        
        let (right, is_negative) = match other.type_name() {
            "DURATION" => {
                let duration = other.as_any().downcast_ref::<DurationItem>().unwrap();
                (duration.as_time(), duration.get_duration().num_seconds().is_negative())
            },
            "TIME" => (other.as_any().downcast_ref::<TimeItem>().unwrap().get_time(), false),
            _ => return None
        };

        let calculated_right = Duration::seconds(right.num_seconds_from_midnight() as i64);

        if is_negative {
            return Some(Arc::new(TimeItem(self.0 - calculated_right, self.1)));
        }
        
        match operation_type {
            OperationType::Add => Some(Arc::new(TimeItem(self.0 + calculated_right, self.1))),
            OperationType::Sub => Some(Arc::new(TimeItem(self.0 - calculated_right, self.1))),
            _ => None
        }
    }
    
    fn get_number(&self, _: &dyn DataItem) -> f64 {
       self.get_underlying_number()
    }
    
    fn get_underlying_number(&self) -> f64 { self.0.nanosecond() as f64 }
    fn type_name(&self) -> &'static str { "TIME" }
    fn type_id(&self) -> TypeId { TypeId::of::<TimeItem>() }
    fn print(&self, _: &SmartCalcConfig, _: &RefCell<Session>) -> String {
        let time = self.1.from_local_datetime(&self.0).unwrap();
        time.format("%H:%M:%S %Z").to_string()
    }
    fn unary(&self, _: UnaryType) -> Arc<dyn DataItem> {
        Arc::new(Self(self.0, self.1))
    }
}


#[cfg(test)]
#[test]
fn time_test() {
    use core::ops::Deref;
    use crate::compiler::time::TimeItem;
    use crate::config::SmartCalcConfig;
    let config = SmartCalcConfig::default();
    let session = RefCell::new(Session::default());

    assert_eq!(TimeItem(NaiveTime::from_hms(15, 25, 35)).print(&config, &session), "15:25:35".to_string());
    let left = TimeItem(NaiveTime::from_hms(15, 25, 35));
    let right = TimeItem(NaiveTime::from_hms(1, 25, 1));
    let result = left.calculate(&config, true, &right, OperationType::Add);
    
    assert!(result.is_some());
    assert_eq!(result.unwrap().deref().print(&config, &session), "16:50:36".to_string());
    
    let left = TimeItem(NaiveTime::from_hms(15, 25, 35));
    let right = TimeItem(NaiveTime::from_hms(1, 25, 1));
    let result = left.calculate(&config, true, &right, OperationType::Sub);
    
    assert!(result.is_some());
    assert_eq!(result.unwrap().deref().print(&config, &session), "14:00:34".to_string());
}