#![no_std]
extern crate alloc;
extern crate lazy_static;
extern crate log;

#[cfg(all(not(target_arch = "wasm32"), not(test)))]
extern crate libc_print;

pub mod types;
pub mod tokinizer;
pub mod syntax;
pub mod worker;
pub mod compiler;
pub mod constants;
pub mod tools;
pub mod logger;
pub mod formatter;
pub mod token;
pub mod config;
pub mod app;

pub mod executer;

#[cfg(target_arch = "wasm32")]
pub mod web;

