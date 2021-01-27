extern crate smartcalc;

#[cfg(test)]
mod tests {
    use smartcalc::worker::WorkerExecuter;
    use smartcalc::tokinizer::Tokinizer;
    use smartcalc::types::Token;
    use std::rc::Rc;
    use smartcalc::executer::{prepare_code, Storage};

    #[test]
    fn alias_1() {
        let prepared_code   = prepare_code(&"120 add %30".to_string());
        let storage         = Rc::new(Storage::new());
        let worker_executer = WorkerExecuter::new();
        let result = Tokinizer::tokinize(&prepared_code);
        match result {
            Ok(mut tokens) => {
                worker_executer.process(&"en".to_string(), &mut tokens, storage.clone());
                assert_eq!(tokens, vec![Token::Number(120.0), Token::Operator('+'), Token::Percent(30.0)]);
            },
            _ => assert!(false)
        };
    }
}