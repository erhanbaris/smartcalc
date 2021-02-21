import * as wasm from "smartcalc";

function execute_codes(code, callback) {
    wasm.process(code, function(results) {
        //console.log(results);
        try {
            callback(results);
        } catch (err) {
            console.log(err);
        }
    });
}

function update_currency(currency, rate) {
    wasm.update_currency(currency, rate, console.log);
}

wasm.initialize_system();
window.update_currency = update_currency;
window.process = process;
window.execute_codes = execute_codes;