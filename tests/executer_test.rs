extern crate smartcalc;

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use smartcalc::worker::WorkerExecuter;
    use smartcalc::tokinizer::Parser;
    use smartcalc::syntax::SyntaxParser;
    use smartcalc::types::{BramaAstType, Token};
    use smartcalc::compiler::Executer;

    #[test]
    fn alias_1() {
        let test_data = "120 add 30%";
        let result = Parser::parse(test_data);
        match result {
            Ok(mut tokens) => {
                let worker_executer = WorkerExecuter::new();
                worker_executer.process(&mut tokens);
                let syntax = SyntaxParser::new(Box::new(tokens));
                match syntax.parse() {
                    Ok(ast) => {
                        let results = Executer::execute(&vec![ast]);
                        assert_eq!(*results[0].as_ref().unwrap(), 156.0);
                    },
                    _ => println!("error")
                }
            },
            _ => println!("{:?}", result)
        };
    }
}