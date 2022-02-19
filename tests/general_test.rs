#![no_std]
extern crate libsmartcalc;
extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use libsmartcalc::executer::{initialize};
    use libsmartcalc::app::SmartCalc;
    use alloc::string::{String, ToString};
    
    fn execute(test_data: String, decimal_seperator: String, thousand_separator: String) {
        let mut query = String::new();
        let mut expected_results = Vec::new();
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
        let mut calculater = SmartCalc::default();
        calculater.config.decimal_seperator = decimal_seperator;
        calculater.config.thousand_separator = thousand_separator;
        let results = calculater.execute("en".to_string(), query);
        
        for (index, result_line) in results.lines.iter().enumerate() {
            match result_line {
                Some(result) => assert_eq!(result.result.as_ref().unwrap().output, expected_results[index].unwrap()),
                None => assert!(expected_results[index].is_none())
            }
        };
    }

    #[test]
    fn execute_1() {
        execute(r#"
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
x = 2                           | 2
h = 2 * 2                       | 4
10 $                            | $10,00
"#.to_string(), ",".to_string(), ".".to_string());        
    }

    #[test]
    fn execute_2() {
        execute(r#"
1024                            | 1,024
200 * 10                        | 2,000
100mb                           | 100MB
100 mb                          | 100MB
100 MegaByte                    | 100MB
100 MegaBytes                   | 100MB
100 Mega Bytes                  | 100MB
22250mb - 250.1mb               | 21,999.90MB
8 gb * 10                       | 80GB
1024mb                          | 1,024MB
1024mb - 24 mb                  | 1,000MB
1024mb - (1024kb * 24)          | 1,000MB
1024mb + (1024kb * 24)          | 1,048MB
1000mb / 10MB                   | 100MB
1 gb to mb                      | 1,024MB
1 gb to byte                    | 1,073,741,824B
x = 2                           | 2
h = 2 * 2                       | 4
10 $                            | $10.00
"#.to_string(), ".".to_string(), ",".to_string());        
    }

    #[test]
    fn execute_3() {
        execute(r#"
1024                            | 1024
200 * 10                        | 2000
100mb                           | 100MB
100 mb                          | 100MB
100 MegaByte                    | 100MB
100 MegaBytes                   | 100MB
100 Mega Bytes                  | 100MB
22250mb - 250.1mb               | 21999.90MB
8 gb * 10                       | 80GB
1024mb                          | 1024MB
1024mb - 24 mb                  | 1000MB
1024mb - (1024kb * 24)          | 1000MB
1024mb + (1024kb * 24)          | 1048MB
1000mb / 10MB                   | 100MB
1 gb to mb                      | 1024MB
1 gb to byte                    | 1073741824B
x = 2                           | 2
h = 2 * 2                       | 4
10 $                            | $10.00
"#.to_string(), ".".to_string(), "".to_string());        
    }
}
