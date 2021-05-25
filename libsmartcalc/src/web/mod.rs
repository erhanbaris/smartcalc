extern crate console_error_panic_hook;

use alloc::format;
use alloc::string::ToString;
use alloc::string::String;

use crate::executer::execute;
use crate::types::BramaAstType;
use crate::executer::initialize;
use crate::formatter::format_result;
use crate::worker::tools::{read_currency};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
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
pub fn update_currency(currency: &str, rate: f64, callback: &js_sys::Function) {
    match read_currency(currency) {
        Some(real_currency) => {
            CURRENCY_RATES.write().unwrap().insert(real_currency.to_string(), rate);
        },
         _ => return
    };

    let arguments = js_sys::Array::new();
    arguments.push(&JsValue::from(format!("Currency({}) rate updated", currency)));
    callback.apply(&JsValue::null(), &arguments).unwrap();
}
#[wasm_bindgen]
pub fn initialize_system() {
    initialize();
}

#[wasm_bindgen]
pub fn process(language: String, data: String, callback: &js_sys::Function) {
    initialize();
    use js_sys::*;

    /* JS referance object */
    let status_ref      = JsValue::from("status");
    let result_type_ref = JsValue::from("type");
    let text_ref        = JsValue::from("text");
    let tokens_ref      = JsValue::from("tokens");

    match FORMATS.read().unwrap().get(&language) {
        Some(formats) => {
            let line_items = js_sys::Array::new();
            for result in execute(&formats.language, &data) {

                let line_object = js_sys::Object::new();
                match result {
                    Ok((tokens, ast)) => {
                        let (status, result_type, output) = match &*ast {
                            BramaAstType::Number(_) => (true, 1, format_result(formats, ast.clone())),
                            BramaAstType::Time(_) => (true, 2, format_result(formats, ast.clone())),
                            BramaAstType::Percent(_) => (true, 3, format_result(formats, ast.clone())),
                            BramaAstType::Money(_, _) => (true, 4, format_result(formats, ast.clone())),
                            BramaAstType::Duration(_) => (true, 5, format_result(formats, ast.clone())),
                            BramaAstType::Date(_) => (true, 6, format_result(formats, ast.clone())),
                            _ => (false, 0, "".to_string())
                        };

                        Reflect::set(line_object.as_ref(), status_ref.as_ref(),      JsValue::from(status).as_ref()).unwrap();
                        Reflect::set(line_object.as_ref(), result_type_ref.as_ref(), JsValue::from(result_type).as_ref()).unwrap();
                        Reflect::set(line_object.as_ref(), text_ref.as_ref(),        JsValue::from(&output[..]).as_ref()).unwrap();

                        /* Token generation */
                        let token_objects = js_sys::Array::new();
                        for token in tokens.iter() {
                            token_objects.push(&token.as_js_object().into());
                        }
                        Reflect::set(line_object.as_ref(), tokens_ref.as_ref(),      token_objects.as_ref()).unwrap();
                    },
                    Err(error) => {
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
        },
        _ => return
    };
}