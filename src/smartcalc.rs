/*
 * smartcalc v1.0.7
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::borrow::Borrow;
use core::ops::Deref;
use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use anyhow::anyhow;
use crate::Session;
use crate::tokinizer::{read_currency, RuleType, small_date};

use crate::compiler::Interpreter;
use crate::logger::{LOGGER, initialize_logger};
use crate::syntax::SyntaxParser;
use crate::token::ui_token::UiToken;
use crate::tokinizer::TokenInfo;
use crate::tokinizer::Tokinizer;
use crate::tools::parse_timezone;
use crate::types::{TokenType, ExpressionFunc};
use crate::types::SmartCalcAstType;
use crate::formatter::format_result;
use crate::config::{SmartCalcConfig, DynamicType};

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
        let mut smartcalc = SmartCalc {
            config: SmartCalcConfig::default()
        };
        smartcalc.set_date_rule("en", vec![
            "{MONTH:month} {NUMBER:day}, {NUMBER:year}".to_string(),
            "{MONTH:month} {NUMBER:day} {NUMBER:year}".to_string(),
            "{NUMBER:day}/{NUMBER:month}/{NUMBER:year}".to_string(),
            "{NUMBER:day} {MONTH:month} {NUMBER:year}".to_string(),
            "{NUMBER:day} {MONTH:month}".to_string()
        ]);
        smartcalc.set_date_rule("tr", vec![
            "{NUMBER:day}/{NUMBER:month}/{NUMBER:year}".to_string(),
            "{NUMBER:day} {MONTH:month} {NUMBER:year}".to_string(),
            "{NUMBER:day} {MONTH:month}".to_string()
        ]);
        smartcalc
    }
}

impl SmartCalc {
    pub fn add_dynamic_type<T: Borrow<str>>(&mut self, name: T) -> bool {
        match self.config.types.get(name.borrow()) {
            Some(_) => false,
            None => {
                self.config.types.insert(name.borrow().to_string(), BTreeMap::new());
                true
            }
        }
    }
    
    pub fn add_dynamic_type_item<T: Borrow<str>>(&mut self, name: T, index: usize, format: T, parse: Vec<T>, upgrade_code: T, downgrade_code: T, names:Vec<String>, decimal_digits: Option<u8>, use_fract_rounding: Option<bool>, remove_fract_if_zero: Option<bool>) -> bool {
        match self.config.types.get(name.borrow()) {
            Some(dynamic_type) => {
                if dynamic_type.contains_key(&index) {
                    return false;
                }
            },
            None => return false
        };
            
        let mut parse_tokens = Vec::new();
        for type_parse_item in parse.iter() {
            let mut session = Session::new();
            session.set_language("en".to_string());
            session.set_text(type_parse_item.borrow().to_string());
            
            let tokens = Tokinizer::token_infos(&self.config, &session);
            parse_tokens.push(tokens);
        }
        
        if let Some(dynamic_type) = self.config.types.get_mut(name.borrow()) {            
            dynamic_type.insert(index, Rc::new(DynamicType::new(name.borrow().to_string(), index, format.borrow().to_string(), parse_tokens, upgrade_code.borrow().to_string(), downgrade_code.borrow().to_string(), names, decimal_digits, use_fract_rounding, remove_fract_if_zero)));
        }
        true
    }
    
    pub fn set_decimal_seperator(&mut self, decimal_seperator: String) {
        self.config.decimal_seperator = decimal_seperator;
    }
    
    pub fn set_thousand_separator(&mut self, thousand_separator: String) {
        self.config.thousand_separator = thousand_separator;
    }
    
    pub fn set_date_rule(&mut self, language: &str, rules: Vec<String>) {        
        let current_rules = match self.config.rule.get_mut(language) {
            Some(current_rules) => current_rules,
            None => return
        };
        
        let mut function_items = Vec::new();
        
        let config = SmartCalcConfig::default(); //todo:: find a way to use self.config
        for rule_item in rules {
            let mut session = Session::new();
            session.set_language(language.to_string());
            session.set_text(rule_item.to_string());
            function_items.push(Tokinizer::token_infos(&config, &session));
        }
        
        /* Remove small_date rule */
        current_rules.retain(|rule| {
            let is_small_date = if let RuleType::Internal { function_name, .. } = rule {
                function_name == "small_date"
            } else {
                false
            };
            
            !is_small_date
        });
        
        current_rules.push(RuleType::Internal {
            function_name: "small_date".to_string(),
            function: small_date as ExpressionFunc,
            tokens_list: function_items
        });
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
    
    pub fn delete_rule(&mut self, language: String, rule_name: String) -> bool {
        match self.config.rule.get_mut(&language) {
            Some(language_collection) => {
                let position = language_collection.iter().position(|item| match item {
                    RuleType::API { tokens_list: _, rule: rule_item } => rule_name == rule_item.name(),
                    _ => false
                });
                
                match position {
                    Some(location) => {
                        language_collection.remove(location);
                        true
                    }
                    _ => false
                }
            },
            None => false
        }
    }
    
    pub fn add_rule(&mut self, language: String, rules: Vec<String>, rule: Rc<dyn RuleTrait>) -> bool {
        let mut rule_tokens = Vec::new();
        
        for rule_item in rules.iter() {
            let mut session = Session::new();
            session.set_language(language.to_string());
            session.set_text(rule_item.to_string());
            let tokens = Tokinizer::token_infos(&self.config, &session);
            rule_tokens.push(tokens);
        }
        
        let language_data = match self.config.rule.get_mut(&language) {
            Some(language) => language,
            None => return false
        };
        
        language_data.push(RuleType::API {
            tokens_list: rule_tokens,
            rule
        });
        true
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
            "test1".to_string()
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
    
    macro_rules! check_basic_rule_output {
        ($result:ident, $expected:expr) => {
            assert!($result.status);
            assert_eq!($result.lines.len(), 1);
            assert_eq!($result.lines[0].is_some(), true);
            match $result.lines.get(0) {
                Some(line) => match line {
                    Some(item) => match item.calculated_tokens.get(0) {
                        Some(calculated_token) => assert_eq!(calculated_token.token_type.borrow().deref().as_ref().unwrap(), &$expected),
                        None => assert!(false, "Calculated token not found")
                    },
                    None => assert!(false, "Result line does not have value")
                },
                None => assert!(false, "Result line not found")
            };
        };
    }

    macro_rules! check_dynamic_type_output {
        ($result:ident, $type_name:literal, $expected:literal) => {
            assert!($result.status);
            assert_eq!($result.lines.len(), 1);
            assert_eq!($result.lines[0].is_some(), true);
            
            match $result.lines.get(0) {
                Some(line) => match line {
                    Some(item) => match item.calculated_tokens.get(0) {
                        Some(calculated_token) => match calculated_token.token_type.borrow().deref() {
                            Some(TokenType::DynamicType(number, item)) => {
                                assert_eq!(*number, $expected);
                                assert_eq!(item.deref().group_name, $type_name.to_string());
                            },
                            _ => assert!(false, "Expected token wrong. Token: {:?}", calculated_token.token_type.borrow().deref())
                        },
                        None => assert!(false, "Calculated token not found")
                    },
                    None => assert!(false, "Result line does not have value")
                },
                None => assert!(false, "Result line not found")
            };
        };
    }
    
    #[test]
    fn add_rule_1() ->  Result<(), ()> {
        let mut calculater = SmartCalc::default();
        let test1 = Rc::new(Test1::default());
        calculater.add_rule("en".to_string(), vec!["erhan {TEXT:surname}".to_string(), "{TEXT:surname} erhan".to_string()], test1.clone());
        let result = calculater.execute("en".to_string(), "erhan baris");
        check_basic_rule_output!(result, TokenType::Number(2022.0, NumberType::Decimal));

        let result = calculater.execute("en".to_string(), "baris erhan");
        check_basic_rule_output!(result, TokenType::Number(2022.0, NumberType::Decimal));

        Ok(())
    }
    
    #[test]
    fn add_rule_2() ->  Result<(), ()> {
        let mut calculater = SmartCalc::default();
        let test1 = Rc::new(Test1::default());
        calculater.add_rule("en".to_string(), vec!["erhan {TEXT:surname:baris}".to_string(), "{TEXT:surname:baris} erhan".to_string()], test1.clone());
        let result = calculater.execute("en".to_string(), "erhan baris");
        check_basic_rule_output!(result, TokenType::Number(2022.0, NumberType::Decimal));

        let result = calculater.execute("en".to_string(), "baris erhan");
        check_basic_rule_output!(result, TokenType::Number(2022.0, NumberType::Decimal));

        Ok(())
    }
    
    #[test]
    fn delete_rule_2() ->  Result<(), ()> {
        let mut calculater = SmartCalc::default();
        let test1 = Rc::new(Test1::default());
        assert!(calculater.add_rule("en".to_string(), vec!["erhan {TEXT:surname:baris}".to_string(), "{TEXT:surname:baris} erhan".to_string()], test1.clone()));
        assert!(calculater.delete_rule("en".to_string(), test1.name().clone()));

        Ok(())
    }
    
    #[test]
    fn delete_rule_3() ->  Result<(), ()> {
        let mut calculater = SmartCalc::default();
        let test1 = Rc::new(Test1::default());
        assert!(!calculater.delete_rule("en".to_string(), test1.name().clone()));

        Ok(())
    }
    
    #[test]
    fn dynamic_type_1() ->  Result<(), ()> {
        let mut calculater = SmartCalc::default();
        assert!(calculater.add_dynamic_type("test1"));
        assert!(calculater.add_dynamic_type_item("test1", 1, "{value} a", vec!["{NUMBER:value} {TEXT:type:a}"], "{value} / 2", "{value} 2", vec!["a".to_string()], None, None, None));
        assert!(calculater.add_dynamic_type_item("test1", 2, "{value} b", vec!["{NUMBER:value} {TEXT:type:b}"], "{value} / 2", "{value} * 2", vec!["b".to_string()], None, None, None));
        assert!(calculater.add_dynamic_type_item("test1", 3, "{value} c", vec!["{NUMBER:value} {TEXT:type:c}"], "{value} / 2", "{value} * 2", vec!["c".to_string()], None, None, None));
        assert!(calculater.add_dynamic_type_item("test1", 4, "{value} d", vec!["{NUMBER:value} {TEXT:type:d}"], "{value}", "{value} * 2", vec!["d".to_string()], None, None, None));
        
        let result = calculater.execute("en".to_string(), "10 a to b");
        check_dynamic_type_output!(result, "test1", 5.0);

        let result = calculater.execute("en".to_string(), "1011 b to a");
        check_dynamic_type_output!(result, "test1", 2022.0);

        let result = calculater.execute("en".to_string(), "1 d to a");
        check_dynamic_type_output!(result, "test1", 8.0);

        let result = calculater.execute("en".to_string(), "8 a to d");
        check_dynamic_type_output!(result, "test1", 1.0);
        Ok(())
    }
    
    #[test]
    fn dynamic_type_2() ->  Result<(), ()> {
        let mut calculater = SmartCalc::default();
        assert!(calculater.add_dynamic_type("test1"));
        assert!(!calculater.add_dynamic_type("test1"));
        
        assert!(calculater.add_dynamic_type_item("test1", 1, "{value} a", vec!["{NUMBER:value} {TEXT:type:a}"], "{value} / 2", "{value} 2", vec!["a".to_string()], None, None, None));
        assert!(!calculater.add_dynamic_type_item("test1", 1, "{value} a", vec!["{NUMBER:value} {TEXT:type:a}"], "{value} / 2", "{value} 2", vec!["a".to_string()], None, None, None));
        Ok(())
    }

    #[test]
    fn basic_test_1() ->  anyhow::Result<()> {
        let config = SmartCalcConfig::default();
        let result = SmartCalc::basic_execute("1024", &config)?;
        assert_eq!(result, 1024.0);
        Ok(())
    }
    
    #[test]
    fn basic_test_2() ->  anyhow::Result<()> {
        let config = SmartCalcConfig::default();
        let result = SmartCalc::basic_execute("1024 * 2", &config)?;
        assert_eq!(result, 2048.0);
        Ok(())
    }
    
    #[test]
    fn basic_test_3() ->  anyhow::Result<()> {
        let config = SmartCalcConfig::default();
        let error = match SmartCalc::basic_execute("a + 1024 * 2", &config) {
            Ok(_) => return Ok(()),
            Err(error) => error
        };
        assert_eq!(error.to_string(), "Number not found".to_string());
        Ok(())
    }
    
    #[test]
    fn basic_test_4() ->  anyhow::Result<()> {
        let config = SmartCalcConfig::default();
        let error = match SmartCalc::basic_execute("+ 1024 * 2", &config) {
            Ok(_) => return Ok(()),
            Err(error) => error
        };
        assert_eq!(error.to_string(), "No more token".to_string());
        Ok(())
    }
    
    #[test]
    fn basic_test_5() ->  anyhow::Result<()> {
        let config = SmartCalcConfig::default();
        let error = match SmartCalc::basic_execute(r#"1+ 1024 * 2
"#, &config) {
            Ok(_) => return Ok(()),
            Err(error) => error
        };
        assert_eq!(error.to_string(), "Multiline calculation not supported".to_string());
        Ok(())
    }
    
    #[test]
    fn basic_test_6() ->  anyhow::Result<()> {
        let config = SmartCalcConfig::default();
        let error = match SmartCalc::basic_execute(r#""#, &config) {
            Ok(_) => return Ok(()),
            Err(error) => error
        };
        assert_eq!(error.to_string(), "Calculation empty".to_string());
        Ok(())
    }
    
    #[derive(Default)]
    pub struct Coin;

    impl RuleTrait for Coin {
        fn name(&self) -> String {
            "Coin".to_string()
        }

        fn call(&self, smartcalc: &SmartCalcConfig, fields: &BTreeMap<String, TokenType>) -> Option<TokenType> {
            let count = match fields.get("count") {
                Some(TokenType::Number(number, NumberType::Decimal)) => *number,
                _ => return None
            };
            let coin = match fields.get("coin") {
                Some(TokenType::Text(text)) => text.clone(),
                _ => return None
            };
            
            let price = match &coin[..] {
                "btc" => 1000.0 * count,
                "eth" => 800.0 * count,
                _ => return None
            };
            
            return Some(TokenType::Money(price, smartcalc.get_currency("usd".to_string()).unwrap()));
        }
    }
    
    #[test]
    fn add_rule_3() ->  Result<(), ()> {
        let mut calculater = SmartCalc::default();
        let test1 = Rc::new(Coin::default());
        calculater.add_rule("en".to_string(), vec!["{NUMBER:count} {TEXT:coin}".to_string()], test1.clone());
        let result = calculater.execute("en".to_string(), "10 btc to usd");
        check_basic_rule_output!(result, TokenType::Money(10000.0, calculater.config.get_currency("usd".to_string()).unwrap()));
        Ok(())
    }
    
    #[test]
    fn add_rule_4() ->  Result<(), ()> {
        let mut calculater = SmartCalc::default();
        let test1 = Rc::new(Coin::default());
        calculater.add_rule("en".to_string(), vec!["{NUMBER:count} {TEXT:coin}".to_string()], test1.clone());
        let result = calculater.execute("en".to_string(), "10 eth to usd");
        check_basic_rule_output!(result, TokenType::Money(8000.0, calculater.config.get_currency("usd".to_string()).unwrap()));
        Ok(())
    }
    
    #[test]
    fn add_rule_5() ->  Result<(), ()> {
        let mut calculater = SmartCalc::default();
        let test1 = Rc::new(Coin::default());
        calculater.add_rule("en".to_string(), vec!["{NUMBER:count} {TEXT:coin}".to_string()], test1.clone());
        let result = calculater.execute("en".to_string(), "10 eth to dkk");
        check_basic_rule_output!(result, TokenType::Money(49644.9970792, calculater.config.get_currency("dkk".to_string()).unwrap()));
        Ok(())
    }
}
