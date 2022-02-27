/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::any::{Any, TypeId};
use core::cell::RefCell;
use alloc::string::ToString;
use alloc::string::String;
use alloc::sync::Arc;
use chrono::{Datelike, Duration, Local, NaiveDate};
use crate::app::Session;
use crate::compiler::duration::DurationItem;
use crate::config::SmartCalcConfig;
use crate::formatter::{MONTH, YEAR, get_month_info, left_padding, uppercase_first_letter};
use crate::types::{TokenType, TimeOffset};

use super::{DataItem, OperationType, UnaryType};

#[derive(Debug)]

pub struct DateItem(pub NaiveDate, pub TimeOffset);

impl DateItem {
    pub fn get_date(&self) -> NaiveDate {
        self.0.clone()
    }
    
    pub fn get_tz(&self) -> TimeOffset {
        self.1.clone()
    }
    
    fn get_month_from_duration(&self, duration: Duration) -> i64 {
        duration.num_seconds().abs() / MONTH
    }

    fn get_year_from_duration(&self, duration: Duration) -> i64 {
        duration.num_seconds().abs() / YEAR
    }
}

impl DataItem for DateItem {
    fn as_token_type(&self) -> TokenType {
        TokenType::Date(self.0, self.1.clone())
    }
    fn is_same<'a>(&self, other: &'a dyn Any) -> bool {
        match other.downcast_ref::<NaiveDate>() {
            Some(l_value) => l_value == &self.0,
            None => false
        }
    }
    fn as_any(&self) -> &dyn Any { self }
    
    fn calculate(&self, _: &SmartCalcConfig, _: bool, other: &dyn DataItem, operation_type: OperationType) -> Option<Arc<dyn DataItem>> {
        /* If both item is money and current money is on left side, skip calculation */
        if other.type_name() != "DURATION" {
            return None;
        }

        let mut date = self.0;
        let mut duration = other.as_any().downcast_ref::<DurationItem>().unwrap().get_duration();

        return match operation_type {
            OperationType::Add => {
                match self.get_year_from_duration(duration) {
                    0 => (),
                    n => {
                        let years_diff = date.year() + n as i32;
                        date     = NaiveDate::from_ymd(years_diff as i32, date.month() as u32, date.day());
                        duration = Duration::seconds(duration.num_seconds() - (YEAR * n))
                    }
                };

                match self.get_month_from_duration(duration) {
                    0 => (),
                    n => {
                        let years_diff = (date.month() + n as u32) / 12;
                        let month = (date.month() + n as u32) % 12;
                        date     = NaiveDate::from_ymd(date.year() + years_diff as i32, month as u32, date.day());
                        duration = Duration::seconds(duration.num_seconds() - (MONTH * n))
                    }
                };
                Some(Arc::new(DateItem(date + duration, self.1.clone())))
            },

            OperationType::Sub => {
                match self.get_year_from_duration(duration) {
                    0 => (),
                    n => {
                        let years_diff = date.year() - n as i32;
                        date     = NaiveDate::from_ymd(years_diff as i32, date.month() as u32, date.day());
                        duration = Duration::seconds(duration.num_seconds() - (YEAR * n))
                    }
                };

                match self.get_month_from_duration(duration) {
                    0 => (),
                    n => {
                        let years = date.year() - (n as i32 / 12);
                        let mut months = date.month() as i32 - (n as i32 % 12);
                        if months < 0 {
                            months += 12;
                        }

                        date = NaiveDate::from_ymd(years as i32, months as u32, date.day());
                        duration = Duration::seconds(duration.num_seconds() - (MONTH * n))
                    }
                };
                Some(Arc::new(DateItem(date - duration, self.1.clone())))
            },
            _ => None
        };
    }
    
    fn get_number(&self, _: &dyn DataItem) -> f64 {
       self.get_underlying_number()
    }
    
    fn get_underlying_number(&self) -> f64 { 0.0 }
    fn type_name(&self) -> &'static str { "DATE" }
    fn type_id(&self) -> TypeId { TypeId::of::<DateItem>() }
    fn print(&self, config: &SmartCalcConfig, session: &RefCell<Session>) -> String {

        let format = match config.format.get( &session.borrow().get_language()) {
            Some(formats) => formats,
            _ => match config.format.get( "en") {
                Some(formats) => formats,
                _ => return "".to_string()
            }
        };
        
        let date_format = match self.0.year() == Local::now().date().year() {
            true => format.date.get("current_year"),
            false => format.date.get("full_date")
        };

        match date_format {
            Some(data) => {
                match get_month_info(config, &format.language, self.0.month() as u8) {
                    Some(month_info) => data.clone()
                        .replace("{day}", &self.0.day().to_string())
                        .replace("{month}", &self.0.month().to_string())
                        .replace("{day_pad}", &left_padding(self.0.day().into(), 2))
                        .replace("{month_pad}", &left_padding(self.0.month().into(), 2))
                        .replace("{month_long}", &uppercase_first_letter(&month_info.long))
                        .replace("{month_short}", &uppercase_first_letter(&month_info.short))
                        .replace("{year}", &self.0.year().to_string())
                        .replace("{timezone}", &self.1.name),
                    None => self.0.to_string()
                }
            },
            None => self.0.to_string()
        }
    }
    fn unary(&self, _: UnaryType) -> Arc<dyn DataItem> {
        Arc::new(Self(self.0, self.1.clone()))
    }
}


#[cfg(test)]
#[test]
fn time_test() {
    use crate::compiler::date::DateItem;
    use crate::compiler::duration::DurationItem;

    use crate::config::SmartCalcConfig;
    let config = SmartCalcConfig::default();
    let session = RefCell::new(Session::default());

    assert_eq!(DateItem(NaiveDate::from_ymd(2020, 1, 1)).print(&config, &session), "1 Jan 2020".to_string());

    let left = DateItem(NaiveDate::from_ymd(2020, 1, 1));
    let right = DateItem(NaiveDate::from_ymd(2020, 1, 1));
    let result = left.calculate(&config, true, &right, OperationType::Sub);
    
    assert!(result.is_none());

    let left = DateItem(NaiveDate::from_ymd(2020, 1, 1));
    let right = DurationItem(Duration::hours(24 * 20));
    let result = left.calculate(&config, true, &right, OperationType::Add);
    
    assert!(result.is_some());
    assert_eq!(result.unwrap().print(&config, &session), "21 Jan 2020".to_string());
}