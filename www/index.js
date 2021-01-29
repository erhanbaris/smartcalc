import * as wasm from "smartcalc";

function process() {
    wasm.process(window.editor.getValue(), function (results) {
        console.log(results);
        var result_texts = [];
        for (var i = 0; i < results.length; ++i)
            result_texts.push(results[i].text);
        window.output.getDoc().setValue(result_texts.join("\r\n"));
    })
}

function execute_codes(code, callback) {
    wasm.process(code, function (results) {
        console.log(results);
        try {
            callback(results);
        }
        catch(err) {
            console.log(err);
        }
    })
}

window.process = process;
window.execute_codes = execute_codes;