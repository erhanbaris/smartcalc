use core::any::{Any, TypeId};
use core::borrow::Borrow;
use core::cell::RefCell;
use alloc::string::ToString;
use alloc::string::String;
use alloc::sync::Arc;
use chrono::Duration;
use chrono::NaiveTime;
use crate::app::Session;
use crate::config::SmartCalcConfig;
use crate::constants::DurationFormatType;
use crate::constants::JsonFormat;
use crate::formatter::{DAY, format_result};
use crate::formatter::HOUR;
use crate::formatter::MINUTE;
use crate::formatter::MONTH;
use crate::formatter::WEEK;
use crate::formatter::YEAR;
use crate::types::TokenType;
use core::write;
use alloc::fmt::Write;

use super::AsNaiveTime;
use super::{DataItem, OperationType, UnaryType};

#[derive(Debug)]

pub struct DurationItem(pub Duration);

impl DurationItem {
    pub fn get_duration(&self) -> Duration {
        self.0.clone()
    }

    fn duration_formatter(format: &JsonFormat, buffer: &mut String, replace_str: &str, duration: i64, duration_type: DurationFormatType) {
        for format_item in format.duration.iter() {
            if format_item.duration_type == duration_type && format_item.count.trim().parse::<i64>().is_ok() && format_item.count.trim().parse::<i64>().unwrap() == duration{
                write!(buffer, "{} ", format_item.format.to_string().replace(replace_str, &duration.to_string())).unwrap();
                return;
            }
        }
    
        for format_item in format.duration.iter() {
            if format_item.duration_type == duration_type && format_item.count.trim().parse::<i64>().is_err() {
                write!(buffer, "{} ", format_item.format.to_string().replace(replace_str, &duration.to_string())).unwrap();
                return;
            }
        }
    
        write!(buffer, "{} ", duration.to_string()).unwrap();
    }
}

impl AsNaiveTime for DurationItem {
    fn as_naive_time(&self) -> NaiveTime {
        let mut duration_info = self.0.num_seconds().abs();
        let mut hours         = 0;
        let mut minutes       = 0;
        let seconds;

        if duration_info >= HOUR {
            hours = (duration_info / HOUR) % 24;
            duration_info %= HOUR
        }

        if duration_info >= MINUTE {
            minutes = (duration_info / MINUTE) % 60;
            duration_info %= MINUTE
        }

        seconds = duration_info;
        NaiveTime::from_hms(hours as u32, minutes as u32, seconds as u32)
    }
}

impl DataItem for DurationItem {
    fn as_token_type(&self) -> TokenType {
        TokenType::Duration(self.0)
    }
    fn is_same<'a>(&self, other: &'a dyn Any) -> bool {
        match other.downcast_ref::<Duration>() {
            Some(l_value) => l_value == &self.0,
            None => false
        }
    }
    fn as_any(&self) -> &dyn Any { self }
    
    fn calculate(&self, _: &SmartCalcConfig, on_left: bool, other: &dyn DataItem, _: OperationType) -> Option<Arc<dyn DataItem>> {
        /* If both item is money and current money is on left side, skip calculation */
        if TypeId::of::<Self>() != other.type_id() && on_left {
            return None;
        }
        
        Some(Arc::new(DurationItem(Duration::seconds(0))))
    }
    
    fn get_number(&self, _: &dyn DataItem) -> f64 {
       self.get_underlying_number()
    }
    
    fn get_underlying_number(&self) -> f64 { self.0.num_seconds() as f64 }
    fn type_name(&self) -> &'static str { "DURATION" }
    fn type_id(&self) -> TypeId { TypeId::of::<DurationItem>() }
    fn print(&self, config: &SmartCalcConfig, session: &RefCell<Session>) -> String {
        
        let format = match config.format.get( &session.borrow().get_language()) {
            Some(formats) => formats,
            _ => return "".to_string()
        };
        
        let mut buffer = String::new();

            let mut duration = self.0.num_seconds().abs();
            if duration >= YEAR {
                DurationItem::duration_formatter(format, &mut buffer, "{year}", duration / YEAR, DurationFormatType::Year);
                duration %= YEAR;
            }
    
            if duration >= MONTH {
                DurationItem::duration_formatter(format, &mut buffer, "{month}", duration / MONTH, DurationFormatType::Month);
                duration %= MONTH;
            }
    
            if duration >= WEEK {
                DurationItem::duration_formatter(format, &mut buffer, "{week}", duration / WEEK, DurationFormatType::Week);
                duration %= WEEK;
            }
    
            if duration >= DAY {
                DurationItem::duration_formatter(format, &mut buffer, "{day}", duration / DAY, DurationFormatType::Day);
                duration %= DAY;
            }
    
            if duration >= HOUR {
                DurationItem::duration_formatter(format, &mut buffer, "{hour}", duration / HOUR, DurationFormatType::Hour);
                duration %= HOUR;
            }
    
            if duration >= MINUTE {
                DurationItem::duration_formatter(format, &mut buffer, "{minute}", duration / MINUTE, DurationFormatType::Minute);
                duration %= MINUTE;
            }
    
            if duration > 0 {
                DurationItem::duration_formatter(format, &mut buffer, "{second}", duration, DurationFormatType::Second);
            }
    
            buffer
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