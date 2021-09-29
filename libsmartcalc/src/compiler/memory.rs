use core::any::{Any, TypeId};
use core::cell::RefCell;
use alloc::string::String;
use alloc::sync::Arc;
use crate::app::Session;
use crate::config::SmartCalcConfig;
use crate::types::{MemoryType, TokenType};
use super::number::NumberItem;
use super::{DataItem, OperationType, UnaryType};
use core::write;
use alloc::fmt::Write;
use alloc::format;

#[derive(Debug)]
pub struct MemoryItem(pub u128, pub MemoryType);

const BYTE: u128 = 1;
const KILO_BYTE: u128 = BYTE * 1024;
const MEGA_BYTE: u128 = KILO_BYTE * 1024;
const GIGA_BYTE: u128 = MEGA_BYTE * 1024;
const TERA_BYTE: u128 = GIGA_BYTE * 1024;
const PETA_BYTE: u128 = TERA_BYTE * 1024;
const EXA_BYTE: u128 = PETA_BYTE * 1024;
const ZETTA_BYTE: u128 = EXA_BYTE * 1024;
const YOTTA_BYTE: u128 = ZETTA_BYTE * 1024;

const MEMORY_ITEMS: &[(u128, &'static str); 9] = &[
    (YOTTA_BYTE, "YB"),
    (ZETTA_BYTE, "ZB"),
    (EXA_BYTE, "EB"),
    (PETA_BYTE, "PB"),
    (TERA_BYTE, "TB"),
    (GIGA_BYTE, "GB"),
    (MEGA_BYTE, "MB"),
    (KILO_BYTE, "KB"),
    (BYTE, "B")
];

impl MemoryItem {
    pub fn get_memory(&self) -> u128 {
        self.0.clone()
    }
}

impl DataItem for MemoryItem {
    fn as_token_type(&self) -> TokenType {
        TokenType::Memory(self.0.clone(), self.1.clone())
    }
    fn is_same<'a>(&self, other: &'a dyn Any) -> bool {
        match other.downcast_ref::<Self>() {
            Some(l_value) => l_value.0 == self.0 && l_value.1 == self.1,
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
            Some(result) => Some(Arc::new(MemoryItem(result, MemoryType::MegaByte))),
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
        for (memory_size, memory_name) in MEMORY_ITEMS {
            if self.0 > *memory_size && self.0 / memory_size > 0 {
                let calculated = self.0 / memory_size;
                write!(buffer, "{}{} ", calculated, memory_name).unwrap();
                
                //let result = self.0 - (calculated * *memory_size);
                //let kalan = result / memory_size;
                //write!(buffer, "Kalan : {} ", kalan).unwrap();
                break;
            }
        }


        match self.1 {
            MemoryType::Byte => format!("{}B", self.0),
            MemoryType::KiloByte => format!("{}KB", self.0),
            MemoryType::MegaByte => format!("{}MB", self.0),
            MemoryType::GigaByte => format!("{}GB", self.0),
            MemoryType::TeraByte => format!("{}TB", self.0),
            MemoryType::PetaByte => format!("{}PB", self.0),
            MemoryType::ExaByte => format!("{}EB", self.0),
            MemoryType::ZettaByte => format!("{}ZB", self.0),
            MemoryType::YottaByte => format!("{}YB", self.0)
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

    assert_eq!(MemoryItem(117, MemoryType::MegaByte).print(&config, &session), "117MB".to_string());
}