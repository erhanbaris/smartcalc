/*
 * smartcalc v1.0.1
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use crate::SmartCalc;
use crate::compiler::AsNaiveTime;
use crate::compiler::date::DateItem;
use crate::compiler::duration::DurationItem;
use crate::compiler::memory::MemoryItem;
use crate::compiler::time::TimeItem;
use crate::config::SmartCalcConfig;
use crate::compiler::money::MoneyItem;
use crate::types::{BramaAstType, MemoryType};
use chrono::{Duration, Local, NaiveDate, NaiveTime};
use chrono::{Datelike};
use alloc::string::ToString;
use core::ops::Deref;

#[test]
fn execute_1() {
    let test_data = "120 + 30% + 10%";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => assert_eq!(item.get_underlying_number(), 171.6),
        _ => assert!(false)
    };
}

#[test]
fn execute_2() {
    let test_data = r"
erhan barış = 120
erhan barış + 120";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);
    assert_eq!(results.lines.len(), 3);
    match results.lines[1].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(number) => assert_eq!(number.get_underlying_number(), 120.0),
        _ => assert!(false)
    };
    match results.lines[2].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(number) => assert_eq!(number.get_underlying_number(), 240.0),
        _ => assert!(false)
    };
}

#[test]
fn execute_3() {
    let test_data = r"
erhan barış = 120
aysel barış = 200
toplam = erhan barış + aysel barış";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);
    assert_eq!(results.lines.len(), 4);
    match results.lines[1].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(number) => assert_eq!(number.get_underlying_number(), 120.0),
        _ => assert!(false)
    };
    match results.lines[2].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(number) => assert_eq!(number.get_underlying_number(), 200.0),
        _ => assert!(false)
    };
    match results.lines[3].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(number) => assert_eq!(number.get_underlying_number(), 320.0),
        _ => assert!(false)
    };
}

#[test]
fn execute_4() {
    let test_data = r"erhan barış = 120
aysel barış = 200
toplam = erhan barış + test aysel barış";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 3);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(number) => assert_eq!(number.get_underlying_number(), 120.0),
        _ => assert!(false)
    };
    match results.lines[1].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(number) => assert_eq!(number.get_underlying_number(), 200.0),
        _ => assert!(false)
    };
    match results.lines[2].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(number) => assert_eq!(number.get_underlying_number(), 320.0),
        _ => assert!(false)
    };
}

#[test]
fn execute_5() {
    let test_data = r"100 200";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(number) => assert_eq!(number.get_underlying_number(), 300.0),
        _ => assert!(false)
    };
}

#[test]
fn execute_6() {
    let test_data = r"aysel = 10324
erhan = 5890
nakit = erhan + aysel
erhan maaş = 25965,25
aysel maaş = 3500
sigorta geri ödemesi = 8600
toplam nakit = (nakit + erhan maaş) + (aysel maaş + sigorta geri ödemesi)";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 7);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(number) => assert_eq!(number.get_underlying_number(), 10324.0),
        _ => assert!(false)
    };
    match results.lines[1].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(number) => assert_eq!(number.get_underlying_number(), 5890.0),
        _ => assert!(false)
    };
    match results.lines[2].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(number) => assert_eq!(number.get_underlying_number(), 16214.0),
        _ => assert!(false)
    };
    match results.lines[3].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(number) => assert_eq!(number.get_underlying_number(), 25965.25),
        _ => assert!(false)
    };
    match results.lines[4].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(number) => assert_eq!(number.get_underlying_number(), 3500.0),
        _ => assert!(false)
    };
    match results.lines[5].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(number) => assert_eq!(number.get_underlying_number(), 8600.0),
        _ => assert!(false)
    };
    match results.lines[6].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(number) => assert_eq!(number.get_underlying_number(), 54279.25),
        _ => assert!(false)
    };
}

#[test]
fn execute_7() {
    let test_data = r"tarih = 11:30
tarih add 12 hour";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 2);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => assert_eq!(item.as_any().downcast_ref::<TimeItem>().unwrap().as_naive_time(), NaiveTime::from_hms(11, 30, 0)),
        _ => assert!(false)
    };
    match results.lines[1].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => assert_eq!(item.as_any().downcast_ref::<TimeItem>().unwrap().as_naive_time(), NaiveTime::from_hms(23, 30, 0)),
        _ => assert!(false)
    };
}

#[test]
fn execute_8() {
    let test_data = r"tarih = 11:30
tarih add -1 hour";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 2);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => assert_eq!(item.as_any().downcast_ref::<TimeItem>().unwrap().as_naive_time(), NaiveTime::from_hms(11, 30, 0)),
        _ => assert!(false)
    };
    match results.lines[1].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => assert_eq!(item.as_any().downcast_ref::<TimeItem>().unwrap().as_naive_time(), NaiveTime::from_hms(10, 30, 0)),
        _ => assert!(false)
    };
}

#[test]
fn execute_9() {
    let test_data = r"2k
3M
4G
5T
6P
7Z
8Y";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 7);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(num) => assert_eq!(num.get_underlying_number(), 2_000.0),
        _ => assert!(false)
    };
    match results.lines[1].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(num) => assert_eq!(num.get_underlying_number(), 3_000_000.0),
        _ => assert!(false)
    };
    match results.lines[2].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(num) => assert_eq!(num.get_underlying_number(), 4_000_000_000.0),
        _ => assert!(false)
    };
    match results.lines[3].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(num) => assert_eq!(num.get_underlying_number(), 5_000_000_000_000.0),
        _ => assert!(false)
    };
    match results.lines[4].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(num) => assert_eq!(num.get_underlying_number(), 6_000_000_000_000_000.0),
        _ => assert!(false)
    };
    match results.lines[5].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(num) => assert_eq!(num.get_underlying_number(), 7_000_000_000_000_000_000.0),
        _ => assert!(false)
    };
    match results.lines[6].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(num) => assert_eq!(num.get_underlying_number(), 8_000_000_000_000_000_000_000.0),
        _ => assert!(false)
    };
}


#[test]
fn execute_10() {
    let test_data = r"8 / (45 - 20%)";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(number) => assert_eq!(number.get_underlying_number(), 0.2222222222222222),
        _ => assert!(false)
    };
}

#[test]
fn execute_11() {
    let test_data = r"tarih = 11:30
tarih add 1 hour 1 minute 30 second";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 2);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => assert_eq!(item.as_any().downcast_ref::<TimeItem>().unwrap().as_naive_time(), NaiveTime::from_hms(11, 30, 0)),
        _ => assert!(false)
    };
    match results.lines[1].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => assert_eq!(item.as_any().downcast_ref::<TimeItem>().unwrap().as_naive_time(), NaiveTime::from_hms(12, 31, 30)),
        _ => assert!(false)
    };
}


#[test]
fn execute_12() {
    let test_data = r"5 hour 21 minute 55 second";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => assert_eq!(item.as_any().downcast_ref::<DurationItem>().unwrap().get_duration(), Duration::seconds(19315)),
        _ => assert!(false)
    };
}

#[test]
fn execute_13() {
    let test_data = r"$25/hour * 14 hours of work";
    let calculater = SmartCalc::default();
    let config = SmartCalcConfig::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => match item.as_any().downcast_ref::<MoneyItem>() {
            Some(item) => {
                assert_eq!(item.get_price(), 350.0);
                assert_eq!(item.get_currency(), config.get_currency("usd".to_string()).unwrap());
            },
            _ => assert!(false)
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_14() {
    let test_data = r"100 minutes 1 seconds";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.as_any().downcast_ref::<DurationItem>().unwrap().get_duration(), Duration::seconds(6001));
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_15() {
    let test_data = r"11:40  - 10 minute";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.as_any().downcast_ref::<TimeItem>().unwrap().as_naive_time(), NaiveTime::from_hms(11, 30, 00));
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_16() {
    let test_data = r"11:40  + 1 hour 1 second";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.as_any().downcast_ref::<TimeItem>().unwrap().as_naive_time(), NaiveTime::from_hms(12, 40, 01));
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_17() {
    let test_data = r"3:35 am + 7 hours 15 minutes";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.as_any().downcast_ref::<TimeItem>().unwrap().as_naive_time(), NaiveTime::from_hms(10, 50, 0));
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_18() {
    let test_data = r"10 June + 3 weeks";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.as_any().downcast_ref::<DateItem>().unwrap().get_date(), NaiveDate::from_ymd(Local::now().date().year(), 7, 1));
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_19() {
    let test_data = r"April 1, 2019 - 3 months 5 days";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.as_any().downcast_ref::<DateItem>().unwrap().get_date(), NaiveDate::from_ymd(2018, 12, 27));
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_20() {
    let test_data = r"Feb 1, 2019 + 1 months";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.as_any().downcast_ref::<DateItem>().unwrap().get_date(), NaiveDate::from_ymd(2019, 3, 1));
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_21() {
    let test_data = r"jan 28, 2019 - 14 months";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.as_any().downcast_ref::<DateItem>().unwrap().get_date(), NaiveDate::from_ymd(2018, 11, 28));
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_22() {
    let test_data = r"jan 28, 2019 - 14 months 10 days";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.as_any().downcast_ref::<DateItem>().unwrap().get_date(), NaiveDate::from_ymd(2018, 11, 18));
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_23() {
    let test_data = r"jan 28, 2019 - 14 months 33 days";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.as_any().downcast_ref::<DateItem>().unwrap().get_date(), NaiveDate::from_ymd(2018, 10, 25));
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_24() {
    let test_data = r"12/02/1988 + 32 years ";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.as_any().downcast_ref::<DateItem>().unwrap().get_date(), NaiveDate::from_ymd(2020, 02, 12));
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_25() {
    let test_data = r"12/02/2020 - 32 years ";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.as_any().downcast_ref::<DateItem>().unwrap().get_date(), NaiveDate::from_ymd(1988, 02, 12));
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_26() {
    let test_data = r"12/02/2020 - 11680 days";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.as_any().downcast_ref::<DateItem>().unwrap().get_date(), NaiveDate::from_ymd(1988, 02, 12));
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_27() {
    let test_data = r"1/1/2000 to 3/3/2021";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.as_any().downcast_ref::<DurationItem>().unwrap().get_duration(), Duration::days(7732));
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_28() {
    let test_data = r"3/3/2021 to 1/1/2000";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.as_any().downcast_ref::<DurationItem>().unwrap().get_duration(), Duration::days(7732));
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_29() {
    let test_data = r"today + 3 weeks";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.as_any().downcast_ref::<DateItem>().unwrap().get_date(), Local::today().naive_local() + Duration::weeks(3));
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_30() {
    let test_data = r"yesterday + 3 weeks";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.as_any().downcast_ref::<DateItem>().unwrap().get_date(), (Local::today().naive_local() + Duration::weeks(3)) -  Duration::days(1));
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_31() {
    let test_data = r"tomorrow + 3 weeks";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.as_any().downcast_ref::<DateItem>().unwrap().get_date(), Local::today().naive_local() + Duration::weeks(3) +  Duration::days(1));
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_32() {
    let test_data = r"(4 * 2,5)";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);

    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(number) => {
            assert_eq!(number.get_underlying_number(), 10.0);
        },
        _ => assert!(false)
    };
}

#[test]
fn execute_33() {
    let test_data = r"1024mb + (1024kb * 24)";
    let calculater = SmartCalc::default();
    let results = calculater.execute("en", test_data);
    
    assert_eq!(results.lines.len(), 1);
    match results.lines[0].as_ref().unwrap().result.as_ref().unwrap().ast.deref() {
        BramaAstType::Item(item) => {
            assert_eq!(item.get_underlying_number(), 1048.0);
            match item.as_any().downcast_ref::<MemoryItem>() {
                Some(memory_item) => {
                    assert_eq!(memory_item.get_memory(), 1048.0);
                    assert_eq!(memory_item.get_memory_type(), MemoryType::MegaByte);
                },
                _ => assert!(false)
            };
        },
        _ => assert!(false)
    };
}
