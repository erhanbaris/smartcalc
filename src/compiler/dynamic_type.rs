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
use alloc::vec::Vec;
use core::ops::Deref;
use crate::app::Session;
use crate::config::DynamicType;
use crate::config::SmartCalcConfig;
use crate::types::TokenType;
use crate::compiler::number::NumberItem;
use crate::types::NumberType;
use super::{DataItem, OperationType, UnaryType};
use crate::formatter::format_number;
use crate::tools::do_divition;

#[derive(Debug)]

pub struct DynamicTypeItem(pub f64, pub Arc<DynamicType>);

impl DynamicTypeItem {
    pub fn get_type(&self) -> Arc<DynamicType> {
        self.1.clone()
    }
    
    pub fn get_number(&self) -> f64 {
        self.0
    }
    
    pub fn convert(config: &SmartCalcConfig, number: f64, source_type: Arc<DynamicType>, target_type: String) -> Option<(f64, Arc<DynamicType>)> {
        let group = config.types.get(&source_type.group_name).unwrap();
        let values: Vec<Arc<DynamicType>> = group.values().cloned().collect();
        
        if let Some(target) = values.iter().find(|&s| s.names.contains(&target_type)) {
            if source_type.index == target.index {
                return Some((number, source_type.clone()));    
            }
                              
            let mut search_index = match source_type.index > target.index {
                true => source_type.index - 1,
                false => source_type.index + 1
            };
            
            let mut multiplier: f64 = source_type.multiplier;

            loop {
                let next_item = group.get(&search_index).unwrap();
                if next_item.index == target.index {
                    break;
                }
                
                multiplier *= next_item.multiplier;
                search_index = match source_type.index > target.index {
                    true => search_index - 1,
                    false => search_index + 1
                };
            }

            let new_number = match source_type.index > target.index {
                true => number * multiplier,
                false => number / multiplier
            };

            return Some((new_number, target.clone()))
        }
        None
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
    
    fn calculate(&self, config: &SmartCalcConfig, on_left: bool, other: &dyn DataItem, operation_type: OperationType) -> Option<Arc<dyn DataItem>> {
        let (other_number, is_same_type)  = match other.type_name() {
            "NUMBER" => (other.get_underlying_number(), false),
            "DYNAMIC_TYPE" => {
                let other_dynamic_type: &DynamicTypeItem = other.as_any().downcast_ref::<DynamicTypeItem>().unwrap();
                let (new_number, _) = DynamicTypeItem::convert(config, other_dynamic_type.get_number(), other_dynamic_type.get_type(), self.1.names[0].clone()).unwrap();
                (new_number, true)
            },
            "PERCENT" => (do_divition(self.0, 100.0) * other.get_underlying_number(), true),
            _ => return None
        };

        let (left, right) = if on_left { 
            (self.0, other_number) 
        } else { 
            (other_number, self.0) 
        };
        
        let result = match operation_type {
            OperationType::Add => left + right,
            OperationType::Div => {
                match is_same_type {
                    true => return Some(Arc::new(NumberItem(do_divition(left, right), NumberType::Decimal))),
                    false => do_divition(left, right)
                }
            },
            OperationType::Mul => left * right,
            OperationType::Sub => left - right
        };
        
        Some(Arc::new(DynamicTypeItem(result, self.1.clone())))
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