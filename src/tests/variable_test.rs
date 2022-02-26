/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use crate::compiler::DataItem;
use crate::compiler::number::NumberItem;
use crate::config::SmartCalcConfig;
use crate::types::{BramaAstType};
use crate::compiler::money::MoneyItem;
use crate::app::SmartCalc;
use alloc::string::ToString;
use core::ops::Deref;

#[test]
fn variable_1() {
    let test_data = r"monthly rent = $1.900
monthly rent = $2.150
monthly rent / 4 people".to_string();
    let calculater = SmartCalc::default();
    let config = SmartCalcConfig::default();
    let results = calculater.execute("en".to_string(), test_data);
    assert_eq!(results.lines.len(), 3);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => match item.as_any().downcast_ref::<MoneyItem>() {
            Some(item) => {
                assert_eq!(item.get_price(), 1900.0);
                assert_eq!(item.get_currency(), config.get_currency("usd".to_string()).unwrap());
            },
            _ => assert!(false)
        },
        _ => assert!(false)
    };
    match &*results.lines[1].as_ref().unwrap().result.as_ref().unwrap().ast {
        BramaAstType::Item(item) => match item.as_any().downcast_ref::<MoneyItem>() {
            Some(item) => {
                assert_eq!(item.get_price(), 2150.0);
                assert_eq!(item.get_currency(), config.get_currency("usd".to_string()).unwrap());
            },
            _ => assert!(false)
        },
        _ => assert!(false)
    };
    match &*results.lines[2].as_ref().unwrap().result.as_ref().unwrap().ast {
        BramaAstType::Item(item) => match item.as_any().downcast_ref::<MoneyItem>() {
            Some(item) => {
                assert_eq!(item.get_price(), 537.5);
                assert_eq!(item.get_currency(), config.get_currency("usd".to_string()).unwrap());
            },
            _ => assert!(false)
        },
        _ => assert!(false)
    };
}

#[test]
fn variable_1_1() {
    let test_data = r"monthly rent = $1.900
monthly rent = $2.150
monthly rent / $4".to_string();
    let calculater = SmartCalc::default();
    let config = SmartCalcConfig::default();
    let results = calculater.execute("en".to_string(), test_data);
    assert_eq!(results.lines.len(), 3);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => match item.as_any().downcast_ref::<MoneyItem>() {
            Some(item) => {
                assert_eq!(item.get_price(), 1900.0);
                assert_eq!(item.get_currency(), config.get_currency("usd".to_string()).unwrap());
            },
            _ => assert!(false)
        },
        _ => assert!(false)
    };
    match &*results.lines[1].as_ref().unwrap().result.as_ref().unwrap().ast {
        BramaAstType::Item(item) => match item.as_any().downcast_ref::<MoneyItem>() {
            Some(item) => {
                assert_eq!(item.get_price(), 2150.0);
                assert_eq!(item.get_currency(), config.get_currency("usd".to_string()).unwrap());
            },
            _ => assert!(false)
        },
        _ => assert!(false)
    };
    match &*results.lines[2].as_ref().unwrap().result.as_ref().unwrap().ast {
        BramaAstType::Item(item) => match item.as_any().downcast_ref::<NumberItem>() {
            Some(item) => {
                assert_eq!(item.get_underlying_number(), 537.5);
            },
            _ => assert!(false)
        },
        _ => assert!(false)
    };
}

#[test]
fn variable_2() {
    let test_data = r"year = 2021
my age = year - 1985".to_string();
    let calculater = SmartCalc::default();
    let results = calculater.execute("en".to_string(), test_data);
    assert_eq!(results.lines.len(), 2);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => match item.as_any().downcast_ref::<NumberItem>() {
            Some(item) => assert_eq!(item.get_underlying_number(), 2021.0),
            _ => assert!(false)
        },
        _ => assert!(false)
    };
    match &*results.lines[1].as_ref().unwrap().result.as_ref().unwrap().ast {
        BramaAstType::Item(item) => match item.as_any().downcast_ref::<NumberItem>() {
            Some(item) => assert_eq!(item.get_underlying_number(), 36.0),
            _ => assert!(false)
        },
        _ => assert!(false)
    };
}
