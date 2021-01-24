extern crate smartcalc;

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use smartcalc::worker::WorkerExecuter;
    use smartcalc::tokinizer::Parser;
    use smartcalc::syntax::SyntaxParser;
    use smartcalc::types::{BramaAstType, Token};

    #[test]
    fn alias_1() {
        let worker_executer = WorkerExecuter::new();
        let test_data       = "120 add %30";
        let result = Parser::parse(test_data);
        match result {
            Ok(mut tokens) => {
                worker_executer.process(&mut tokens);
                let syntax = SyntaxParser::new(Box::new(tokens));
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