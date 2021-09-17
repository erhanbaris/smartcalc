#![no_std]
extern crate libsmartcalc;
extern crate alloc;

#[cfg(test)]
mod tests {
    use libsmartcalc::compiler::DataItem;
    use libsmartcalc::compiler::number::NumberItem;
    use libsmartcalc::config::SmartCalcConfig;
    use libsmartcalc::types::{BramaAstType};
    use libsmartcalc::executer::initialize;
    use libsmartcalc::compiler::money::MoneyItem;
    use libsmartcalc::app::SmartCalc;
    use alloc::string::ToString;
    use core::ops::Deref;

    #[test]
    fn variable_1() {
        let test_data = r"monthly rent = $1.900
monthly rent = $2.150
monthly rent / 4 people".to_string();
        initialize();
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
    fn variable_2() {
        let test_data = r"year = 2021
my age = year - 1985".to_string();
        initialize();
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
}