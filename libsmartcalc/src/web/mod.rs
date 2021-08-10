extern crate console_error_panic_hook;

use alloc::format;
use core::cell::RefCell;
use alloc::string::ToString;
use crate::types::BramaAstType;
use js_sys::*;

use crate::app::SmartCalc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct SmartCalcWeb {
    smartcalc: RefCell<SmartCalc>
}

#[wasm_bindgen]
impl SmartCalcWeb {
    #[wasm_bindgen]
    pub fn default() -> Self {
        SmartCalcWeb {
            smartcalc: RefCell::new(SmartCalc::default())
        }
    }
    
    #[wasm_bindgen]
    pub fn load_from_json(json_data: &str) -> Self {
        SmartCalcWeb {
            smartcalc: RefCell::new(SmartCalc::load_from_json(json_data))
        }
    }

    #[wasm_bindgen]
    pub fn execute(&self, language: &str, data: &str) -> JsValue {
        let status_ref      = JsValue::from("status");
        let result_type_ref = JsValue::from("type");
        let text_ref        = JsValue::from("output");
        let tokens_ref      = JsValue::from("tokens");

        let line_items = js_sys::Array::new();
        let execute_result = self.smartcalc.borrow().execute(language, data);
        for result in execute_result.lines {
            let line_object = js_sys::Object::new();
            match result {
                Some(result) =>
                    match result {
                        Ok(line_result) => {
                            let (status, result_type, output) = match &*line_result.ast {
                                BramaAstType::Number(_) => (true, 1, self.smartcalc.borrow().format_result(language, line_result.ast.clone())),
                                BramaAstType::Time(_) => (true, 2, self.smartcalc.borrow().format_result(language, line_result.ast.clone())),
                                BramaAstType::Percent(_) => (true, 3, self.smartcalc.borrow().format_result(language, line_result.ast.clone())),
                                BramaAstType::Money(_, _) => (true, 4, self.smartcalc.borrow().format_result(language, line_result.ast.clone())),
                                BramaAstType::Duration(_) => (true, 5, self.smartcalc.borrow().format_result(language, line_result.ast.clone())),
                                BramaAstType::Date(_) => (true, 6, self.smartcalc.borrow().format_result(language, line_result.ast.clone())),
                                _ => (false, 0, "".to_string())
                            };

                            Reflect::set(line_object.as_ref(), status_ref.as_ref(),      JsValue::from(status).as_ref()).unwrap();
                            Reflect::set(line_object.as_ref(), result_type_ref.as_ref(), JsValue::from(result_type).as_ref()).unwrap();
                            Reflect::set(line_object.as_ref(), text_ref.as_ref(),        JsValue::from(&output[..]).as_ref()).unwrap();

                            /* Token generation */
                            let token_objects = js_sys::Array::new();
                            for token in line_result.tokens.iter() {
                                token_objects.push(&token.as_js_object().into());
                            }
                            Reflect::set(line_object.as_ref(), tokens_ref.as_ref(),      token_objects.as_ref()).unwrap();
                        },
                        Err(error) => {
                            Reflect::set(line_object.as_ref(), status_ref.as_ref(),      JsValue::from(false).as_ref()).unwrap();
                            Reflect::set(line_object.as_ref(), result_type_ref.as_ref(), JsValue::from(0).as_ref()).unwrap();
                            Reflect::set(line_object.as_ref(), text_ref.as_ref(),        JsValue::from(&error[..]).as_ref()).unwrap();
                        }
                },

                None => {
                    Reflect::set(line_object.as_ref(), status_ref.as_ref(),      JsValue::from(false).as_ref()).unwrap();
                    Reflect::set(line_object.as_ref(), result_type_ref.as_ref(), JsValue::from(0).as_ref()).unwrap();
                    Reflect::set(line_object.as_ref(), text_ref.as_ref(),        JsValue::from("").as_ref()).unwrap();
                }
            }
            line_items.push(&line_object.into());
        }

        line_items.into()
    }

    #[wasm_bindgen]
    pub fn update_currency(&self, currency: &str, rate: f64, callback: &js_sys::Function) {
        self.smartcalc.borrow_mut().update_currency(currency, rate);
    
        let arguments = js_sys::Array::new();
        arguments.push(&JsValue::from(format!("Currency({}) rate updated", currency)));
        callback.apply(&JsValue::null(), &arguments).unwrap();
    }
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

