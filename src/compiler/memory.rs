/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::any::{Any, TypeId};
use core::cell::RefCell;
use alloc::string::{ToString, String};
use core::ops::Deref;
use alloc::sync::Arc;
use crate::app::Session;
use crate::config::SmartCalcConfig;
use crate::formatter::format_number;
use crate::types::{MemoryType, TokenType};
use super::number::NumberItem;
use super::{DataItem, OperationType, UnaryType};
use alloc::format;

#[derive(Debug)]
pub struct MemoryItem(pub f64, pub MemoryType);

impl MemoryItem {
    pub fn get_memory(&self) -> f64 {
        self.0.clone()
    }
    pub fn get_memory_type(&self) -> MemoryType {
        self.1.clone()
    }
}

impl DataItem for MemoryItem {
    fn as_token_type(&self) -> TokenType {
        TokenType::Memory(self.0.clone(), self.1.clone())
    }
    fn is_same<'a>(&self, other: &'a dyn Any) -> bool {
        match other.downcast_ref::<(f64, Arc<MemoryType>)>() {
            Some((l_value, l_symbol)) => (l_value - self.0).abs() < f64::EPSILON && l_symbol.deref().clone() == self.1.clone(),
            None => false
        }
    }
    fn as_any(&self) -> &dyn Any { self }
    
    fn calculate(&self, _: &SmartCalcConfig, on_left: bool, other: &dyn DataItem, operation_type: OperationType) -> Option<Arc<dyn DataItem>> {
        /* If both item is money and current money is on left side, skip calculation */
        if TypeId::of::<Self>() == other.type_id() && !on_left {
            return None;
        }

        let (memory, memory_type) = match other.type_name() {
            "MEMORY" => {
                let other_memory = other.as_any().downcast_ref::<Self>().unwrap();
                (other_memory.get_memory(), other_memory.get_memory_type())
            },
            "NUMBER" => (other.as_any().downcast_ref::<NumberItem>().unwrap().get_underlying_number(), self.get_memory_type()),
            _ => return None
        };

        let distance = (self.get_memory_type() as i32 - memory_type.clone() as i32).abs();
        let divition = 1024.0_f32.powi(distance) as f64;

        let (left, right, target_type) = match self.get_memory_type() as i32 > memory_type.clone() as i32 {
            true => (self.get_memory(), memory as f64 / divition, self.get_memory_type()),
            false => (self.get_memory() as f64 / divition, memory, memory_type)
        };

        let operation_result = match operation_type {
            OperationType::Add => left + right,
            OperationType::Sub => left - right,
            OperationType::Div => left / right,
            OperationType::Mul => left * right
        };

        match operation_result.is_infinite() || operation_result.is_nan() {
            true => None,
            false => Some(Arc::new(MemoryItem(operation_result, target_type)))
        }
    }
    
    fn get_number(&self, _: &dyn DataItem) -> f64 {
       self.0 as f64
    }
    
    fn get_underlying_number(&self) -> f64 { self.0 as f64 }
    fn type_name(&self) -> &'static str { "MEMORY" }
    fn type_id(&self) -> TypeId { TypeId::of::<Self>() }
    fn print(&self, config: &SmartCalcConfig, _: &RefCell<Session>) -> String {

        let formated_number = format_number(self.0, config.thousand_separator.to_string(), config.decimal_seperator.to_string(), 2, true, true);
        match self.1 {
            MemoryType::Byte =>      format!("{}B",  formated_number),
            MemoryType::KiloByte =>  format!("{}KB", formated_number),
            MemoryType::MegaByte =>  format!("{}MB", formated_number),
            MemoryType::GigaByte =>  format!("{}GB", formated_number),
            MemoryType::TeraByte =>  format!("{}TB", formated_number),
            MemoryType::PetaByte =>  format!("{}PB", formated_number),
            MemoryType::ExaByte =>   format!("{}EB", formated_number),
            MemoryType::ZettaByte => format!("{}ZB", formated_number),
            MemoryType::YottaByte => format!("{}YB", formated_number)
        }
    }
    fn unary(&self, _: UnaryType) -> Arc<dyn DataItem> {
        Arc::new(Self(self.0, self.1.clone()))
    }
}

#[cfg(test)]
#[test]
fn format_result_test() {
    use alloc::string::ToString;
    use crate::executer::initialize;
    use crate::compiler::memory::MemoryItem;
    initialize();
    use crate::config::SmartCalcConfig;
    let config = SmartCalcConfig::default();
    let session = RefCell::new(Session::default());

    assert_eq!(MemoryItem(117.0, MemoryType::MegaByte).print(&config, &session), "117MB".to_string());
}