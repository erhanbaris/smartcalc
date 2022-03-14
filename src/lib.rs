/*
 * smartcalc v1.0.5
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

#![no_std]
extern crate alloc;
extern crate lazy_static;
extern crate log;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(all(not(target_arch = "wasm32"), not(test)))]
extern crate libc_print;

pub(crate) mod types;
pub(crate) mod tokinizer;
pub(crate) mod syntax;
pub(crate) mod compiler;
pub(crate) mod constants;
pub(crate) mod tools;
pub(crate) mod logger;
pub(crate) mod formatter;
pub(crate) mod token;
pub(crate) mod config;
pub(crate) mod app;
pub(crate) mod variable;
pub(crate) mod session;

#[cfg(test)]
mod tests;

pub use app::SmartCalc;
pub use session::Session;
pub use config::SmartCalcConfig;
pub use types::SmartCalcAstType;
pub use types::FieldType;
pub use compiler::DataItem;
pub use token::ui_token::UiToken;
pub use token::ui_token::UiTokenType;
pub use app::RuleFunc;
pub use types::TokenType;
pub use types::NumberType;