#![no_std]
extern crate libsmartcalc;
extern crate alloc;

#[cfg(test)]
mod tests {
    use libsmartcalc::types::{BramaAstType};
    use libsmartcalc::executer::{execute, initialize};
    use chrono::NaiveTime;
    use alloc::string::ToString;

    #[test]
    fn variable_1() {
        let test_data = r"monthly rent = $1.900
monthly rent = $2.150
monthly rent / 4 people".to_string();
        initialize();
        let results = execute(&test_data, &"en".to_string());
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
}