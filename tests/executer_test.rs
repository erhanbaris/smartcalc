extern crate smartcalc;

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use smartcalc::worker::WorkerExecuter;
    use smartcalc::tokinizer::Parser;
    use smartcalc::syntax::SyntaxParser;
    use smartcalc::types::{BramaAstType};
    use smartcalc::compiler::Executer;

    #[test]
    fn execute_1() {
        let test_data = "120 + 30% + 10%";
        let result = Parser::parse(test_data);
        match result {
            Ok(mut tokens) => {
                let worker_executer = WorkerExecuter::new();
                worker_executer.process(&mut tokens);
                let syntax = SyntaxParser::new(Box::new(tokens));
                match syntax.parse() {
                    Ok(ast) => {
                        let results = Executer::execute(&vec![Rc::new(ast)]);
                        match &**results[0].as_ref().unwrap() {
                            BramaAstType::Number(number) => assert_eq!(*number, 171.6),
                            _ => assert!(false)
                        };
                    },
                    _ => assert!(false)
                }
            },
            _ => println!("{:?}", result)
        };
    }
}