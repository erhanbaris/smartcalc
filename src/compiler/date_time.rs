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
use alloc::format;
use chrono::{Datelike, Local, NaiveDateTime, Timelike};
use crate::app::Session;
use crate::compiler::duration::DurationItem;
use crate::config::SmartCalcConfig;
use crate::formatter::{get_month_info, left_padding, uppercase_first_letter};
use crate::types::TokenType;

use super::{DataItem, OperationType, UnaryType};

#[derive(Debug)]

pub struct DateTimeItem(pub NaiveDateTime);

impl DateTimeItem {
    pub fn get_date_time(&self) -> NaiveDateTime {
        self.0.clone()
    }
}

impl DataItem for DateTimeItem {
    fn as_token_type(&self) -> TokenType {
        TokenType::DateTime(self.0)
    }
    fn is_same<'a>(&self, other: &'a dyn Any) -> bool {
        match other.downcast_ref::<NaiveDateTime>() {
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

        let date = self.0;
        let duration = other.as_any().downcast_ref::<DurationItem>().unwrap().get_duration();
        match operation_type {
            OperationType::Add => Some(Arc::new(DateTimeItem(date + duration))),
            OperationType::Sub => Some(Arc::new(DateTimeItem(date - duration))),
            _ => None
        }
    }
    
    fn get_number(&self, _: &dyn DataItem) -> f64 {
       self.get_underlying_number()
    }
    
    fn get_underlying_number(&self) -> f64 { 0.0 }
    fn type_name(&self) -> &'static str { "DATE_TIME" }
    fn type_id(&self) -> TypeId { TypeId::of::<DateTimeItem>() }
    fn print(&self, config: &SmartCalcConfig, session: &RefCell<Session>) -> String {

        let format = match config.format.get( &session.borrow().get_language()) {
            Some(formats) => formats,
            _ => match config.format.get( "en") {
                Some(formats) => formats,
                _ => return "".to_string()
            }
        };
        
        let date_format = match self.0.year() == Local::now().date().year() {
            true => format.date.get("current_year_with_time"),
            false => format.date.get("full_date_time")
        };

        match date_format {
            Some(data) => {
                match get_month_info(config, &format.language, self.0.month() as u8) {
                    Some(month_info) => data.clone()
                        .replace("{second_pad}", &format!("{:02}", self.0.second()))
                        .replace("{minute_pad}", &format!("{:02}", self.0.minute()))
                        .replace("{hour_pad}", &format!("{:02}", self.0.hour()))
                        .replace("{second}", &self.0.second().to_string())
                        .replace("{minute}", &self.0.minute().to_string())
                        .replace("{hour}", &self.0.hour().to_string())
                        .replace("{day}", &self.0.day().to_string())
                        .replace("{month}", &self.0.month().to_string())
                        .replace("{day_pad}", &left_padding(self.0.day().into(), 2))
                        .replace("{month_pad}", &left_padding(self.0.month().into(), 2))
                        .replace("{month_long}", &uppercase_first_letter(&month_info.long))
                        .replace("{month_short}", &uppercase_first_letter(&month_info.short))
                        .replace("{year}", &self.0.year().to_string()),
                    None => self.0.to_string()
                }
            },
            None => self.0.to_string()
        }
    }
    fn unary(&self, _: UnaryType) -> Arc<dyn DataItem> {
        Arc::new(Self(self.0))
    }
}

#[cfg(test)]
#[test]
fn time_test() {
    use chrono::{Duration, NaiveDate};

    use crate::executer::initialize;
    use crate::compiler::date_time::DateTimeItem;
    use crate::compiler::duration::DurationItem;

    initialize();
    use crate::config::SmartCalcConfig;
    let config = SmartCalcConfig::default();
    let session = RefCell::new(Session::default());

    assert_eq!(DateTimeItem(NaiveDate::from_ymd(2020, 1, 1).and_hms(1, 12, 13)).print(&config, &session), "1 Jan 2020 01:12:13".to_string());

    let left = DateTimeItem(NaiveDate::from_ymd(2020, 1, 1).and_hms(1, 1, 1));
    let right = DateTimeItem(NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0));
    let result = left.calculate(&config, true, &right, OperationType::Sub);
    
    assert!(result.is_none());

    let left = DateTimeItem(NaiveDate::from_ymd(2020, 1, 1).and_hms(1, 0, 0));
    let right = DurationItem(Duration::hours(1));
    let result = left.calculate(&config, true, &right, OperationType::Sub);
    
    
    assert!(result.is_some());
    assert_eq!(result.unwrap().print(&config, &session), "1 Jan 2020 00:00:00".to_string());
}
