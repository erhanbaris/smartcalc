extern crate libsmartcalc;

#[cfg(test)]
mod tests {
    use libsmartcalc::worker::WorkerExecuter;
    use libsmartcalc::tokinizer::Tokinizer;
    use libsmartcalc::types::TokenType;
    use std::rc::Rc;
    use libsmartcalc::executer::{prepare_code, Storage, initialize};

    #[test]
    fn alias_1() {
        initialize();
        let prepared_code   = prepare_code(&"120 add %30".to_string());
        let storage         = Rc::new(Storage::new());
        let worker_executer = WorkerExecuter::new();
        let result = Tokinizer::tokinize(&prepared_code);
        match result {
            Ok(mut tokens) => {
                worker_executer.process(&"en".to_string(), &mut tokens, storage.clone());
                assert_eq!(tokens[0].token, TokenType::Number(120.0));
                assert_eq!(tokens[1].token, TokenType::Operator('+'));
                assert_eq!(tokens[2].token, TokenType::Percent(30.0));
            },
            _ => assert!(false)
        };
    }
}