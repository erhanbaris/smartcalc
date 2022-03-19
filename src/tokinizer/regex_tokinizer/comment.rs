/*
 * smartcalc v1.0.7
 * Copyright (c) Erhan BARIS (Ruslan Ognyanov Asenov)
 * Licensed under the GNU General Public License v2.0.
 */

use alloc::borrow::ToOwned;
use regex::Regex;
use crate::config::SmartCalcConfig;
use crate::tokinizer::Tokinizer;
use crate::token::ui_token::UiTokenType;

pub fn comment_regex_parser(_: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            if tokinizer.add_token_from_match(&capture.get(0), None) {
                tokinizer.add_uitoken_from_match(capture.get(0), UiTokenType::Comment);
            }
        }
    }
}

#[cfg(test)]
#[test]
fn comment_test_1() {
    use alloc::string::ToString;
    use crate::tokinizer::regex_tokinizer;
    use crate::tokinizer::test::setup_tokinizer;
    use crate::config::SmartCalcConfig;
    use crate::session::Session;
    let mut session = Session::new();
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("#123".to_string(), &mut session, &config);

    regex_tokinizer(&mut tokinizer_mut);
    assert_eq!(tokinizer_mut.ui_tokens.len(), 1);
}

#[cfg(test)]
#[test]
fn comment_test_2() {
    use crate::alloc::string::ToString;
    use crate::tokinizer::regex_tokinizer;
    use crate::tokinizer::test::setup_tokinizer;
    use crate::config::SmartCalcConfig;
    use crate::session::Session;
    let mut session = Session::new();
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("#".to_string(), &mut session, &config);

    regex_tokinizer(&mut tokinizer_mut);
    assert_eq!(tokinizer_mut.ui_tokens.len(), 1);
}

#[cfg(test)]
#[test]
fn comment_test_3() {
    use crate::alloc::string::ToString;
    use crate::tokinizer::regex_tokinizer;
    use crate::tokinizer::test::setup_tokinizer;
    use crate::config::SmartCalcConfig;
    use crate::session::Session;
    let mut session = Session::new();
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("# 111".to_string(), &mut session, &config);

    regex_tokinizer(&mut tokinizer_mut);
    assert_eq!(tokinizer_mut.ui_tokens.len(), 1);
}
