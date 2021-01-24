extern crate smartcalc;

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use smartcalc::tokinizer::Parser;
    use smartcalc::types::Token;

    #[test]
    fn tokinizer_1() {
        let test_data = "120 add 30%";
        let result = Parser::parse(test_data);
        match result {
            Ok(tokens) => assert_eq!(tokens.len(), 3),
            _ => assert!(false)
        };
    }

    #[test]
    fn tokinizer_2() {
        let test_data = "120 + 30%";
        let result = Parser::parse(test_data);
        match result {
            Ok(tokens) => assert_eq!(tokens.len(), 3),
            _ => assert!(false)
        };
    }

    #[test]
    fn tokinizer_3() {
        let test_data = "120 + 30%";
        let result = Parser::parse(test_data);
        match result {
            Ok(tokens) => assert_eq!(tokens, vec![Token::Number(120.0), Token::Operator('+'), Token::Percent(30.0)]),
            _ => assert!(false)
        };
    }

    #[test]
    fn tokinizer_4() {
        let test_data = "120 add 30%";
        let result = Parser::parse(test_data);
        match result {
            Ok(tokens) => assert_eq!(tokens, vec![Token::Number(120.0), Token::Text(Rc::new("add".to_string())), Token::Percent(30.0)]),
            _ => assert!(false)
        };
    }

    #[test]
    fn tokinizer_5() {
        let test_data = "120 add %30";
        let result = Parser::parse(test_data);
        match result {
            Ok(tokens) => assert_eq!(tokens, vec![Token::Number(120.0), Token::Text(Rc::new("add".to_string())), Token::Percent(30.0)]),
            _ => assert!(false)
        };
    }

    #[test]
    fn tokinizer_6() {
        let test_data = "%30 + 120";
        let result = Parser::parse(test_data);
        match result {
            Ok(tokens) => assert_eq!(tokens, vec![Token::Percent(30.0), Token::Operator('+'), Token::Number(120.0)]),
            _ => assert!(false)
        };
    }
}