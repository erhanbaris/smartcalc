/*
 * smartcalc v1.0.7
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::borrow::ToOwned;
use regex::Regex;
use crate::config::SmartCalcConfig;
use crate::tokinizer::Tokinizer;

pub fn whitespace_regex_parser(_: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            tokinizer.add_token_from_match(&capture.get(0), None);
        }
    }
}

#[cfg(test)]
#[test]
fn whitespace_test_1() {
    use crate::tokinizer::regex_tokinizer;
    use crate::tokinizer::test::setup_tokinizer;
    use alloc::string::ToString;
    use crate::config::SmartCalcConfig;
    use crate::session::Session;
    let mut session = Session::new();
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("                                          ".to_string(), &mut session, &config);

    regex_tokinizer(&mut tokinizer_mut);
    assert_eq!(tokinizer_mut.token_infos.len(), 0);
}
