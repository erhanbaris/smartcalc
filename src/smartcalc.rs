/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::borrow::Borrow;
use core::ops::Deref;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use anyhow::anyhow;
use crate::Session;
use crate::tokinizer::read_currency;

use crate::compiler::Interpreter;
use crate::logger::{LOGGER, initialize_logger};
use crate::syntax::SyntaxParser;
use crate::token::ui_token::UiToken;
use crate::tokinizer::TokenInfo;
use crate::tokinizer::Tokinizer;
use crate::tools::parse_timezone;
use crate::types::TokenType;
use crate::types::SmartCalcAstType;
use crate::formatter::format_result;
use crate::config::SmartCalcConfig;

pub type ExecutionLine = Option<ExecuteLine>;

pub trait RuleTrait {
    fn name(&self) -> String;
    fn call(&self, smartcalc: &SmartCalcConfig, fields: &BTreeMap<String, TokenType>) -> Option<TokenType>;
}

#[derive(Debug)]
#[derive(Default)]
pub struct ExecuteResult {
    pub status: bool,
    pub lines: Vec<ExecutionLine>
}

#[derive(Debug, Clone)]
pub struct ExecuteLineResult {
    pub output: String,
    pub ast: Rc<SmartCalcAstType>
}

impl ExecuteLineResult {
    pub fn new(output: String, ast: Rc<SmartCalcAstType>) -> Self {
        ExecuteLineResult { output, ast }
    }
}

#[derive(Debug)]
pub struct ExecuteLine {
    pub result: Result<ExecuteLineResult, String>,
    pub raw_tokens: Vec<Rc<TokenType>>,
    pub ui_tokens: Vec<UiToken>,
    pub calculated_tokens: Vec<Rc<TokenInfo>>
}

impl ExecuteLine {
    pub fn new(result: Result<ExecuteLineResult, String>, ui_tokens: Vec<UiToken>, raw_tokens: Vec<Rc<TokenType>>, calculated_tokens: Vec<Rc<TokenInfo>>) -> Self {
        ExecuteLine { result, ui_tokens, raw_tokens, calculated_tokens }
    }
}

pub struct SmartCalc {
    config: SmartCalcConfig
}

impl Default for SmartCalc {
    fn default() -> Self {
        initialize_logger();
        SmartCalc {
            config: SmartCalcConfig::default()
        }
    }
}

impl SmartCalc {
    pub fn set_decimal_seperator(&mut self, decimal_seperator: String) {
        self.config.decimal_seperator = decimal_seperator;
    }
    
    pub fn set_thousand_separator(&mut self, thousand_separator: String) {
        self.config.thousand_separator = thousand_separator;
    }
    
    pub fn set_timezone(&mut self, timezone: String) -> Result<(), String> {
        let timezone = match self.config.token_parse_regex.get("timezone") {
            Some(regexes) => {
                let capture = regexes[0].captures(&timezone).unwrap();
                match capture.name("timezone") {
                    Some(_) => parse_timezone(&self.config, &capture),
                    None => None
                }
            },
            _ => None
        };
        
        match timezone {
            Some((timezone, offset)) => {
                self.config.timezone = timezone.to_uppercase();
                self.config.timezone_offset = offset;
                Ok(())
            },
            None => Err("Timezone information not found".to_string())
        }
    }
    
    pub fn load_from_json(json_data: &str) -> Self {
        SmartCalc {
            config: SmartCalcConfig::load_from_json(json_data)
        }
    }

    pub fn update_currency(&mut self, currency: &str, rate: f64) -> bool {
        match read_currency(&self.config, currency) {
            Some(real_currency) => {
                self.config.currency_rate.insert(real_currency, rate);
                true
            },
             _ => false
        }
    }

    pub fn add_rule(&mut self, language: String, rules: Vec<String>, callback: Rc<dyn RuleTrait>) -> Result<(), ()> {
        let mut rule_tokens = Vec::new();
        
        for rule_item in rules.iter() {
            let mut session = Session::new();
            session.set_language(language.to_string());
            session.set_text(rule_item.to_string());
            let tokens = Tokinizer::token_infos(&self.config, &session);
            rule_tokens.push(tokens);
        }
        
        let language_data = match self.config.api_parser.get_mut(&language) {
            Some(language) => language,
            None => {
                self.config.api_parser.insert(language.to_string(), Vec::new());
                self.config.api_parser.get_mut(&language).unwrap()
            }
        };
        
        language_data.push((rule_tokens, callback));
        Ok(())
    }
    
