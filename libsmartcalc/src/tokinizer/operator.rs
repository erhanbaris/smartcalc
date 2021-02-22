use alloc::string::ToString;
use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use regex::Regex;
use crate::{types::*};
use crate::tokinizer::Tokinizer;
use crate::token::ui_token::UiTokenType;

pub fn operator_regex_parser(tokinizer: &mut Tokinizer, group_item: &Vec<Regex>) {
    for re in group_item.iter() {
        for capture in re.captures_iter(&tokinizer.data.to_owned()) {
            if tokinizer.add_token_location(capture.get(0).unwrap().start(), capture.get(0).unwrap().end(), Some(TokenType::Operator(capture.get(0).unwrap().as_str().chars().nth(0).unwrap())), capture.get(0).unwrap().as_str().to_string())  {
                tokinizer.add_ui_token(capture.get(0), UiTokenType::Operator);
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
        let data = " - merhaba".to_string();
        let mut tokinizer = Tokinizer {
            column: 0,
            line: 0,
            tokens: Vec::new(),
            iter: data.chars().collect(),
            data: data.to_string(),
            index: 0,
            indexer: 0,
            total: data.chars().count(),
            token_locations: Vec::new(),
            ui_tokens: Vec::new(),
            char_sizes: Vec::with_capacity(data.len())
        };
        initialize();
        tokinizer.calculate_utf8_sizes();
        tokinizer.tokinize_with_regex();

        assert_eq!(tokinizer.token_locations.len(), 2);

        assert_eq!(tokinizer.token_locations[0].start, 1);
        assert_eq!(tokinizer.token_locations[0].end, 2);
        assert_eq!(tokinizer.token_locations[0].token_type, Some(TokenType::Operator('-')));
        
        assert_eq!(tokinizer.token_locations[1].start, 3);
        assert_eq!(tokinizer.token_locations[1].end, 10);
        assert_eq!(tokinizer.token_locations[1].token_type, Some(TokenType::Text("merhaba".to_string())));
    }

    #[cfg(test)]
    #[test]
    fn operator_test_2() {
        use alloc::string::ToString;
        use alloc::vec::Vec;
        let data = "- ' * ` /,".to_string();
        let mut tokinizer = Tokinizer {
            column: 0,
            line: 0,
            tokens: Vec::new(),
            iter: data.chars().collect(),
            data: data.to_string(),
            index: 0,
            indexer: 0,
            total: data.chars().count(),
            token_locations: Vec::new(),
            ui_tokens: Vec::new(),
            char_sizes: Vec::with_capacity(data.len())
        };
        initialize();

        tokinizer.calculate_utf8_sizes();
        tokinizer.tokinize_with_regex();

        assert_eq!(tokinizer.token_locations.len(), 6);
        assert_eq!(tokinizer.token_locations[0].start, 0);
        assert_eq!(tokinizer.token_locations[0].end, 1);
        assert_eq!(tokinizer.token_locations[0].token_type, Some(TokenType::Operator('-')));
        
        assert_eq!(tokinizer.token_locations[1].start, 2);
        assert_eq!(tokinizer.token_locations[1].end, 3);
        assert_eq!(tokinizer.token_locations[1].token_type, Some(TokenType::Operator('\'')));

        assert_eq!(tokinizer.token_locations[2].start, 4);
        assert_eq!(tokinizer.token_locations[2].end, 5);
        assert_eq!(tokinizer.token_locations[2].token_type,Some(TokenType::Operator('*')));

        assert_eq!(tokinizer.token_locations[3].start, 6);
        assert_eq!(tokinizer.token_locations[3].end, 7);
        assert_eq!(tokinizer.token_locations[3].token_type,Some(TokenType::Operator('`')));

        assert_eq!(tokinizer.token_locations[4].start, 8);
        assert_eq!(tokinizer.token_locations[4].end, 9);
        assert_eq!(tokinizer.token_locations[4].token_type,Some(TokenType::Operator('/')));

        assert_eq!(tokinizer.token_locations[5].start, 9);
        assert_eq!(tokinizer.token_locations[5].end, 10);
        assert_eq!(tokinizer.token_locations[5].token_type,Some(TokenType::Operator(',')));
    }
}