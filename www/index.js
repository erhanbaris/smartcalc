import * as wasm from "smartcalc";

function process() {
    wasm.process(window.editor.getValue(), function (results) {
        console.log(results);
        window.output.getDoc().setValue(results.join("\r\n"));
    })
}
window.process = process;