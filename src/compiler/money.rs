/*
 * smartcalc v1.0.3
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::any::{Any, TypeId};
use core::cell::RefCell;
use alloc::format;
use alloc::string::ToString;
use alloc::string::String;
use alloc::sync::Arc;
use core::ops::Deref;
use crate::app::Session;
use crate::config::SmartCalcConfig;
use crate::types::{CurrencyInfo, TokenType};

use super::number::NumberItem;
use super::{DataItem, OperationType, UnaryType};
use crate::formatter::format_number;
use crate::tools::do_divition;

#[derive(Debug)]

pub struct MoneyItem(pub f64, pub Arc<CurrencyInfo>);

impl MoneyItem {
    pub fn get_currency(&self) -> Arc<CurrencyInfo> {
        self.1.clone()
    }
    
    pub fn get_price(&self) -> f64 {
        self.0
    }
    
    fn convert_currency(&self, config: &SmartCalcConfig, left: &MoneyItem) -> f64 {
        let as_usd = match config.currency_rate.get(&left.get_currency()) {
            Some(l_rate) => do_divition(left.get_price(), *l_rate),
            _ => 0.0
        };
    
        match config.currency_rate.get(&self.get_currency()) {
            Some(r_rate) => as_usd * r_rate,
            _ => 0.0
        }
    }
}

impl DataItem for MoneyItem {
    fn as_token_type(&self) -> TokenType {
        TokenType::Money(self.0, self.1.clone())
    }
    fn is_same<'a>(&self, other: &'a dyn Any) -> bool {
        match other.downcast_ref::<(f64, Arc<CurrencyInfo>)>() {
            Some((l_value, l_symbol)) => (l_value - self.0).abs() < f64::EPSILON && l_symbol.deref() == self.1.deref(),
            None => false
        }
    }
    fn as_any(&self) -> &dyn Any { self }
    
    fn calculate(&self, config: &SmartCalcConfig, on_left: bool, other: &dyn DataItem, operation_type: OperationType) -> Option<Arc<dyn DataItem>> {
        /* If both item is money and current money is on left side, skip calculation */
        let (other_amount, target_curreny, is_other_money)  = match other.type_name() {
            "NUMBER" => (other.get_underlying_number(), self.1.clone(), false),
            "MONEY" => (self.convert_currency(config, other.as_any().downcast_ref::<MoneyItem>().unwrap()), self.1.clone(), true),
            "PERCENT" => (other.get_number(self), self.1.clone(), false),
            "DURATION" => (other.get_number(self), self.1.clone(), false),
            _ => return None
        };
        
        let (left, right) = if on_left { 
            (self.0, other_amount) 
        } else { 
            (other_amount, self.0 ) 
        };
        
        let result = match operation_type {
            OperationType::Add => left + right,
            OperationType::Div => {
                let div_result = do_divition(left, right);
                match is_other_money {
                    true => return Some(Arc::new(NumberItem(div_result))),
                    false => div_result
                }
            },
            OperationType::Mul => left * right,
            OperationType::Sub => left - right
        };
        Some(Arc::new(MoneyItem(result, target_curreny)))
    }
    
    fn get_number(&self, other: &dyn DataItem) -> f64 {
       if self.type_name() == other.type_name() {
           return self.0 
       }
       
       return other.get_underlying_number() * self.0
    }
    
    fn get_underlying_number(&self) -> f64 { self.0 }
    fn type_name(&self) -> &'static str { "MONEY" }
    fn type_id(&self) -> TypeId { TypeId::of::<MoneyItem>() }
    fn print(&self, config: &SmartCalcConfig, _: &RefCell<Session>) -> String {
        let currency = self.get_currency();
        let formated_price = format_number(self.get_price(), config.thousand_separator.to_string(), config.decimal_seperator.to_string(), currency.decimal_digits, false, true);
        match (currency.symbol_on_left, currency.space_between_amount_and_symbol) {
            (true, true) => format!("{} {}", currency.symbol, formated_price),
            (true, false) => format!("{}{}", currency.symbol, formated_price),
            (false, true) => format!("{} {}", formated_price, currency.symbol),
            (false, false) => format!("{}{}", formated_price, currency.symbol),
        }
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