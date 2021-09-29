#![no_std]
extern crate libsmartcalc;
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use libsmartcalc::types::{BramaAstType};
    use libsmartcalc::executer::{initialize};
    use libsmartcalc::app::SmartCalc;
    use alloc::string::{String, ToString};
    use core::ops::Deref;

    #[test]
    fn execute_1() {
        let mut query = String::new();
        let mut expected_results = Vec::new();
        let test_data = r#"
1024     | 1024
200 * 10 | 2000
        "#.to_string();

        for line in test_data.lines() {
            let splited_line = line.split("|").collect::<Vec<&str>>();

            if splited_line.len() > 1 {
                expected_results.push(splited_line[1]);
                query.push_str(splited_line[0]);
            }
            else {
                expected_results.push("");
                query.push_str("");
            }
            query.push_str("\r\n");
        }

        initialize();
        let calculater = SmartCalc::default();
        let results = calculater.execute("en".to_string(), query);
        match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
            BramaAstType::Item(item) => assert_eq!(item.get_underlying_number(), 171.6),
            _ => assert!(false)
        };
    }
}
