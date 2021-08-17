use alloc::string::ToString;
use alloc::borrow::ToOwned;
use regex::Regex;
use crate::config::SmartCalcConfig;
use crate::{types::*};
use crate::tokinizer::Tokinizer;
use crate::token::ui_token::UiTokenType;

pub fn operator_regex_parser(_: &SmartCalcConfig, tokinizer: &mut Tokinizer, group_item: &[Regex]) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            if tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), Some(TokenType::Operator(capture.get(0).unwrap().as_str().chars().next().unwrap())), capture.get(0).unwrap().as_str().to_string())  {
                tokinizer.ui_tokens.add_from_regex_match(capture.get(0), UiTokenType::Operator);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{executer::initialize, types::*};
    use crate::tokinizer::Tokinizer;
    use alloc::string::ToString;
    use alloc::vec::Vec;

    #[cfg(test)]
    #[test]
    fn operator_test_1() {
        use crate::token::ui_token::UiTokenCollection;
        use crate::config::SmartCalcConfig;
        let config = SmartCalcConfig::default();
        let data = " - merhaba".to_string();
        let mut tokinizer = Tokinizer {
            column: 0,
            tokens: Vec::new(),
            iter: data.chars().collect(),
            data: data.to_string(),
            index: 0,
            indexer: 0,
            total: data.chars().count(),
            token_infos: Vec::new(),
            ui_tokens: UiTokenCollection::new(&data),
            language: "en",
            config: &config
        };
        initialize();
        tokinizer.tokinize_with_regex();

        assert_eq!(tokinizer.token_infos.len(), 2);

        assert_eq!(tokinizer.token_infos[0].start, 1);
        assert_eq!(tokinizer.token_infos[0].end, 2);
        assert_eq!(tokinizer.token_infos[0].token_type, Some(TokenType::Operator('-')));
        
        assert_eq!(tokinizer.token_infos[1].start, 3);
        assert_eq!(tokinizer.token_infos[1].end, 10);
        assert_eq!(tokinizer.token_infos[1].token_type, Some(TokenType::Text("merhaba".to_string())));
    }

    #[cfg(test)]
    #[test]
    fn operator_test_2() {
        use crate::token::ui_token::UiTokenCollection;
        use crate::tokinizer::Tokinizer;
        use crate::config::SmartCalcConfig;

        use alloc::string::ToString;
        use alloc::vec::Vec;
        let config = SmartCalcConfig::default();
        let data = "- ' * ` /,".to_string();
        let mut tokinizer = Tokinizer {
            column: 0,
            tokens: Vec::new(),
            iter: data.chars().collect(),
            data: data.to_string(),
            index: 0,
            indexer: 0,
            total: data.chars().count(),
            token_infos: Vec::new(),
            ui_tokens: UiTokenCollection::new(&data),
            language: "en",
            config: &config
        };
        initialize();

        tokinizer.tokinize_with_regex();

        assert_eq!(tokinizer.token_infos.len(), 6);
        assert_eq!(tokinizer.token_infos[0].start, 0);
        assert_eq!(tokinizer.token_infos[0].end, 1);
        assert_eq!(tokinizer.token_infos[0].token_type, Some(TokenType::Operator('-')));
        
        assert_eq!(tokinizer.token_infos[1].start, 2);
        assert_eq!(tokinizer.token_infos[1].end, 3);
        assert_eq!(tokinizer.token_infos[1].token_type, Some(TokenType::Operator('\'')));

        assert_eq!(tokinizer.token_infos[2].start, 4);
        assert_eq!(tokinizer.token_infos[2].end, 5);
        assert_eq!(tokinizer.token_infos[2].token_type,Some(TokenType::Operator('*')));

        assert_eq!(tokinizer.token_infos[3].start, 6);
        assert_eq!(tokinizer.token_infos[3].end, 7);
        assert_eq!(tokinizer.token_infos[3].token_type,Some(TokenType::Operator('`')));

        assert_eq!(tokinizer.token_infos[4].start, 8);
        assert_eq!(tokinizer.token_infos[4].end, 9);
        assert_eq!(tokinizer.token_infos[4].token_type,Some(TokenType::Operator('/')));

        assert_eq!(tokinizer.token_infos[5].start, 9);
        assert_eq!(tokinizer.token_infos[5].end, 10);
        assert_eq!(tokinizer.token_infos[5].token_type,Some(TokenType::Operator(',')));
    }
}