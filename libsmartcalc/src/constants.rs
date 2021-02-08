use lazy_static::*;
use std::collections::HashMap;
use std::sync::{Mutex};
use regex::Regex;

use crate::worker::{rule::RuleLanguage};

pub static mut SYSTEM_INITED: bool = false;
lazy_static! {
    pub static ref CURRENCIES: Mutex<HashMap<String, String>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };

    pub static ref TOKEN_PARSE_REGEXES: Mutex<HashMap<String, Vec<Regex>>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };

    pub static ref ALIAS_REGEXES: Mutex<Vec<(Regex, String)>> = {
        let m = Vec::new();
        Mutex::new(m)
    };

    pub static ref RULES: Mutex<RuleLanguage> = {
        let m = RuleLanguage::new();
        Mutex::new(m)
    };
}

pub const JSON_DATA: &str = r#"{
  "parse":  {
    "percent": [
        "(?P<NUMBER>[-+]?[0-9]+([,\\.][0-9]+){0,})%",
        "%(?P<NUMBER>[-+]?[0-9]+([,\\.][0-9]+){0,})"
    ],
    "time": [
        "(?P<hour>1[0-2]|0?[1-9]):(?P<minute>[0-5][0-9]):(?P<second>[0-5][0-9]) ?(?P<meridiem>[AaPp][Mm])",
        "(?P<hour>1[0-2]|0?[1-9]):(?P<minute>[0-5][0-9]) ?(?P<meridiem>[AaPp][Mm])",
        "(?P<hour>[0-1]?[0-9]|2[0-3]):(?P<minute>[0-5][0-9]):(?P<second>[0-5][0-9])",
        "(?P<hour>[0-1]?[0-9]|2[0-3]):(?P<minute>[0-5][0-9])"
    ],
    "money": [
        "(?P<CURRENCY>\\p{Currency_Symbol})(?P<PRICE>[-+]?[0-9]+[0-9.,]{0,})",
        "(?P<PRICE>[-+]?[0-9]+[0-9.,]{0,})[ ]*(?P<CURRENCY>[a-zA-Z]{2,3})",
        "(?P<PRICE>[-+]?[0-9]+[0-9.,]{0,})[ ]*(?P<CURRENCY>\\p{Currency_Symbol})"
    ],
    "number": [
        "0[xX](?P<HEX>[0-9a-fA-F]+)",
        "0[oO](?P<OCTAL>[0-7]+)",
        "0[bB](?P<BINARY>[01]+)",
        "(?P<DECIMAL>[-+]?[0-9]+[0-9.,]{0,})"
    ],
    "text": [
        "(?P<TEXT>[\\p{L}]+)"
    ],
    "field": [
        "(\\{(?P<FIELD>[A-Z]+):(?P<NAME>[^}]+)\\})"
    ],
    "atom": [
        "(\\[(?P<ATOM>[A-Z]+):(?P<DATA>[^\\]]+)\\])"
    ],
    "whitespace": [
        "(?P<WHITESPACE>[ ]+)"
    ],
    "operator": [
        "(?P<OPERATOR>[^0-9\\p{L} ])"
    ]
  },

    "rules": {
        "en": {
            "percent_calculator": ["{PERCENT:p} {NUMBER:number}", "{NUMBER:number} {PERCENT:p}"],
            "hour_add": ["{TIME:time} add {NUMBER:hour} hour"],
            "date_add": ["{DATE:date}\"e {NUMBER:day} gün ekle"],
            "time_for_location": ["time in {TEXT:location}", "time at {TEXT:location}", "time for {TEXT:location}"]
        }
    },

  "alias": {
    "at": "in",
    "for": "in",
    "hours": "hour",
    "günler": "gün",

    "_": "",
    ";": "",
    "!": "",
    "\\?": "",
    "'": "",
    "&": "",
    "\\^": "",

    "times": "[OPERATOR:*]",
    "multiply": "[OPERATOR:*]",
    "x": "[OPERATOR:*]",

    "add": "[OPERATOR:+]",
    "sum": "[OPERATOR:+]",
    "append": "[OPERATOR:+]",

    "exclude": "[OPERATOR:-]",
    "minus": "[OPERATOR:-]",

    "percent": "[OPERATOR:%]",
    "percentage": "[OPERATOR:%]"
  },
  "currencies" : {
    "try": "try",
    "tl": "try",
    "₺": "try",

    "$": "usd",
    "usd": "usd",
    "dollar": "usd",

    "dkk": "dkk",
    "kr": "dkk",
    "kroner": "dkk"
  }
}"#;