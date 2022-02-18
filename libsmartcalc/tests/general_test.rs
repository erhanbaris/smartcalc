#![no_std]
extern crate libsmartcalc;
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use libsmartcalc::executer::{initialize};
    use libsmartcalc::app::SmartCalc;
    use alloc::string::{String, ToString};

    #[test]
    fn execute_1() {
        let mut query = String::new();
        let mut expected_results = Vec::new();
        let test_data = r#"
1024                            | 1.024
200 * 10                        | 2.000
100mb                           | 100MB
100 mb                          | 100MB
100 MegaByte                    | 100MB
100 MegaBytes                   | 100MB
100 Mega Bytes                  | 100MB
22250mb - 250,1mb               | 21.999,90MB
8 gb * 10                       | 80GB
1024mb                          | 1.024MB
1024mb - 24 mb                  | 1.000MB
1024mb - (1024kb * 24)          | 1.000MB
1024mb + (1024kb * 24)          | 1.048MB
1000mb / 10MB                   | 100MB
1 gb to mb                      | 1.024MB
1 gb to byte                    | 1.073.741.824B
"#.to_string();

        for line in test_data.lines() {
            let splited_line = line.split("|").collect::<Vec<&str>>();

            if splited_line.len() > 1 {
                expected_results.push(Some(splited_line[1].trim()));
                query.push_str(splited_line[0].trim());
            }
            else {
                expected_results.push(None);
                query.push_str("");
            }
            query.push_str("\r\n");
        }
        expected_results.push(None);

        initialize();
        let calculater = SmartCalc::default();
        let results = calculater.execute("en".to_string(), query);
        
        for (index, result_line) in results.lines.iter().enumerate() {
            match result_line {
                Some(result) => assert_eq!(result.result.as_ref().unwrap().output, expected_results[index].unwrap()),
                None => assert!(expected_results[index].is_none())
            }
        };
    }
}
