#[macro_use]
extern crate lazy_static;

use std::panic;
mod types;
mod tokinizer;
mod syntax;
mod worker;
mod executer;
mod compiler;

use executer::execute;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let test_data = r"tarih = 10:30
tarih add 12 hour
".to_string();

    println!("{:?}", execute(&test_data, &"en".to_string()));
}


#[cfg(target_arch = "wasm32")]
extern crate console_error_panic_hook;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::types::BramaAstType;

#[cfg(target_arch = "wasm32")]
use serde_json::value::Value::Array;

#[cfg(target_arch = "wasm32")]
use serde_json::{Value, Number};

#[cfg(target_arch = "wasm32")]
fn my_init_function() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn process(data: String, callback: &js_sys::Function) {
    let result_items = js_sys::Array::new();
    for result in execute(&data, &"en".to_string()) {
        let item_text = match result {
            Ok(ast) => {
                match &*ast {
                    BramaAstType::Number(number) => number.to_string(),
                    BramaAstType::Time(time) => time.to_string(),
                    BramaAstType::Percent(percent) => format!("%{}", percent.to_string()),
                    _ => "".to_string()
                }
            },
            Err(error) => error
        };
        result_items.push(&item_text.into());
    }

    let arguments = js_sys::Array::new();
    arguments.push(&result_items);
    callback.apply(&JsValue::null(), &arguments).unwrap();
}

/*
cargo build --target wasm32-unknown-unknown --release
wasm-pack build
cd www/
npm run start
*/
