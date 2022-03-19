/*
 * smartcalc v1.0.6
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

extern crate alloc;

use alloc::vec::Vec;
use crate::smartcalc::SmartCalc;
use alloc::string::{String, ToString};

fn execute(test_data: String, decimal_seperator: String, thousand_separator: String, timezone: String) {
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

    let mut calculater = SmartCalc::default();
    calculater.set_decimal_seperator(decimal_seperator);
    calculater.set_thousand_separator(thousand_separator);
    calculater.set_timezone(timezone).unwrap();
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
22250mb - 250,1mb               | 21.999,90MB
8 gb * 10                       | 80GB
1024mb                          | 1.024MB
1024mb - 24 mb                  | 1.000MB
1024mb - (1024kb * 24)          | 1.000MB
1024mb + (1024kb * 24)          | 1.048MB
1000mb / 10MB                   | 100
1 gb to mb                      | 1.024MB
1 gb to byte                    | 1.073.741.824byte
x = 2                           | 2
h = 2 * 2                       | 4
10 $                            | $10,00
"#.to_string(), ",".to_string(), ".".to_string(), "UTC".to_string());        
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
22250mb - 250.1mb               | 21,999.90MB
8 gb * 10                       | 80GB
1024mb                          | 1,024MB
1024mb - 24 mb                  | 1,000MB
1024mb - (1024kb * 24)          | 1,000MB
1024mb + (1024kb * 24)          | 1,048MB
1000mb / 10MB                   | 100
1 gb to mb                      | 1,024MB
1 gb to byte                    | 1,073,741,824byte
x = 2                           | 2
h = 2 * 2                       | 4
10 $                            | $10.00
"#.to_string(), ".".to_string(), ",".to_string(), "UTC".to_string());        
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
22250mb - 250.1mb               | 21999.90MB
8 gb * 10                       | 80GB
1024mb                          | 1024MB
1024mb - 24 mb                  | 1000MB
1024mb - (1024kb * 24)          | 1000MB
1024mb + (1024kb * 24)          | 1048MB
1000mb / 10MB                   | 100
1 gb to mb                      | 1024MB
1 gb to byte                    | 1073741824byte
x = 2                           | 2
h = 2 * 2                       | 4
10 $                            | $10.00
"#.to_string(), ".".to_string(), "".to_string(), "UTC".to_string());        
}

#[test]
fn execute_4() {
    execute(r#"
15:00 EST to CET     | 21:00:00 CET
15:00 CET to EST     | 09:00:00 EST
date = 15:00 EST     | 15:00:00 EST
date to CET          | 21:00:00 CET
22:00                | 22:00:00 CET
15:00 GMT+1          | 15:00:00 GMT+1
15:00 GMT1           | 15:00:00 GMT1
15:00 GMT-1:30       | 15:00:00 GMT-1:30
15:00 GMT+3:30 to CET| 12:30:00 CET
"#.to_string(), ".".to_string(), "".to_string(), "CET".to_string());        
}


#[test]
fn execute_5() {
    execute(r#"
100.0 to binary      | 0b1100100
100.0 to binary      | 0b1100100
100.0 to octal       | 0o144
100.0 to octal       | 0o144
100.0 to hexadecimal | 0x64
a = 0o12             | 0o12
a to hex             | 0xA
a to decimal         | 10
0o12 to decimal      | 10
100 to hex           | 0x64
6% off 40            | 37.60
6% of 40             | 2.40
6% on 40             | 42.40
"#.to_string(), ".".to_string(), ",".to_string(), "CET".to_string());        
}


#[test]
fn date_tests() {
    execute(r#"
1646401747 to date      | 4 March 14:49:07 CET
1 oct 2022 as unix      | 1664582400
a = 1664582400 to date  | 1 October 01:00:00 CET
a to UTC                | 1 October 00:00:00 UTC
"#.to_string(), ".".to_string(), ",".to_string(), "CET".to_string());        
}


#[test]
fn variable_usage_test() {
    execute(r#"
test 1 = 123   | 123
test 2 = 2usd  | $2,00

house price = 250k usd  | $250.000,00
salary = 10k usd        |  $10.000,00

home expense = 1k usd   |   $1.000,00
child expense = 1k usd  |   $1.000,00
hosue rent = 1,5k usd   |   $1.500,00
other expense = 2k usd  |   $2.000,00

total expenses =  home expense + child expense + hosue rent + other expense  |  $5.500,00 
saving = salary - total expenses                                             |  $4.500,00 
down payment = house price of %15                                            | $37.500,00 
total month = down payment / saving                                          |       8,33
"#.to_string(), ",".to_string(), ".".to_string(), "CET".to_string());        
}

#[test]
fn weight_height_tests() {
    execute(r#"
1 m to mm                | 1,000 Millimeter
1 yard to inch           |    36 Inch
100 yard to mile         |     0.06 Mile
1 Stone to kg            |     6.35 Kilogram
10 pound to ounce        |   160 Ounce
"#.to_string(), ".".to_string(), ",".to_string(), "CET".to_string());        
}
