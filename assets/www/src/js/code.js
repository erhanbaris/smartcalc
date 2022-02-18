import { SmartCalcWeb, default as init } from './libsmartcalc.js';
import language from './language.js';

const DEFAULT_LANGUAGE = 'en';

var typeColors = {
    0: "",
    1: "number-token",
    2: "money-symbol-token",
    4: "operator-token",
    5: "text-token",
    7: "number-token",
    8: "variable-token",
    9: "comment-token",
    10: "money-symbol-token",
    11: "variable-use-token",
    12: "variable-defination-token"
};

function makeMarker(msg) {
    const marker = document.createElement('div');
    marker.classList.add('error-marker');

    const error = document.createElement('div');
    error.innerHTML = msg;
    error.classList.add('error-message');
    marker.appendChild(error);

    return marker;
}

function getNumberSeparators() {
  
    // default
    var res = {
        "decimal": ".",
        "thousand": ""
    };

    // convert a number formatted according to locale
    var str = parseFloat(1234.56).toLocaleString();

    // if the resulting number does not contain previous number
    // (i.e. in some Arabic formats), return defaults
    if (!str.match("1"))
        return res;

    // get decimal and thousand separators
    res.decimal = str.replace(/.*4(.*)5.*/, "$1");
    res.thousand = str.replace(/.*1(.*)2.*/, "$1");

    // return results
    return res;
}

await init('./src/js/libsmartcalc_bg.wasm');
const separators = getNumberSeparators();
const calculator = SmartCalcWeb.default(separators.decimal, separators.thousand);

new Vue({
    el: '#app',
    data: function() {
        return {
            currency_updating: false,
            last_currency_update: new Date(),
            current_status: "-",
            timer: null,
            language: language[DEFAULT_LANGUAGE]
        }
    },
    created() {
        var that = this;
        setTimeout(function() {
            var language_key = localStorage.getItem("language");
            if (language_key != null) {
                this.language[language_key];
            }

            var code = localStorage.getItem("code");
            if (code == null) {
                code = `tomorrow + 3 weeks
3/3/2021 to 3/3/2000
12/02/2020 - 11680 days
jan 28, 2019 - 14 months 33 days
3:35 am + 7 hours 15 minutes

date information = 11:30
date information add 1 hour 1 minute 30 second

8 / (45 - 20%)

10% of 200 try
180 is 10% of what

10% off 200

10 * 20 + 40

22250mb - 250.1mb
1024mb + (1024kb * 24)

$1k earninng / 5 people`
            }

            window.editor = CodeMirror(document.getElementById("source"), {
                value: code,
                mode: "smartcalc",
                theme: "ayu-mirage",
                styleSelectedText: true,
                styleActiveLine: true,
                electricChars: false,
                smartIndent: false,
                scrollbarStyle: 'null',
                gutters: [{
                    'className': 'error',
                    'style': 'width: 300px; background-color: #363e50;'
                }]
            });

            window.editor.setSize(null, null);
            window.editor.on("change", that.editor_change);

            that.editor_change();
            that.update_currencies();
            that.start_timer();

            $(".dropdown").change(that.language_changes);
            $('.ui.dropdown').dropdown();

        }, 100);
    },
    methods: {
        start_timer() {
            var that = this;
            this.timer = setInterval(() => {
                that.current_status = moment(that.last_currency_update).fromNow();
            }, 1000)
        },
        translate: function(key) {
            return this.language[key];
        },
        language_changes: function() {
            moment.locale($("#language").val());
            this.language = language[$("#language").val()];
            localStorage.setItem("language", $("#language").val());
            this.editor_change();
        },
        editor_change: function() {
            window.editor.clearGutter('error');

            var marks = window.editor.getAllMarks();
            marks.forEach(function(marker) {
                marker.clear();
            });

            let code = window.editor.getValue();
            let results = calculator.execute($("#language").val(), code);
            localStorage.setItem("code", code);

            var result_texts = [];
            for (var i = 0; i < results.length; ++i) {
                if (!results[i].status) {
                    window.editor.setGutterMarker(i, 'error', makeMarker(""));
                    result_texts.push("");
                    continue;
                }

                window.editor.setGutterMarker(i, 'error', makeMarker(results[i].output));
                result_texts.push(results[i].output);
                for (var j = 0; j < results[i].tokens.length; ++j) {
                    window.editor.markText({ line: i, ch: results[i].tokens[j].start }, { line: i, ch: results[i].tokens[j].end }, { className: typeColors[results[i].tokens[j].type] });
                }
            }
        },
        update_currencies: function() {
            var that = this;
            that.currency_updating = true;
            var currenciyRatesApi = "http://www.floatrates.com/daily/usd.json";
            $.getJSON(currenciyRatesApi, {
                    tagmode: "any",
                    format: "json"
                })
                .done(function(currencies) {
                    that.last_currency_update = new Date();
                    Object.keys(currencies).forEach(function(currency) {
                        calculator.update_currency(currency, currencies[currency].rate, function() {});
                    });
                    that.currency_updating = false;
                });
        }
    }
});