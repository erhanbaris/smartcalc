/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::any::{Any, TypeId};
use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::string::ToString;
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Deref;
use crate::session::Session;
use crate::config::DynamicType;
use crate::config::SmartCalcConfig;
use crate::types::TokenType;
use crate::compiler::number::NumberItem;
use crate::types::NumberType;
use super::{DataItem, OperationType, UnaryType};
use crate::formatter::format_number;
use crate::tools::do_divition;

#[derive(Debug)]

pub struct DynamicTypeItem(pub f64, pub Rc<DynamicType>);

impl DynamicTypeItem {
    pub fn get_type(&self) -> Rc<DynamicType> {
        self.1.clone()
    }
    
    pub fn get_number(&self) -> f64 {
        self.0
    }
    
    fn  calculate_unit(number: f64, source_type: Rc<DynamicType>, target_type: Rc<DynamicType>, group: &BTreeMap<usize, Rc<DynamicType>>) -> f64 {
        
        if source_type.index == target_type.index {
            return number;
        }
        
        let (mut search_index, multiplier) = match source_type.index > target_type.index {
            true => (source_type.index - 1, target_type.multiplier),
            false => (source_type.index + 1, source_type.multiplier)
        };

        let mut multiplier = multiplier.unwrap();
        
        loop {
            let next_item = group.get(&search_index).unwrap();
            if next_item.index == target_type.index {
                break;
            }
            
            multiplier *= match next_item.multiplier {
                Some(multiplier) => multiplier,
                None => 1.0
            };

            search_index = match source_type.index > target_type.index {
                true => search_index - 1,
                false => search_index + 1
            };
        }

        match source_type.index > target_type.index {
            true => number * multiplier,
            false => number / multiplier
        }
    }
    
    pub fn convert(config: &SmartCalcConfig, number: f64, source_type: Rc<DynamicType>, target_type: String) -> Option<(f64, Rc<DynamicType>)> {
        let group = config.types.get(&source_type.group_name).unwrap();
        let values: Vec<Rc<DynamicType>> = group.values().cloned().collect();
        
        if let Some(target) = values.iter().find(|&s| s.names.contains(&target_type)) {
            if source_type.index == target.index {
                return Some((number, source_type.clone()));    
            }
            
            let calculated_number = Self::calculate_unit(number, source_type.clone(), target.clone(), group);
            return Some((calculated_number, target.clone()))
        }
        
        let type_conversion = match config.type_conversion.iter().find(|&s| s.source.name == source_type.group_name || s.target.name == source_type.group_name) {
            Some(type_conversion) => type_conversion,
            None => return None
        };
        
        let (source_index, target_index) = match type_conversion.source.name == source_type.group_name {
            true => (type_conversion.source.index, type_conversion.target.index),
            false => (type_conversion.target.index, type_conversion.source.index)
        };
        
        let target_dynamic_type = match group.get(&source_index) {
            Some(target_type) => target_type,
            None => return None
        };
        
        let number = Self::calculate_unit(number, source_type.clone(), target_dynamic_type.clone(), group);        
        let number = match type_conversion.source.name == source_type.group_name {
            true => number * type_conversion.multiplier,
            false => number / type_conversion.multiplier
        };
        
        for (_, group) in config.types.iter() {
            for (_, target_dynamic_type) in group.iter() {
                
                if target_dynamic_type.names.contains(&target_type) {
                    let source_type = match group.get(&target_index) {
                        Some(source_type) => source_type,
                        None => return None
                    };
                    return Some((Self::calculate_unit(number, source_type.clone(), target_dynamic_type.clone(), group), target_dynamic_type.clone()));
                }
            }
        }
        
        None
    }
}

impl DataItem for DynamicTypeItem {
    fn as_token_type(&self) -> TokenType {
        TokenType::DynamicType(self.0, self.1.clone())
    }
    fn is_same(&self, other: &dyn Any) -> bool {
        match other.downcast_ref::<(f64, Rc<DynamicType>)>() {
            Some((l_value, l_type)) => (l_value - self.0).abs() < f64::EPSILON && l_type.deref() == self.1.deref(),
            None => false
        }
    }
    fn as_any(&self) -> &dyn Any { self }
    
    fn calculate(&self, config: &SmartCalcConfig, on_left: bool, other: &dyn DataItem, operation_type: OperationType) -> Option<Rc<dyn DataItem>> {
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
                    true => return Some(Rc::new(NumberItem(do_divition(left, right), NumberType::Decimal))),
                    false => do_divition(left, right)
                }
            },
            OperationType::Mul => left * right,
            OperationType::Sub => left - right
        };
        
        Some(Rc::new(DynamicTypeItem(result, self.1.clone())))
    }
    
    fn get_number(&self, other: &dyn DataItem) -> f64 {
       if self.type_name() == other.type_name() {
           return self.0 
       }
       
       other.get_underlying_number() * self.0
    }
    
    fn get_underlying_number(&self) -> f64 { self.0 }
    fn type_name(&self) -> &'static str { "DYNAMIC_TYPE" }
    fn type_id(&self) -> TypeId { TypeId::of::<DynamicTypeItem>() }
    fn print(&self, config: &SmartCalcConfig, _: &Session) -> String {
        let decimal_digit = self.1.decimal_digits.map_or(2, |x| x);
        let remove_fract_if_zero = self.1.remove_fract_if_zero.map_or(true, |x| x);
        let use_fract_rounding = self.1.use_fract_rounding.map_or(true, |x| x);

        let formated_number = format_number(self.0, config.thousand_separator.to_string(), config.decimal_seperator.to_string(), decimal_digit, remove_fract_if_zero, use_fract_rounding);
        self.1.format.replace("{value}", &formated_number)
    }
    fn unary(&self, unary: UnaryType) -> Rc<dyn DataItem> {
        match unary {
            UnaryType::Minus => Rc::new(Self(-1.0 * self.0, self.1.clone())),
            UnaryType::Plus => Rc::new(Self(self.0, self.1.clone()))
        }
    }
}

#[cfg(test)]
#[test]
fn format_result_test() {
    use crate::config::DynamicType;
    use crate::config::SmartCalcConfig;
    let config = SmartCalcConfig::default();
    let session = Session::default();

    let dynamic_type_1 = Rc::new(DynamicType::new("test".to_string(), 0, "{value} Test1".to_string(), Vec::new(), Some(1.0), None, None, Vec::new(), Some(5), Some(true), Some(true)));

    assert_eq!(DynamicTypeItem(1000.0, dynamic_type_1.clone()).print(&config, &session), "1.000 Test1".to_string());
    assert_eq!(DynamicTypeItem(10.0, dynamic_type_1.clone()).print(&config, &session), "10 Test1".to_string());
    assert_eq!(DynamicTypeItem(10.1, dynamic_type_1.clone()).print(&config, &session), "10,10000 Test1".to_string());

    let dynamic_type_2 = Rc::new(DynamicType::new("test".to_string(), 0, "Test2 {value}".to_string(), Vec::new(), Some(1.0), None, None, Vec::new(), Some(3), Some(false), Some(false)));
    assert_eq!(DynamicTypeItem(1000.0, dynamic_type_2.clone()).print(&config, &session), "Test2 1.000".to_string());
    assert_eq!(DynamicTypeItem(10.0, dynamic_type_2.clone()).print(&config, &session), "Test2 10".to_string());
    assert_eq!(DynamicTypeItem(10.1, dynamic_type_2.clone()).print(&config, &session), "Test2 10,1".to_string());
}