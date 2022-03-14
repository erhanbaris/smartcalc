/*
 * smartcalc v1.0.6
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::any::{Any, TypeId};
use alloc::rc::Rc;
use alloc::string::ToString;
use alloc::string::String;
use chrono::{Duration, NaiveDateTime, Utc};
use crate::session::Session;
use crate::config::SmartCalcConfig;
use crate::constants::DurationFormatType;
use crate::constants::JsonFormat;
use crate::formatter::DAY;
use crate::formatter::HOUR;
use crate::formatter::MINUTE;
use crate::formatter::MONTH;
use crate::formatter::WEEK;
use crate::formatter::YEAR;
use crate::types::TokenType;
use core::write;
use alloc::fmt::Write;

use super::{DataItem, OperationType, UnaryType};

#[derive(Debug)]

pub struct DurationItem(pub Duration);

impl DurationItem {
    pub fn get_duration(&self) -> Duration {
        self.0
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

    fn get_high_duration_number(&self) -> i64 {
        let duration_info = self.0.num_seconds().abs();
        if duration_info >= YEAR {
            return duration_info / YEAR;
        }

        if duration_info >= MONTH {
            return (duration_info / MONTH) % 30;
        }

        if duration_info >= DAY {
            return duration_info / DAY;
        }

        if duration_info >= HOUR {
            return (duration_info / HOUR) % 24;
        }

        if duration_info >= MINUTE {
            return (duration_info / MINUTE) % 60;
        }

        duration_info
    }

    pub fn as_time(&self) -> NaiveDateTime {
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
        
        let date = Utc::now().naive_local().date();
        let time = chrono::NaiveTime::from_hms(hours as u32, minutes as u32, seconds as u32);
        NaiveDateTime::new(date, time)
    }
}

impl DataItem for DurationItem {
    fn as_token_type(&self) -> TokenType {
        TokenType::Duration(self.0)
    }
    fn is_same(&self, other: &dyn Any) -> bool {
        match other.downcast_ref::<Duration>() {
            Some(l_value) => l_value == &self.0,
            None => false
        }
    }
    fn as_any(&self) -> &dyn Any { self }
    
    fn calculate(&self, _: &SmartCalcConfig, on_left: bool, other: &dyn DataItem, operation_type: OperationType) -> Option<Rc<dyn DataItem>> {
        /* If both item is money and current money is on left side, skip calculation */
        if TypeId::of::<Self>() != other.type_id() && on_left {
            return None;
        }

        match operation_type {
            OperationType::Add => Some(Rc::new(DurationItem(self.0 + other.as_any().downcast_ref::<Self>().unwrap().get_duration()))),
            OperationType::Sub => Some(Rc::new(DurationItem(self.0 - other.as_any().downcast_ref::<Self>().unwrap().get_duration()))),
            _ => None
        }
    }
    
    fn get_number(&self, _: &dyn DataItem) -> f64 {
       self.get_high_duration_number() as f64
    }
    
    fn get_underlying_number(&self) -> f64 { self.0.num_seconds() as f64 }
    fn type_name(&self) -> &'static str { "DURATION" }
    fn type_id(&self) -> TypeId { TypeId::of::<DurationItem>() }
    fn print(&self, config: &SmartCalcConfig, session: &Session) -> String {
        
        let format = match config.format.get( &session.get_language()) {
            Some(formats) => formats,
            _ => match config.format.get( "en") {
                Some(formats) => formats,
                _ => return "".to_string()
            }
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

        buffer.trim().to_string()
    }
    fn unary(&self, _: UnaryType) -> Rc<dyn DataItem> {
        Rc::new(Self(self.0))
    }
}


#[cfg(test)]
#[test]
fn duration_test() {
    use crate::compiler::duration::DurationItem;
    use crate::config::SmartCalcConfig;
    let config = SmartCalcConfig::default();
    let session = Session::default();

    assert_eq!(DurationItem(Duration::hours(12)).print(&config, &session), "12 hours".to_string());
    assert_eq!(DurationItem(Duration::hours(24)).print(&config, &session), "1 day".to_string());
    assert_eq!(DurationItem(Duration::hours(25)).print(&config, &session), "1 day 1 hour".to_string());
    assert_eq!(DurationItem(Duration::hours(48)).print(&config, &session), "2 days".to_string());
    
    assert_eq!(DurationItem(Duration::minutes(48)).print(&config, &session), "48 minutes".to_string());
    assert_eq!(DurationItem(Duration::minutes(60)).print(&config, &session), "1 hour".to_string());
    assert_eq!(DurationItem(Duration::minutes(61)).print(&config, &session), "1 hour 1 minute".to_string());
    assert_eq!(DurationItem(Duration::minutes(161)).print(&config, &session), "2 hours 41 minutes".to_string());

    assert_eq!(DurationItem(Duration::seconds(1)).print(&config, &session), "1 second".to_string());
    assert_eq!(DurationItem(Duration::seconds(30)).print(&config, &session), "30 seconds".to_string());

    let left = DurationItem(Duration::hours(15));
    let right = DurationItem(Duration::minutes(1));
    let result = left.calculate(&config, true, &right, OperationType::Add);
    
    assert!(result.is_some());
    assert_eq!(result.unwrap().print(&config, &session), "15 hours 1 minute".to_string());

    let left = DurationItem(Duration::hours(15));
    let right = DurationItem(Duration::minutes(1));
    let result = left.calculate(&config, true, &right, OperationType::Sub);
    
    assert!(result.is_some());
    assert_eq!(result.unwrap().print(&config, &session), "14 hours 59 minutes".to_string());
}