    pub fn format_result(&self, session: &Session, result: Rc<SmartCalcAstType>) -> String {
        format_result(&self.config, session, result)
    }

    pub fn initialize() {
        if log::set_logger(&LOGGER).is_ok() {
            if cfg!(debug_assertions) {
                log::set_max_level(log::LevelFilter::Debug)
            } else {
                log::set_max_level(log::LevelFilter::Info)
            }
        }
    }


    pub(crate) fn execute_text(&self, session: &Session) -> ExecutionLine {
        log::debug!("> {}", session.current_line());
        if session.current_line().is_empty() {
            return None;
        }

        let mut tokinizer = Tokinizer::new(&self.config, session);
        if !tokinizer.tokinize() {
            return None;
        }

        let mut syntax = SyntaxParser::new(session, &tokinizer);
        log::debug!(" > parse starting");

        let execution_result = match syntax.parse() {
            Ok(ast) => {
                log::debug!(" > parse Ok {:?}", ast);
                let ast_rc = Rc::new(ast);

                match Interpreter::execute(&self.config, ast_rc, session) {
                    Ok(ast) => Ok(ExecuteLineResult::new(self.format_result(session, ast.clone()), ast)),
                    Err(error) => Err(error)
                }
            },
            Err((error, _, _)) => {
                log::debug!(" > parse Err");
                log::info!("Syntax parse error, {}", error);
                Err(error.to_string())
            }
        };
        
        Some(ExecuteLine::new(execution_result, tokinizer.ui_tokens.get_tokens(), tokinizer.tokens, tokinizer.token_infos.clone()))
    }

    pub fn execute<Tlan: Borrow<str>, Tdata: Borrow<str>>(&self, language: Tlan, data: Tdata) -> ExecuteResult {
        let mut session = Session::new();

        session.set_text(data.borrow().to_string());
        session.set_language(language.borrow().to_string());
        self.execute_session(&session)
    }

    pub fn basic_execute<T: Borrow<str>>(data: T, config: &SmartCalcConfig) -> anyhow::Result<f64> {
        let mut session = Session::new();

        session.set_text(data.borrow().to_string());
        session.set_language("en".borrow().to_string());
        
        if session.line_count() != 1 {
            return Err(anyhow!("Multiline calculation not supported"));
        }
        
        if !session.has_value() {
            return Err(anyhow!("No data found"));
        }

        log::debug!("> {}", session.current_line());
        if session.current_line().is_empty() {
            return Err(anyhow!("Calculation empty"));
        }

        let mut tokinizer = Tokinizer::new(config, &session);
        if !tokinizer.basic_tokinize() {
            return Err(anyhow!("Syntax error"));
        }

        let mut syntax = SyntaxParser::new(&session, &tokinizer);
        log::debug!(" > parse starting");

        match syntax.parse() {
            Ok(ast) => {
                log::debug!(" > parse Ok {:?}", ast);
                let ast_rc = Rc::new(ast);

                match Interpreter::execute(config, ast_rc, &session) {
                    Ok(ast) => {
                        match ast.deref() {
                            SmartCalcAstType::Item(item) => Ok(item.get_underlying_number()),
                            _ => Err(anyhow!("Number not found"))
                        }
                    },
                    Err(error) => Err(anyhow!(error))
                }
            },
            Err((error, _, _)) => {
                log::debug!(" > parse Err");
                log::info!("Syntax parse error, {}", error);
                Err(anyhow!(error))
            }
        }
    }

