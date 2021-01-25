extern crate smartcalc;

#[cfg(test)]
mod tests {
    use smartcalc::worker::WorkerExecuter;
    use smartcalc::tokinizer::Parser;
    use smartcalc::types::Token;

    #[test]
    fn alias_1() {
        let worker_executer = WorkerExecuter::new();
        let test_data       = "120 add %30".to_string();
        let result = Parser::parse(&test_data);
        match result {
            Ok(mut tokens) => {
                worker_executer.process(&"en".to_string(), &mut tokens);
                assert_eq!(tokens, vec![Token::Number(120.0), Token::Operator('+'), Token::Percent(30.0)]);
            },
            _ => assert!(false)
        };
    }
}