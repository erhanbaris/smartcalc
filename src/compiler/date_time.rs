/*
 * smartcalc v1.0.7
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::any::{Any, TypeId};
use alloc::rc::Rc;
use alloc::string::ToString;
use alloc::string::String;
use alloc::format;
use chrono::{Datelike, NaiveDateTime, Timelike, Utc};
use chrono::TimeZone;
use crate::session::Session;
use crate::compiler::duration::DurationItem;
use crate::config::SmartCalcConfig;
use crate::formatter::{get_month_info, left_padding, uppercase_first_letter};
use crate::types::{TokenType, TimeOffset};

use super::{DataItem, OperationType, UnaryType};

#[derive(Debug)]

pub struct DateTimeItem(pub NaiveDateTime, pub TimeOffset);

impl DateTimeItem {
    pub fn get_date_time(&self) -> NaiveDateTime {
        self.0
    }
    
    pub fn get_tz(&self) -> TimeOffset {
        self.1.clone()
    }
}

impl DataItem for DateTimeItem {
    fn as_token_type(&self) -> TokenType {
        TokenType::DateTime(self.0, self.1.clone())
    }
    fn is_same(&self, other: &dyn Any) -> bool {
        match other.downcast_ref::<NaiveDateTime>() {
            Some(l_value) => l_value == &self.0,
            None => false
        }
    }
    fn as_any(&self) -> &dyn Any { self }
    
    fn calculate(&self, _: &SmartCalcConfig, _: bool, other: &dyn DataItem, operation_type: OperationType) -> Option<Rc<dyn DataItem>> {
        /* If both item is money and current money is on left side, skip calculation */
        if other.type_name() != "DURATION" {
            return None;
        }

        let date = self.0;
        let duration = other.as_any().downcast_ref::<DurationItem>().unwrap().get_duration();
        match operation_type {
            OperationType::Add => Some(Rc::new(DateTimeItem(date + duration, self.1.clone()))),
            OperationType::Sub => Some(Rc::new(DateTimeItem(date - duration, self.1.clone()))),
            _ => None
        }
    }
    
    fn get_number(&self, _: &dyn DataItem) -> f64 {
       self.get_underlying_number()
    }
    
    fn get_underlying_number(&self) -> f64 { 0.0 }
    fn type_name(&self) -> &'static str { "DATE_TIME" }
    fn type_id(&self) -> TypeId { TypeId::of::<DateTimeItem>() }
    fn print(&self, config: &SmartCalcConfig, session: &Session) -> String {

        let format = match config.format.get( &session.get_language()) {
            Some(formats) => formats,
            _ => match config.format.get( "en") {
                Some(formats) => formats,
                _ => return "".to_string()
            }
        };
        
        let tz_offset = chrono::FixedOffset::east(self.1.offset * 60);
        let datetime = tz_offset.from_utc_datetime(&self.0);
        
        let date_format = match datetime.year() == Utc::now().date().year() {
            true => format.date.get("current_year_with_time"),
            false => format.date.get("full_date_time")
        };

        match date_format {
            Some(data) => {
                match get_month_info(config, &format.language, datetime.month() as u8) {
                    Some(month_info) => data.clone()
                        .replace("{second_pad}", &format!("{:02}", datetime.second()))
                        .replace("{minute_pad}", &format!("{:02}", datetime.minute()))
                        .replace("{hour_pad}", &format!("{:02}", datetime.hour()))
                        .replace("{second}", &datetime.second().to_string())
                        .replace("{minute}", &datetime.minute().to_string())
                        .replace("{hour}", &datetime.hour().to_string())
                        .replace("{day}", &datetime.day().to_string())
                        .replace("{month}", &datetime.month().to_string())
                        .replace("{day_pad}", &left_padding(datetime.day().into(), 2))
                        .replace("{month_pad}", &left_padding(datetime.month().into(), 2))
                        .replace("{month_long}", &uppercase_first_letter(&month_info.long))
                        .replace("{month_short}", &uppercase_first_letter(&month_info.short))
                        .replace("{year}", &datetime.year().to_string())
                        .replace("{timezone}", &self.1.name),
                    None => datetime.to_string()
                }
            },
            None => datetime.to_string()
        }
    }
    fn unary(&self, _: UnaryType) -> Rc<dyn DataItem> {
        Rc::new(Self(self.0, self.1.clone()))
    }
}

#[cfg(test)]
#[test]
fn date_time_test() {
    use chrono::{Duration, NaiveDate};

    use crate::compiler::date_time::DateTimeItem;
    use crate::compiler::duration::DurationItem;

    use crate::config::SmartCalcConfig;
    let config = SmartCalcConfig::default();
    let session = Session::default();

    assert_eq!(DateTimeItem(NaiveDate::from_ymd(2020, 1, 1).and_hms(1, 12, 13), config.get_time_offset()).print(&config, &session), "1 Jan 2020 01:12:13 UTC".to_string());

    let left = DateTimeItem(NaiveDate::from_ymd(2020, 1, 1).and_hms(1, 1, 1), config.get_time_offset());
    let right = DateTimeItem(NaiveDate::from_ymd(2020, 1, 1).and_hms(0, 0, 0), config.get_time_offset());
    let result = left.calculate(&config, true, &right, OperationType::Sub);
    
    assert!(result.is_none());

    let left = DateTimeItem(NaiveDate::from_ymd(2020, 1, 1).and_hms(1, 0, 0), config.get_time_offset());
    let right = DurationItem(Duration::hours(1));
    let result = left.calculate(&config, true, &right, OperationType::Sub);
    
    
    assert!(result.is_some());
    assert_eq!(result.unwrap().print(&config, &session), "1 Jan 2020 00:00:00 UTC".to_string());
}
