#![no_std]
extern crate libsmartcalc;
extern crate alloc;

#[cfg(test)]
mod tests {
    use libsmartcalc::types::{BramaAstType};
    use libsmartcalc::executer::{execute, initialize};
    use chrono::{Duration, Local, NaiveDate, NaiveTime};
    use chrono::{Datelike};
    use alloc::string::ToString;

    #[test]
    fn execute_1() {
        let test_data = "120 + 30% + 10%".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Number(number) => assert_eq!(*number, 171.6),
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_2() {
        let test_data = r"
erhan barış = 120
erhan barış + 120".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);
        assert_eq!(results.len(), 3);
        match &*results[1].as_ref().unwrap().1 {
            BramaAstType::Number(number) => assert_eq!(*number, 120.0),
            _ => assert!(false)
        };
        match &*results[2].as_ref().unwrap().1 {
            BramaAstType::Number(number) => assert_eq!(*number, 240.0),
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_3() {
        let test_data = r"
erhan barış = 120
aysel barış = 200
toplam = erhan barış + aysel barış".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);
        assert_eq!(results.len(), 4);
        match &*results[1].as_ref().unwrap().1 {
            BramaAstType::Number(number) => assert_eq!(*number, 120.0),
            _ => assert!(false)
        };
        match &*results[2].as_ref().unwrap().1 {
            BramaAstType::Number(number) => assert_eq!(*number, 200.0),
            _ => assert!(false)
        };
        match &*results[3].as_ref().unwrap().1 {
            BramaAstType::Number(number) => assert_eq!(*number, 320.0),
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_4() {
        let test_data = r"erhan barış = 120
aysel barış = 200
toplam = erhan barış + test aysel barış".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 3);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Number(number) => assert_eq!(*number, 120.0),
            _ => assert!(false)
        };
        match &*results[1].as_ref().unwrap().1 {
            BramaAstType::Number(number) => assert_eq!(*number, 200.0),
            _ => assert!(false)
        };
        match &*results[2].as_ref().unwrap().1 {
            BramaAstType::Number(number) => assert_eq!(*number, 320.0),
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_5() {
        let test_data = r"100 200".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Number(number) => assert_eq!(*number, 300.0),
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
toplam nakit = nakit + erhan maaş + aysel maaş + sigorta geri ödemesi".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 7);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Number(number) => assert_eq!(*number, 10324.0),
            _ => assert!(false)
        };
        match &*results[1].as_ref().unwrap().1 {
            BramaAstType::Number(number) => assert_eq!(*number, 5890.0),
            _ => assert!(false)
        };
        match &*results[2].as_ref().unwrap().1 {
            BramaAstType::Number(number) => assert_eq!(*number, 16214.0),
            _ => assert!(false)
        };
        match &*results[3].as_ref().unwrap().1 {
            BramaAstType::Number(number) => assert_eq!(*number, 25965.25),
            _ => assert!(false)
        };
        match &*results[4].as_ref().unwrap().1 {
            BramaAstType::Number(number) => assert_eq!(*number, 3500.0),
            _ => assert!(false)
        };
        match &*results[5].as_ref().unwrap().1 {
            BramaAstType::Number(number) => assert_eq!(*number, 8600.0),
            _ => assert!(false)
        };
        match &*results[6].as_ref().unwrap().1 {
            BramaAstType::Number(number) => assert_eq!(*number, 54279.25),
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_7() {
        let test_data = r"tarih = 11:30
tarih add 12 hour".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 2);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Time(time) => assert_eq!(*time, NaiveTime::from_hms(11, 30, 0)),
            _ => assert!(false)
        };
        match &*results[1].as_ref().unwrap().1 {
            BramaAstType::Time(time) => assert_eq!(*time, NaiveTime::from_hms(23, 30, 0)),
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_8() {
        let test_data = r"tarih = 11:30
tarih add -1 hour".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 2);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Time(time) => assert_eq!(*time, NaiveTime::from_hms(11, 30, 0)),
            _ => assert!(false)
        };
        match &*results[1].as_ref().unwrap().1 {
            BramaAstType::Time(time) => assert_eq!(*time, NaiveTime::from_hms(10, 30, 0)),
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
8Y".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 7);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Number(num) => assert_eq!(*num, 2_000.0),
            _ => assert!(false)
        };
        match &*results[1].as_ref().unwrap().1 {
            BramaAstType::Number(num) => assert_eq!(*num, 3_000_000.0),
            _ => assert!(false)
        };
        match &*results[2].as_ref().unwrap().1 {
            BramaAstType::Number(num) => assert_eq!(*num, 4_000_000_000.0),
            _ => assert!(false)
        };
        match &*results[3].as_ref().unwrap().1 {
            BramaAstType::Number(num) => assert_eq!(*num, 5_000_000_000_000.0),
            _ => assert!(false)
        };
        match &*results[4].as_ref().unwrap().1 {
            BramaAstType::Number(num) => assert_eq!(*num, 6_000_000_000_000_000.0),
            _ => assert!(false)
        };
        match &*results[5].as_ref().unwrap().1 {
            BramaAstType::Number(num) => assert_eq!(*num, 7_000_000_000_000_000_000.0),
            _ => assert!(false)
        };
        match &*results[6].as_ref().unwrap().1 {
            BramaAstType::Number(num) => assert_eq!(*num, 8_000_000_000_000_000_000_000.0),
            _ => assert!(false)
        };
    }


    #[test]
    fn execute_10() {
        let test_data = r"8 / (45 - 20%)".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Number(number) => assert_eq!(*number, 0.2222222222222222),
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_11() {
        let test_data = r"tarih = 11:30
tarih add 1 hour 1 minute 30 second".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 2);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Time(time) => assert_eq!(*time, NaiveTime::from_hms(11, 30, 0)),
            _ => assert!(false)
        };
        match &*results[1].as_ref().unwrap().1 {
            BramaAstType::Time(time) => assert_eq!(*time, NaiveTime::from_hms(12, 31, 30)),
            _ => assert!(false)
        };
    }


    #[test]
    fn execute_12() {
        let test_data = r"5 hour 21 minute 55 second".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Duration(duration) => assert_eq!(*duration, Duration::seconds(19315)),
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_13() {
        let test_data = r"$25/hour * 14 hours of work".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Money(price, currency) => {
                assert_eq!(*price, 350.0);
                assert_eq!(*currency, "usd".to_string());
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_14() {
        let test_data = r"100 minutes 1 seconds".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Duration(duration) => {
                assert_eq!(*duration, Duration::seconds(6001));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_15() {
        let test_data = r"11:40  - 10 minute".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Time(time) => {
                assert_eq!(*time, NaiveTime::from_hms(11, 30, 00));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_16() {
        let test_data = r"11:40  + 1 hour 1 second".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Time(time) => {
                assert_eq!(*time, NaiveTime::from_hms(12, 40, 01));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_17() {
        let test_data = r"3:35 am + 7 hours 15 minutes".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Time(time) => {
                assert_eq!(*time, NaiveTime::from_hms(10, 50, 0));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_18() {
        let test_data = r"10 June + 3 weeks".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Date(date) => {
                assert_eq!(*date, NaiveDate::from_ymd(Local::now().date().year(), 7, 1));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_19() {
        let test_data = r"April 1, 2019 - 3 months 5 days".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Date(date) => {
                assert_eq!(*date, NaiveDate::from_ymd(2018, 12, 27));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_20() {
        let test_data = r"Feb 1, 2019 + 1 months".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Date(date) => {
                assert_eq!(*date, NaiveDate::from_ymd(2019, 3, 1));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_21() {
        let test_data = r"jan 28, 2019 - 14 months".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Date(date) => {
                assert_eq!(*date, NaiveDate::from_ymd(2018, 11, 28));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_22() {
        let test_data = r"jan 28, 2019 - 14 months 10 days".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Date(date) => {
                assert_eq!(*date, NaiveDate::from_ymd(2018, 11, 18));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_23() {
        let test_data = r"jan 28, 2019 - 14 months 33 days".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Date(date) => {
                assert_eq!(*date, NaiveDate::from_ymd(2018, 10, 25));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_24() {
        let test_data = r"12/02/1988 + 32 years ".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Date(date) => {
                assert_eq!(*date, NaiveDate::from_ymd(2020, 02, 12));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_25() {
        let test_data = r"12/02/2020 - 32 years ".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Date(date) => {
                assert_eq!(*date, NaiveDate::from_ymd(1988, 02, 12));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_26() {
        let test_data = r"12/02/2020 - 11680 days".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Date(date) => {
                assert_eq!(*date, NaiveDate::from_ymd(1988, 02, 12));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_27() {
        let test_data = r"1/1/2000 to 3/3/2021".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Duration(date) => {
                assert_eq!(*date, Duration::days(7732));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_28() {
        let test_data = r"3/3/2021 to 1/1/2000".to_string();
        initialize();
        let results = execute(&"en".to_string(), &test_data);

        assert_eq!(results.len(), 1);
        match &*results[0].as_ref().unwrap().1 {
            BramaAstType::Duration(date) => {
                assert_eq!(*date, Duration::days(7732));
            },
            _ => assert!(false)
        };
    }
}
