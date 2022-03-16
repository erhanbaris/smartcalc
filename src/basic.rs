/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use core::borrow::Borrow;
use alloc::rc::Rc;
use core::ops::Deref;
use alloc::string::{String, ToString};
use crate::Session;

use crate::compiler::Interpreter;
use crate::logger::{LOGGER, initialize_logger};
use crate::syntax::SyntaxParser;
use crate::tokinizer::Tokinizer;
use crate::types::SmartCalcAstType;
use crate::config::SmartCalcConfig;

pub struct BasicSmartCalc {
    config: SmartCalcConfig
}

impl Default for BasicSmartCalc {
    fn default() -> Self {
        initialize_logger();
        BasicSmartCalc {
            config: SmartCalcConfig::default()
        }
    }
}

impl BasicSmartCalc {
    pub fn initialize() {
        if log::set_logger(&LOGGER).is_ok() {
            if cfg!(debug_assertions) {
                log::set_max_level(log::LevelFilter::Debug)
            } else {
                log::set_max_level(log::LevelFilter::Info)
            }
        }
    }

    pub(crate) fn execute_text(&self, session: &Session) -> Result<f64, String> {
        log::debug!("> {}", session.current_line());
        if session.current_line().is_empty() {
            return Err("Calculation empty".to_string());
        }

        let mut tokinizer = Tokinizer::new(&self.config, session);
        if !tokinizer.tokinize() {
            return Err("Syntax error".to_string());
        }

        let mut syntax = SyntaxParser::new(session, &tokinizer);
        log::debug!(" > parse starting");

        match syntax.parse() {
            Ok(ast) => {
                log::debug!(" > parse Ok {:?}", ast);
                let ast_rc = Rc::new(ast);

                match Interpreter::execute(&self.config, ast_rc, session) {
                    Ok(ast) => {
                        match ast.deref() {
                            SmartCalcAstType::Item(item) => Ok(item.get_underlying_number()),
                            _ => Err("Number not found".to_string())
                        }
                    },
                    Err(error) => Err(error)
                }
            },
            Err((error, _, _)) => {
                log::debug!(" > parse Err");
                log::info!("Syntax parse error, {}", error);
                Err(error.to_string())
            }
        }
    }

    pub fn execute<Tlan: Borrow<str>, Tdata: Borrow<str>>(&self, language: Tlan, data: Tdata) ->  Result<f64, String> {
        let mut session = Session::new();

        session.set_text(data.borrow().to_string());
        session.set_language(language.borrow().to_string());
        
        if session.line_count() != 1 {
            return Err("Multiline calculation not supported".to_string());
        }
        
        match session.has_value() {
            true => self.execute_text(&session),
            false => Err("No data found".to_string())
        }
    }
}
