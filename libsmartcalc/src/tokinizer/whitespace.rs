use alloc::string::ToString;
use alloc::borrow::ToOwned;
use regex::Regex;
use crate::config::SmartCalcConfig;
use crate::tokinizer::Tokinizer;

pub fn whitespace_regex_parser(_: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), None, capture.get(0).unwrap().as_str().to_string());
        }
    }
}

#[cfg(test)]
#[test]
fn whitespace_test_1() {
    use crate::tokinizer::test::setup_tokinizer;
    use core::cell::RefCell;
    use crate::config::SmartCalcConfig;
    use crate::app::Session;
    let session = RefCell::new(Session::new());
    let config = SmartCalcConfig::default();
    let mut tokinizer_mut = setup_tokinizer("                                          ".to_string(), &session, &config);

    tokinizer_mut.tokinize_with_regex();
    assert_eq!(tokinizer_mut.session.borrow().token_infos.len(), 0);
}
