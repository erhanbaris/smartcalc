use core::any::{Any, TypeId};
use core::cell::RefCell;
use alloc::string::ToString;
use alloc::string::String;
use alloc::sync::Arc;
use crate::app::Session;
use crate::config::SmartCalcConfig;
use crate::types::TokenType;
use super::number::NumberItem;
use super::{DataItem, OperationType, UnaryType};
use core::write;
use alloc::fmt::Write;

#[derive(Debug)]
pub struct MemoryItem(pub u128);

const BIT: u128 = 1;
const BYTE: u128 = BIT * 8;
const KILO_BYTE: u128 = BYTE * 1024;
const MEGA_BYTE: u128 = KILO_BYTE * 1024;
const GIGA_BYTE: u128 = MEGA_BYTE * 1024;
const TERA_BYTE: u128 = GIGA_BYTE * 1024;
const PETA_BYTE: u128 = TERA_BYTE * 1024;
const EXA_BYTE: u128 = PETA_BYTE * 1024;
const ZETTA_BYTE: u128 = EXA_BYTE * 1024;
const YOTTA_BYTE: u128 = ZETTA_BYTE * 1024;

impl MemoryItem {
    pub fn get_memory(&self) -> u128 {
        self.0.clone()
    }

    fn calculate_number(&self, buffer: &mut String, number: u128, target: u128, text: &str) -> u128 {
        if self.0 % target != 0 {
            let calculated = self.0 % target;
            write!(buffer, "{} {} ", calculated, text).unwrap();
            return number - (calculated * target);
        }
        number
    }
}

impl DataItem for MemoryItem {
    fn as_token_type(&self) -> TokenType {
        TokenType::Month(0)
    }
    fn is_same<'a>(&self, other: &'a dyn Any) -> bool {
        match other.downcast_ref::<Self>() {
            Some(l_value) => l_value.0 == self.0,
            None => false
        }
    }
    fn as_any(&self) -> &dyn Any { self }
    
    fn calculate(&self, _: &SmartCalcConfig, on_left: bool, other: &dyn DataItem, operation_type: OperationType) -> Option<Arc<dyn DataItem>> {
        /* If both item is money and current money is on left side, skip calculation */
        if TypeId::of::<Self>() != other.type_id() && on_left {
            return None;
        }

        let number = match other.type_name() {
            "MEMORY" => other.as_any().downcast_ref::<Self>().unwrap().get_memory(),
            "NUMBER" => other.as_any().downcast_ref::<NumberItem>().unwrap().get_underlying_number() as u128,
            _ => return None
        };

        let operation_result = match operation_type {
            OperationType::Add => self.0.checked_add(number),
            OperationType::Sub => self.0.checked_sub(number),
            OperationType::Div => self.0.checked_div(number),
            OperationType::Mul => self.0.checked_mul(number)
        };

        match operation_result {
            Some(result) => Some(Arc::new(MemoryItem(result))),
            None => None
        }
    }
    
    fn get_number(&self, _: &dyn DataItem) -> f64 {
       self.0 as f64
    }
    
    fn get_underlying_number(&self) -> f64 { self.0 as f64 }
    fn type_name(&self) -> &'static str { "MEMORY" }
    fn type_id(&self) -> TypeId { TypeId::of::<Self>() }
    fn print(&self, _: &SmartCalcConfig, _: &RefCell<Session>) -> String {

        let mut buffer = String::new();
        let mut number = self.0;

        number = self.calculate_number(&mut buffer, number, YOTTA_BYTE, "YottaByte");
        number = self.calculate_number(&mut buffer, number, ZETTA_BYTE, "ZettaByte");
        number = self.calculate_number(&mut buffer, number, EXA_BYTE, "ExaByte");
        number = self.calculate_number(&mut buffer, number, PETA_BYTE, "PetaByte");
        number = self.calculate_number(&mut buffer, number, TERA_BYTE, "TeraByte");
        number = self.calculate_number(&mut buffer, number, GIGA_BYTE, "GigaByte");
        number = self.calculate_number(&mut buffer, number, MEGA_BYTE, "MegaByte");
        number = self.calculate_number(&mut buffer, number, KILO_BYTE, "KiloByte");
        number = self.calculate_number(&mut buffer, number, BYTE, "Byte");
        self.calculate_number(&mut buffer, number, BIT, "Bit");
        
    
        buffer.to_string()
    }
    fn unary(&self, _: UnaryType) -> Arc<dyn DataItem> {
        Arc::new(Self(self.0))
    }
}

#[cfg(test)]
#[test]
fn format_result_test() {
    use crate::executer::initialize;
    use crate::compiler::memory::MemoryItem;
    initialize();
    use crate::config::SmartCalcConfig;
    let config = SmartCalcConfig::default();
    let session = RefCell::new(Session::default());

    assert_eq!(MemoryItem(123456789).print(&config, &session), "$0.00".to_string());
}