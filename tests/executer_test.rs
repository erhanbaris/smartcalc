extern crate smartcalc;

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use smartcalc::worker::WorkerExecuter;
    use smartcalc::tokinizer::Tokinizer;
    use smartcalc::syntax::SyntaxParser;
    use smartcalc::types::{BramaAstType, Token};
    use smartcalc::compiler::Interpreter;

    #[test]
    fn execute_1() {
        let test_data = "120 + 30% + 10%".to_string();
        let result = Tokinizer::tokinize(&test_data);

        match result {
            Ok(mut tokens) => {
                let worker_executer = WorkerExecuter::new();
                worker_executer.process(&"en".to_string(), &mut tokens);
                let syntax = SyntaxParser::new(Rc::new(tokens), Vec::new());
                match syntax.parse() {
                    Ok(ast) => {
                        let results = Interpreter::execute(&vec![Rc::new(ast)]);
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
erhan barış + 120".to_string();
        let mut asts = Vec::new();
        let mut variables: Vec<Vec<Token>> = Vec::new();

        for text in test_data.lines() {
            let result = Tokinizer::tokinize(&text.to_string());
            match result {
                Ok(mut tokens) => {
                    worker_executer.process(&"en".to_string(), &mut tokens);
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

        let results = Interpreter::execute(&asts);
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
            let result = Tokinizer::tokinize(&text.to_string());
            match result {
                Ok(mut tokens) => {
                    worker_executer.process(&"en".to_string(), &mut tokens);
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

        let results = Interpreter::execute(&asts);
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

    #[test]
    fn execute_4() {
        let worker_executer = WorkerExecuter::new();
        let test_data = r"
erhan barış = 120
aysel barış = 200
toplam = erhan barış + test aysel barış";
        let mut asts = Vec::new();
        let mut variables: Vec<Vec<Token>> = Vec::new();

        for text in test_data.lines() {
            let result = Tokinizer::tokinize(&text.to_string());
            match result {
                Ok(mut tokens) => {
                    worker_executer.process(&"en".to_string(), &mut tokens);
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

        let results = Interpreter::execute(&asts);
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

    #[test]
    fn execute_5() {
        let worker_executer = WorkerExecuter::new();
        let test_data = r"100 200";
        let mut asts = Vec::new();
        let mut variables: Vec<Vec<Token>> = Vec::new();

        for text in test_data.lines() {
            let result = Tokinizer::tokinize(&text.to_string());
            match result {
                Ok(mut tokens) => {
                    worker_executer.process(&"en".to_string(), &mut tokens);
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

        let results = Interpreter::execute(&asts);
        assert_eq!(results.len(), 1);
        match &**results[0].as_ref().unwrap() {
            BramaAstType::Number(number) => assert_eq!(*number, 300.0),
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

    #[test]
    fn execute_6() {
        let worker_executer = WorkerExecuter::new();
        let test_data = r"aysel = 10324
erhan = 5890
nakit = erhan + aysel
erhan maaş = 25965.25
aysel maaş = 3500
sigorta geri ödemesi = 8600
toplam nakit = nakit + erhan maaş + aysel maaş + sigorta geri ödemesi";
        let mut asts = Vec::new();
        let mut variables: Vec<Vec<Token>> = Vec::new();

        for text in test_data.lines() {
            let result = Tokinizer::tokinize(&text.to_string());
            match result {
                Ok(mut tokens) => {
                    worker_executer.process(&"en".to_string(), &mut tokens);
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

        let results = Interpreter::execute(&asts);
        assert_eq!(results.len(), 7);
        match &**results[0].as_ref().unwrap() {
            BramaAstType::Number(number) => assert_eq!(*number, 10324.0),
            _ => assert!(false)
        };
        match &**results[1].as_ref().unwrap() {
            BramaAstType::Number(number) => assert_eq!(*number, 5890.0),
            _ => assert!(false)
        };
        match &**results[2].as_ref().unwrap() {
            BramaAstType::Number(number) => assert_eq!(*number, 16214.0),
            _ => assert!(false)
        };
        match &**results[3].as_ref().unwrap() {
            BramaAstType::Number(number) => assert_eq!(*number, 25965.25),
            _ => assert!(false)
        };
        match &**results[4].as_ref().unwrap() {
            BramaAstType::Number(number) => assert_eq!(*number, 3500.0),
            _ => assert!(false)
        };
        match &**results[5].as_ref().unwrap() {
            BramaAstType::Number(number) => assert_eq!(*number, 8600.0),
            _ => assert!(false)
        };
        match &**results[6].as_ref().unwrap() {
            BramaAstType::Number(number) => assert_eq!(*number, 54279.25),
            _ => assert!(false)
        };
    }
}