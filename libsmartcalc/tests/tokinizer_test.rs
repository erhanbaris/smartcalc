extern crate libsmartcalc;

#[cfg(test)]
mod tests {
    use libsmartcalc::tokinizer::Tokinizer;
    use libsmartcalc::types::TokenType;

    #[test]
    fn tokinizer_1() {
        let test_data = "120 add 30%".to_string();
        let result = Tokinizer::tokinize(&test_data);
        match result {
            Ok(tokens) => assert_eq!(tokens.len(), 3),
            _ => assert!(false)
        };
    }

    #[test]
    fn tokinizer_2() {
        let test_data = "120 + 30%".to_string();
        let result = Tokinizer::tokinize(&test_data);
        match result {
            Ok(tokens) => assert_eq!(tokens.len(), 3),
            _ => assert!(false)
        };
    }

    #[test]
    fn tokinizer_3() {
        let test_data = "120 + 30%".to_string();
        let result = Tokinizer::tokinize(&test_data);
        match result {
            Ok(tokens) => {
                assert_eq!(tokens[0].token, TokenType::Number(120.0));
                assert_eq!(tokens[1].token, TokenType::Operator('+'));
                assert_eq!(tokens[2].token, TokenType::Percent(30.0));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn tokinizer_4() {
        let test_data = "120 + 30%".to_string();
        let result = Tokinizer::tokinize(&test_data);
        match result {
            Ok(tokens) => {
                assert_eq!(tokens[0].token, TokenType::Number(120.0));
                assert_eq!(tokens[1].token, TokenType::Operator('+'));
                assert_eq!(tokens[2].token, TokenType::Percent(30.0));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn tokinizer_5() {
        let test_data = "120 + %30".to_string();
        let result = Tokinizer::tokinize(&test_data);
        match result {
            Ok(tokens) => {
                assert_eq!(tokens[0].token, TokenType::Number(120.0));
                assert_eq!(tokens[1].token, TokenType::Operator('+'));
                assert_eq!(tokens[2].token, TokenType::Percent(30.0));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn tokinizer_6() {
        let test_data = "%30 + 120";
        let result = Tokinizer::tokinize(&test_data.to_string());
        match result {
            Ok(tokens) => {
                assert_eq!(tokens[0].token, TokenType::Percent(30.0));
                assert_eq!(tokens[1].token, TokenType::Operator('+'));
                assert_eq!(tokens[2].token, TokenType::Number(120.0));
            },
            _ => assert!(false)
        };
    }
}