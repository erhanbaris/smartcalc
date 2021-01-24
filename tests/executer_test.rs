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
    fn execute_1() {
        let test_data = "120 + 30% + 10%";
        let result = Parser::parse(test_data);

        match result {
            Ok(mut tokens) => {
                let worker_executer = WorkerExecuter::new();
                worker_executer.process(&mut tokens);
                let syntax = SyntaxParser::new(Rc::new(tokens), Vec::new());
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

    #[test]
    fn execute_2() {
        let worker_executer = WorkerExecuter::new();
        let test_data = r"
erhan barış = 120
erhan barış + 120";
        let mut asts = Vec::new();
        let mut variables: Vec<Vec<Token>> = Vec::new();

        for text in test_data.lines() {
            let result = Parser::parse(text);
            match result {
                Ok(mut tokens) => {
                    worker_executer.process(&mut tokens);
                    let syntax = SyntaxParser::new(Rc::new(tokens), variables.to_vec());
                    match syntax.parse() {
                        Ok(ast) => {
                            asts.push(Rc::new(ast));
                            variables = syntax.variables.borrow().to_vec();
                        },
                        _ => println!("error")
                    }
                },
                _ => println!("{:?}", result)
            };
        }

        let results = Executer::execute(&asts);
        assert_eq!(results.len(), 3);
        match &**results[1].as_ref().unwrap() {
            BramaAstType::Number(number) => assert_eq!(*number, 120.0),
            _ => assert!(false)
        };
        match &**results[2].as_ref().unwrap() {
            BramaAstType::Number(number) => assert_eq!(*number, 240.0),
            _ => assert!(false)
        };
    }

    #[test]
    fn execute_3() {
        let worker_executer = WorkerExecuter::new();
        let test_data = r"
erhan barış = 120
aysel barış = 200
toplam = erhan barış + aysel barış";
        let mut asts = Vec::new();
        let mut variables: Vec<Vec<Token>> = Vec::new();

        for text in test_data.lines() {
            let result = Parser::parse(text);
            match result {
                Ok(mut tokens) => {
                    worker_executer.process(&mut tokens);
                    let syntax = SyntaxParser::new(Rc::new(tokens), variables.to_vec());
                    match syntax.parse() {
                        Ok(ast) => {
                            asts.push(Rc::new(ast));
                            variables = syntax.variables.borrow().to_vec();
                        },
                        _ => println!("error")
                    }
                },
                _ => println!("{:?}", result)
            };
        }

        let results = Executer::execute(&asts);
        assert_eq!(results.len(), 4);
        match &**results[1].as_ref().unwrap() {
            BramaAstType::Number(number) => assert_eq!(*number, 120.0),
            _ => assert!(false)
        };
        match &**results[2].as_ref().unwrap() {
            BramaAstType::Number(number) => assert_eq!(*number, 200.0),
            _ => assert!(false)
        };
        match &**results[3].as_ref().unwrap() {
            BramaAstType::Number(number) => assert_eq!(*number, 320.0),
            _ => assert!(false)
        };
    }
}