/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::any::{Any, TypeId};
use core::cell::RefCell;
use alloc::format;
use alloc::string::{ToString, String};
use alloc::sync::Arc;
use crate::app::Session;
use crate::config::SmartCalcConfig;
use crate::types::{TokenType, NumberType};
use super::percent::PercentItem;
use super::{DataItem, OperationType, UnaryType};
use crate::formatter::format_number;
use crate::tools::do_divition;

#[derive(Debug)]

pub struct NumberItem(pub f64, pub NumberType);
impl DataItem for NumberItem {
    fn as_token_type(&self) -> TokenType {
        TokenType::Number(self.0, self.1)
    }
    fn is_same<'a>(&self, other: &'a dyn Any) -> bool {
        match other.downcast_ref::<f64>() {
            Some(value) => (value - self.0).abs() < f64::EPSILON,
            None => false
        }
    }
    fn as_any(&self) -> &dyn Any { self }
    fn calculate(&self, _: &SmartCalcConfig, on_left: bool, other: &dyn DataItem, operation_type: OperationType) -> Option<Arc<dyn DataItem>> {
        let other_number  = if TypeId::of::<NumberItem>() == other.type_id() { 
            other.get_underlying_number()
            
        } else if TypeId::of::<PercentItem>() == other.type_id() { 
            other.get_number(self)
            
        } else {
            return None;
        };
        
        let (left, right) = if on_left { 
            (self.0, other_number) 
        } else { 
            (other_number, self.0 ) 
        };
        
        let result = match operation_type {
            OperationType::Add => left + right,
            OperationType::Div => do_divition(left, right),
            OperationType::Mul => left * right,
            OperationType::Sub => left - right
        };
        Some(Arc::new(NumberItem(result, self.1)))
    }
    fn get_number(&self, _: &dyn DataItem) -> f64 { self.0 }
    fn get_underlying_number(&self) -> f64 { self.0 }
    fn type_name(&self) -> &'static str { "NUMBER" }
    fn type_id(&self) -> TypeId { TypeId::of::<NumberItem>() }
    fn print(&self, config: &SmartCalcConfig, _: &RefCell<Session>) -> String {
        match self.1 {
            NumberType::Decimal     => format_number(self.0, config.thousand_separator.to_string(), config.decimal_seperator.to_string(), 2, true, true),
            NumberType::Binary      => format!("{:#b}", self.0 as i32),
            NumberType::Octal       => format!("{:#o}", self.0 as i32),
            NumberType::Hexadecimal => format!("{:#X}", self.0 as i32),
            NumberType::RAW         => format!("{}", self.0 as i32)
        }
    }
    fn unary(&self, unary: UnaryType) -> Arc<dyn DataItem> {
        match unary {
            UnaryType::Minus => Arc::new(Self(-1.0 * self.0, self.1)),
            UnaryType::Plus => Arc::new(Self(self.0, self.1))
        }
    }
}
