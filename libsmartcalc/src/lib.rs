#![no_std]
extern crate alloc;
extern crate lazy_static;
#[cfg(target_arch = "wasm32")]
extern crate wee_alloc;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod types;
pub mod tokinizer;
pub mod syntax;
pub mod worker;
pub mod compiler;
pub mod constants;

pub mod executer;

#[cfg(target_arch = "wasm32")]
pub mod web;

