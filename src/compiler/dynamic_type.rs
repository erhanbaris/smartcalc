/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::any::{Any, TypeId};
use core::cell::RefCell;
use alloc::string::ToString;
use alloc::string::String;
use alloc::sync::Arc;
use core::ops::Deref;
use crate::app::Session;
use crate::config::DynamicType;
use crate::config::SmartCalcConfig;
use crate::types::TokenType;
use super::{DataItem, OperationType, UnaryType};
use crate::formatter::format_number;

#[derive(Debug)]

pub struct DynamicTypeItem(pub f64, pub Arc<DynamicType>);

impl DynamicTypeItem {
    pub fn get_type(&self) -> Arc<DynamicType> {
        self.1.clone()
    }
    
    pub fn get_number(&self) -> f64 {
        self.0
    }
}

impl DataItem for DynamicTypeItem {
    fn as_token_type(&self) -> TokenType {
        TokenType::DynamicType(self.0, self.1.clone())
    }
    fn is_same<'a>(&self, other: &'a dyn Any) -> bool {
        match other.downcast_ref::<(f64, Arc<DynamicType>)>() {
            Some((l_value, l_type)) => (l_value - self.0).abs() < f64::EPSILON && l_type.deref() == self.1.deref(),
            None => false
        }
    }
    fn as_any(&self) -> &dyn Any { self }
    
    fn calculate(&self, _: &SmartCalcConfig, _: bool, _: &dyn DataItem, _: OperationType) -> Option<Arc<dyn DataItem>> {
        Some(Arc::new(DynamicTypeItem(self.0, self.1.clone())))
    }
    
    fn get_number(&self, other: &dyn DataItem) -> f64 {
       if self.type_name() == other.type_name() {
           return self.0 
       }
       
       return other.get_underlying_number() * self.0
    }
    
    fn get_underlying_number(&self) -> f64 { self.0 }
    fn type_name(&self) -> &'static str { "DYNAMIC_TYPE" }
    fn type_id(&self) -> TypeId { TypeId::of::<DynamicTypeItem>() }
    fn print(&self, config: &SmartCalcConfig, _: &RefCell<Session>) -> String {
        let formated_number = format_number(self.0, config.thousand_separator.to_string(), config.decimal_seperator.to_string(), 2, true, true);
        self.1.format.replace("{value}", &formated_number)
    }
    fn unary(&self, unary: UnaryType) -> Arc<dyn DataItem> {
        match unary {
            UnaryType::Minus => Arc::new(Self(-1.0 * self.0, self.1.clone())),
            UnaryType::Plus => Arc::new(Self(self.0, self.1.clone()))
        }
    }
}


#[cfg(test)]
#[test]
fn format_result_test() {
    use crate::compiler::money::MoneyItem;
    use crate::config::SmartCalcConfig;
    let config = SmartCalcConfig::default();
    let session = RefCell::new(Session::default());

    let usd = config.get_currency("usd".to_string()).unwrap();
    let tl = config.get_currency("try".to_string()).unwrap();
    let uzs = config.get_currency("uzs".to_string()).unwrap();
    let uyu = config.get_currency("uyu".to_string()).unwrap();

    assert_eq!(MoneyItem(0.0, usd.clone()).print(&config, &session), "$0,00".to_string());
    assert_eq!(MoneyItem(0.05555, usd.clone()).print(&config, &session), "$0,06".to_string());
    assert_eq!(MoneyItem(123.05555, usd.clone()).print(&config, &session), "$123,06".to_string());
    assert_eq!(MoneyItem(1234.05555, usd.clone()).print(&config, &session), "$1.234,06".to_string());
    assert_eq!(MoneyItem(123456.05555, usd.clone()).print(&config, &session), "$123.456,06".to_string());
    assert_eq!(MoneyItem(123456.0, usd.clone()).print(&config, &session), "$123.456,00".to_string());

    assert_eq!(MoneyItem(0.0, tl.clone()).print(&config, &session), "₺0,00".to_string());
    assert_eq!(MoneyItem(0.05555, tl.clone()).print(&config, &session), "₺0,06".to_string());
    assert_eq!(MoneyItem(123.05555, tl.clone()).print(&config, &session), "₺123,06".to_string());
    assert_eq!(MoneyItem(1234.05555, tl.clone()).print(&config, &session), "₺1.234,06".to_string());
    assert_eq!(MoneyItem(123456.05555, tl.clone()).print(&config, &session), "₺123.456,06".to_string());
    assert_eq!(MoneyItem(123456.0, tl.clone()).print(&config, &session), "₺123.456,00".to_string());

    assert_eq!(MoneyItem(0.0, uzs.clone()).print(&config, &session), "0,00 сўм".to_string());
    assert_eq!(MoneyItem(0.05555, uzs.clone()).print(&config, &session), "0,06 сўм".to_string());
    assert_eq!(MoneyItem(123.05555, uzs.clone()).print(&config, &session), "123,06 сўм".to_string());
    assert_eq!(MoneyItem(1234.05555, uzs.clone()).print(&config, &session), "1.234,06 сўм".to_string());
    assert_eq!(MoneyItem(123456.05555, uzs.clone()).print(&config, &session), "123.456,06 сўм".to_string());
    assert_eq!(MoneyItem(123456.0, uzs.clone()).print(&config, &session), "123.456,00 сўм".to_string());

    assert_eq!(MoneyItem(0.0, uyu.clone()).print(&config, &session), "$U 0,00".to_string());
    assert_eq!(MoneyItem(0.05555, uyu.clone()).print(&config, &session), "$U 0,06".to_string());
    assert_eq!(MoneyItem(123.05555, uyu.clone()).print(&config, &session), "$U 123,06".to_string());
    assert_eq!(MoneyItem(1234.05555, uyu.clone()).print(&config, &session), "$U 1.234,06".to_string());
    assert_eq!(MoneyItem(123456.05555, uyu.clone()).print(&config, &session), "$U 123.456,06".to_string());
    assert_eq!(MoneyItem(123456.0, uyu.clone()).print(&config, &session), "$U 123.456,00".to_string());
}