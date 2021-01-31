extern crate libsmartcalc;

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use libsmartcalc::worker::WorkerExecuter;
    use libsmartcalc::tokinizer::Tokinizer;
    use libsmartcalc::syntax::SyntaxParser;
    use libsmartcalc::types::{BramaAstType};
    use libsmartcalc::executer::Storage;

    #[test]
    fn add_1() {
        let storage         = Rc::new(Storage::new());
        let worker_executer = WorkerExecuter::new();
        let test_data       = "120 + %30".to_string();
        let result = Tokinizer::tokinize(&test_data);
        match result {
            Ok(mut tokens) => {
                worker_executer.process(&"en".to_string(), &mut tokens, storage.clone());
                let syntax = SyntaxParser::new(Rc::new(tokens), storage.clone());
                match syntax.parse() {
                    Ok(BramaAstType::Binary { left, operator, right}) => {
                        assert_eq!(*left, BramaAstType::Number(120.0));
                        assert_eq!(operator, '+');
                        assert_eq!(*right, BramaAstType::Percent(30.0));
                    },
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        };
    }
}