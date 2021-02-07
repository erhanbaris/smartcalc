extern crate libsmartcalc;

#[cfg(test)]
mod tests {
    use libsmartcalc::tokinizer::Tokinizer;
    use libsmartcalc::types::TokenType;
    use libsmartcalc::executer::initialize;

    #[test]
    fn tokinizer_1() {
        initialize();
        let test_data = "120 add 30%".to_string();
        let result = Tokinizer::tokinize(&test_data);
        match result {
            Ok(tokens) => assert_eq!(tokens.len(), 3),
            _ => assert!(false)
        };
    }

    #[test]
    fn tokinizer_2() {
        initialize();
        let test_data = "120 + 30%".to_string();
        let result = Tokinizer::tokinize(&test_data);
        match result {
            Ok(tokens) => assert_eq!(tokens.len(), 3),
            _ => assert!(false)
        };
    }

    #[test]
    fn tokinizer_3() {
        initialize();
        let test_data = "120 + 30%".to_string();
        let result = Tokinizer::tokinize(&test_data);
        match result {
            Ok(tokens) => {
                assert_eq!(tokens[0].token_type, Some(TokenType::Number(120.0)));
                assert_eq!(tokens[1].token_type, Some(TokenType::Operator('+')));
                assert_eq!(tokens[2].token_type, Some(TokenType::Percent(30.0)));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn tokinizer_4() {
        initialize();
        let test_data = "120 + 30%".to_string();
        let result = Tokinizer::tokinize(&test_data);
        match result {
            Ok(tokens) => {
                assert_eq!(tokens[0].token_type, Some(TokenType::Number(120.0)));
                assert_eq!(tokens[1].token_type, Some(TokenType::Operator('+')));
                assert_eq!(tokens[2].token_type, Some(TokenType::Percent(30.0)));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn tokinizer_5() {
        initialize();
        let test_data = "120 + %30".to_string();
        let result = Tokinizer::tokinize(&test_data);
        match result {
            Ok(tokens) => {
                assert_eq!(tokens[0].token_type, Some(TokenType::Number(120.0)));
                assert_eq!(tokens[1].token_type, Some(TokenType::Operator('+')));
                assert_eq!(tokens[2].token_type, Some(TokenType::Percent(30.0)));
            },
            _ => assert!(false)
        };
    }

    #[test]
    fn tokinizer_6() {
        initialize();
        let test_data = "%30 + 120";
        let result = Tokinizer::tokinize(&test_data.to_string());
        match result {
            Ok(tokens) => {
                assert_eq!(tokens[0].token_type, Some(TokenType::Percent(30.0)));
                assert_eq!(tokens[1].token_type, Some(TokenType::Operator('+')));
                assert_eq!(tokens[2].token_type, Some(TokenType::Number(120.0)));
            },
            _ => assert!(false)
        };
    }
}