use lazy_static::*;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    pub static ref CURRENCIES: Mutex<HashMap<String, String>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

pub const JSON_DATA: &str = r#"{
  "parse":  {
    "time": [
      "(?P<hour>1[0-2]|0?[1-9]):(?P<minute>[0-5][0-9]):(?P<second>[0-5][0-9]) ?(?P<meridiem>[AaPp][Mm])",
      "(?P<hour>1[0-2]|0?[1-9]):(?P<minute>[0-5][0-9]) ?(?P<meridiem>[AaPp][Mm])",
      "(?P<hour>[0-1]?[0-9]|2[0-3]):(?P<minute>[0-5][0-9]):(?P<second>[0-5][0-9])",
      "(?P<hour>[0-1]?[0-9]|2[0-3]):(?P<minute>[0-5][0-9])"
    ],
    "money": [
        "(?P<CURRENCY>\\\\p\\{Sc\\})(?P<PRICE>[-+]?[0-9]+[0-9.,]{0,})",
        "(?P<PRICE>[-+]?[0-9]+[0-9.,]{0,})[ ]*(?P<CURRENCY>[a-zA-Z]{2,3})",
        "(?P<PRICE>[-+]?[0-9]+[0-9.,]{0,})[ ]*(?P<CURRENCY>\\\\p\\{Sc\\})"
    ],
    "number": [
        "(?P<NUMBER>[-+]?[0-9]+[0-9.,]{0,})"
    ]
  },

  "rules": {
    "percent_calculator": ["{PERCENT:percent} {NUMBER:number}", "{NUMBER:number} {PERCENT:percent}"],
    "hour_add": ["{TIME:time} add {NUMBER:hours} hour"],
    "date_add": ["{DATE:date}\"e {NUMBER:day} gün ekle"],
    "time_for_location": ["time in {TEXT:location}", "time at {TEXT:location}", "time for {TEXT:location}"]
  },

  "alias": {
    "at": "in",
    "for": "in",
    "hours": "hour",
    "günler": "gün",

    "_": "",
    ";": "",
    "!": "",
    "?": "",
    "'": "",
    "&": "",
    "^": "",

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