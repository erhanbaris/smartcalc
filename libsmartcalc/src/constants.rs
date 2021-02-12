use lazy_static::*;
use mut_static::MutStatic;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::btree_map::BTreeMap;
use regex::Regex;

use crate::worker::{rule::RuleLanguage};

pub static mut SYSTEM_INITED: bool = false;
lazy_static! {
    pub static ref CURRENCIES: MutStatic<BTreeMap<String, String>> = {
        let m = BTreeMap::new();
        MutStatic::from(m)
    };
    
    pub static ref CURRENCY_RATES: MutStatic<BTreeMap<String, f64>> = {
        let m = BTreeMap::new();
        MutStatic::from(m)
    };

    pub static ref TOKEN_PARSE_REGEXES: MutStatic<BTreeMap<String, Vec<Regex>>> = {
        let m = BTreeMap::new();
        MutStatic::from(m)
    };

    pub static ref WORD_GROUPS: MutStatic<BTreeMap<String, Vec<String>>> = {
        let m = BTreeMap::new();
        MutStatic::from(m)
    };

    pub static ref ALIAS_REGEXES: MutStatic<Vec<(Regex, String)>> = {
        let m = Vec::new();
        MutStatic::from(m)
    };

    pub static ref RULES: MutStatic<RuleLanguage> = {
        let m = RuleLanguage::new();
        MutStatic::from(m)
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
        "(?P<DECIMAL>[-+]?[0-9]+[0-9.,]{0,})(?P<NOTATION>[kKMGTPZY]{0,1})"
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
            "hour_add": ["{TIME:time} add {NUMBER:hour} {GROUP:hour_group}"],
            "time_for_location": ["time in {TEXT:location}", "time at {TEXT:location}", "time for {TEXT:location}"],
            "convert_money": ["{MONEY:money} {GROUP:conversion_group} {TEXT:curency}", "{MONEY:money} {TEXT:curency}"]
        }
    },

    "word_group": {
        "hour_group": ["hour", "hours"],
        "week_group": ["week", "weeks"],
        "conversion_group": ["in", "into", "as", "to"]
    },

  "alias": {
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
    "×": "[OPERATOR:*]",

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
    "kroner": "dkk",

    "bgn": "bgn",
    "leva": "bgn",
    "lef": "bgn",
    "лв": "bgn",

    "eur": "eur",
    "euro": "eur",
    "avo": "eur",
    "€": "eur"
  },
  "currency_rates": {
    "hkd": 7.7526495869,
    "isk": 129.2664608195,
    "php": 48.1023116081,
    "dkk": 6.2056246349,
    "huf": 297.5715597096,
    "czk": 21.5355086372,
    "gbp": 0.7305182342,
    "ron": 4.0680130184,
    "sek": 8.4476341484,
    "idr": 14036.752065426,
    "inr": 72.9091212551,
    "brl": 5.4450471501,
    "rub": 74.7997162647,
    "hrk": 6.3100225319,
    "jpy": 105.749812234,
    "thb": 30.1001418676,
    "chf": 0.9033630977,
    "eur": 0.8345155637,
    "myr": 4.0705165651,
    "bgn": 1.6321455395,
    "try": 7.0727697572,
    "cny": 6.4704164233,
    "nok": 8.6011850121,
    "nzd": 1.3999833097,
    "zar": 14.9717933739,
    "usd": 1.0,
    "mxn": 20.3196194609,
    "sgd": 1.3379788033,
    "aud": 1.31527998,
    "ils": 3.2926646082,
    "krw": 1122.7989652007,
    "pln": 3.7572394225
  }
}"#;