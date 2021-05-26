extern crate console_error_panic_hook;

use alloc::format;
use core::cell::RefCell;

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
    pub fn execute(&self, language: &str, data: &str) {
        let execute_result = self.smartcalc.borrow().execute(language, data);
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

