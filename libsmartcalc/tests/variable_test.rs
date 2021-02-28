#![no_std]
extern crate libsmartcalc;
extern crate alloc;

#[cfg(test)]
mod tests {
    use libsmartcalc::types::{BramaAstType};
    use libsmartcalc::executer::{execute, initialize};
    use alloc::string::ToString;

    #[test]
    fn variable_1() {
        let test_data = r"monthly rent = $1.900
monthly rent = $2.150
monthly rent / 4 people".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);
        assert_eq!(results.len(), 3);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Money(price, currency) => {
                assert_eq!(*price, 1900.0);
                assert_eq!(*currency, "usd".to_string());
            },
            _ => assert!(false)
        };
        match &*results[1].as_ref().unwrap().1 {
            BramaAstType::Money(price, currency) => {
                assert_eq!(*price, 2150.0);
                assert_eq!(*currency, "usd".to_string());
            },
            _ => assert!(false)
        };
        match &*results[2].as_ref().unwrap().1 {
            BramaAstType::Money(price, currency) => {
                assert_eq!(*price, 537.5);
                assert_eq!(*currency, "usd".to_string());
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn variable_2() {
        let test_data = r"year = 2021
my age = year - 1985".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);
        assert_eq!(results.len(), 2);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Number(number) => {
                assert_eq!(*number, 2021.0);
            },
            _ => assert!(false)
        };
        match &*results[1].as_ref().unwrap().1 {
            BramaAstType::Number(number) => {
                assert_eq!(*number, 36.0);
            },
            _ => assert!(false)
        };
    }
}