    pub fn execute_session(&self, session: &Session) -> ExecuteResult {
        let mut results = ExecuteResult::default();

        if session.has_value() {
            results.status = true;
            loop {
                let line_result = self.execute_text(session);
                results.lines.push(line_result);
                if session.next_line().is_none() {
                    break;
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod test {
    use core::ops::Deref;
    use alloc::{collections::BTreeMap, string::{String, ToString}, vec, rc::Rc};

    use crate::{SmartCalc, types::{TokenType, NumberType}, RuleTrait, SmartCalcConfig};

    #[derive(Default)]
    pub struct Test1;
    impl RuleTrait for Test1 {
        fn name(&self) -> String {
            "CTest1oin".to_string()
        }
        fn call(&self, _: &SmartCalcConfig, fields: &BTreeMap<String, TokenType>) -> Option<TokenType> {
            match fields.get("surname") {
                Some(TokenType::Text(surname)) => {
                    assert_eq!(surname, &"baris".to_string());
                    Some(TokenType::Number(2022.0, NumberType::Decimal))
                },
                _ => None
            }
         }
    }
    
    #[test]
    fn add_rule_1() ->  Result<(), ()> {
        let mut calculater = SmartCalc::default();
        let test1 = Rc::new(Test1::default());
        calculater.add_rule("en".to_string(), vec!["erhan {TEXT:surname}".to_string(), "{TEXT:surname} erhan".to_string()], test1.clone())?;
        let result = calculater.execute("en".to_string(), "erhan baris");
        assert!(result.status);
        assert_eq!(result.lines.len(), 1);
        assert_eq!(result.lines[0].is_some(), true);
        match result.lines.get(0) {
            Some(line) => match line {
                Some(item) => match item.calculated_tokens.get(0) {
                    Some(calculated_token) => match calculated_token.token_type.borrow().deref() {
                        Some(TokenType::Number(number, NumberType::Decimal)) => assert_eq!(*number, 2022.0),
                        _ => assert!(false, "Expected token wrong")
                    },
                    None => assert!(false, "Calculated token not found")
                },
                None => assert!(false, "Result line does not have value")
            },
            None => assert!(false, "Result line not found")
        };

        let result = calculater.execute("en".to_string(), "baris erhan");
        assert!(result.status);
        assert_eq!(result.lines.len(), 1);
        assert_eq!(result.lines[0].is_some(), true);
        match result.lines.get(0) {
            Some(line) => match line {
                Some(item) => match item.calculated_tokens.get(0) {
                    Some(calculated_token) => match calculated_token.token_type.borrow().deref() {
                        Some(TokenType::Number(number, NumberType::Decimal)) => assert_eq!(*number, 2022.0),
                        _ => assert!(false, "Expected token wrong")
                    },
                    None => assert!(false, "Calculated token not found")
                },
                None => assert!(false, "Result line does not have value")
            },
            None => assert!(false, "Result line not found")
        };
        Ok(())
    }

    #[test]
    fn basic_test_1() ->  anyhow::Result<()> {
        let calculater = SmartCalc::default();
        let result = calculater.basic_execute("1024")?;
        assert_eq!(result, 1024.0);
        Ok(())
    }
    
    #[test]
    fn basic_test_2() ->  anyhow::Result<()> {
        let calculater = SmartCalc::default();
        let result = calculater.basic_execute("1024 * 2")?;
        assert_eq!(result, 2048.0);
        Ok(())
    }
    
    #[test]
    fn basic_test_3() ->  anyhow::Result<()> {
        let calculater = SmartCalc::default();
        let error = match calculater.basic_execute("a + 1024 * 2") {
            Ok(_) => return Ok(()),
            Err(error) => error
        };
        assert_eq!(error.to_string(), "Number not found".to_string());
        Ok(())
    }
    
    #[test]
    fn basic_test_4() ->  anyhow::Result<()> {
        let calculater = SmartCalc::default();
        let error = match calculater.basic_execute("+ 1024 * 2") {
            Ok(_) => return Ok(()),
            Err(error) => error
        };
        assert_eq!(error.to_string(), "No more token".to_string());
        Ok(())
    }
    
    #[test]
    fn basic_test_5() ->  anyhow::Result<()> {
        let calculater = SmartCalc::default();
        let error = match calculater.basic_execute(r#"1+ 1024 * 2
"#) {
            Ok(_) => return Ok(()),
            Err(error) => error
        };
        assert_eq!(error.to_string(), "Multiline calculation not supported".to_string());
        Ok(())
    }
    
    #[test]
    fn basic_test_6() ->  anyhow::Result<()> {
        let calculater = SmartCalc::default();
        let error = match calculater.basic_execute(r#""#) {
            Ok(_) => return Ok(()),
            Err(error) => error
        };
        assert_eq!(error.to_string(), "Calculation empty".to_string());
        Ok(())
    }
}
