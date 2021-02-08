extern crate console_error_panic_hook;

use crate::executer::execute;
use crate::types::BramaAstType;
use crate::tokinizer::{TokenLocationStatus};
use crate::executer::initialize;

use std::panic;
use wasm_bindgen::prelude::*;
use serde_json::value::Value::Array;
use serde_json::{Value, Number};

fn my_init_function() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn process(data: String, callback: &js_sys::Function) {
    initialize();
    use js_sys::*;

    /* JS referance object */
    let status_ref      = JsValue::from("status");
    let result_type_ref = JsValue::from("type");
    let text_ref        = JsValue::from("text");
    let tokens_ref      = JsValue::from("tokens");

    //log("1");
    let line_items = js_sys::Array::new();
    for result in execute(&data, &"en".to_string()) {
        //log("2");

        let line_object = js_sys::Object::new();
        match result {
            Ok((tokens, ast)) => {
                //log("3.1");

                let (status, result_type, output) = match &*ast {
                    BramaAstType::Number(number) => (true, 1, number.to_string()),
                    BramaAstType::Time(time) => (true, 2, time.to_string()),
                    BramaAstType::Percent(percent) => (true, 3, format!("%{}", percent.to_string())),
                    BramaAstType::Money(price, currency) => (true, 4, format!("{:.2}{}", price, currency.to_uppercase())),
                    _ => (false, 0, "".to_string())
                };

                Reflect::set(line_object.as_ref(), status_ref.as_ref(),      JsValue::from(status).as_ref()).unwrap();
                Reflect::set(line_object.as_ref(), result_type_ref.as_ref(), JsValue::from(result_type).as_ref()).unwrap();
                Reflect::set(line_object.as_ref(), text_ref.as_ref(),        JsValue::from(&output[..]).as_ref()).unwrap();

                /* Token generation */
                let token_objects = js_sys::Array::new();
                for token in tokens.iter() {
                    if token.status == TokenLocationStatus::Active {
                        token_objects.push(&token.as_js_object().into());
                    }
                }
                Reflect::set(line_object.as_ref(), tokens_ref.as_ref(),      token_objects.as_ref()).unwrap();
            },
            Err(error) => {
                //log("3.2");
                Reflect::set(line_object.as_ref(), status_ref.as_ref(),      JsValue::from(false).as_ref()).unwrap();
                Reflect::set(line_object.as_ref(), result_type_ref.as_ref(), JsValue::from(0).as_ref()).unwrap();
                Reflect::set(line_object.as_ref(), text_ref.as_ref(),        JsValue::from(&error[..]).as_ref()).unwrap();
            }
        };

        line_items.push(&line_object.into());
    }

    let arguments = js_sys::Array::new();
    arguments.push(&line_items);
    callback.apply(&JsValue::null(), &arguments).unwrap();
}