use alloc::vec::Vec;
use regex::{Match};
use log;

use crate::types::{UiToken, UiTokenType};

pub struct UiTokenCollection {
    tokens: Vec<UiToken>,
    char_sizes: Vec<usize>
}

pub struct UiTokenIterator<'a> {
    iter: alloc::slice::Iter<'a, UiToken>
}

impl<'a> Iterator for UiTokenIterator<'a> {
    type Item = &'a UiToken;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl UiTokenCollection {
    pub fn new() -> UiTokenCollection {
        UiTokenCollection {
            tokens: Vec::new(),
            char_sizes: Vec::with_capacity(64)
        }
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn generate_char_map<'a>(&mut self, data: &'a str) {
        for (index, ch) in data.chars().enumerate() {
            for _ in 0..ch.len_utf8() {
                self.char_sizes.push(index);
            }
        }
    }

    pub fn add(&mut self, start: usize, end: usize, ui_type: UiTokenType) {
        if self.check_collision(start, end) {
            self.tokens.push(UiToken { start, end, ui_type })
        }
    }

    pub fn add_from_regex_match<'t>(&mut self, capture: Option<Match<'t>>, token_type: UiTokenType) {
        match capture {
            Some(content) => {
                if content.start() < content.end() {
                    if self.check_collision(content.start(), content.end()) {
                        self.tokens.push(UiToken {
                            start: self.get_position(content.start()),
                            end: self.get_position(content.end()-1),
                            ui_type: token_type
                        });
                    }
                }
            },
            _ => ()
        };
    }

    pub fn iter(&self) -> UiTokenIterator {
        UiTokenIterator { iter: self.tokens.iter() }
    }

    fn get_position(&self, index: usize) -> usize {
        match self.char_sizes.get(index) {
            Some(position) => *position,
            None => {
                log::error!("{} not found in char map list, returned 0", index);
                0
            }
        }
    }

    fn check_collision(&self, start_position: usize, end_position: usize) -> bool {
        for item in self.iter() {
            if item.start <= start_position && item.end > start_position {
                return false
            }
            else if item.start < end_position && item.end >= end_position {
                return false
            }
        }

        true
    }

    pub fn update_tokens(&mut self, position_start: usize, position_end: usize, new_type: UiTokenType) {
        let ui_start_position   = self.get_position(position_start);
        let ui_end_position     = self.get_position(position_end-1);

        let mut ui_start_index: i8  = -1;

        for (index, ui_token) in self.iter().enumerate() {
            if ui_token.start == ui_start_position {
                ui_start_index = index as i8;
                break;
            }
        }

        if ui_start_index > -1 {
            for (index, ui_token) in self.tokens.iter().enumerate() {
                if ui_token.end == ui_end_position {
                    self.tokens.drain(ui_start_index as usize..index + 1);
                    self.tokens.insert(ui_start_index as usize, UiToken {
                        start: ui_start_position as usize,
                        end: ui_end_position as usize,
                        ui_type: new_type
                    });

                    break;
                }
            }
        }
    }
}

#[cfg(test)]
#[test]
fn collection_test_1() {
    let mut collection = UiTokenCollection::new();
    assert_eq!(collection.len(), 0);

    collection.add(0, 10, UiTokenType::Money);
    assert_eq!(collection.len(), 1);

    collection.add(10, 11, UiTokenType::Money);
    assert_eq!(collection.len(), 2);

    collection.add(10, 11, UiTokenType::Money);
    assert_eq!(collection.len(), 2);
}

#[cfg(test)]
#[test]
fn collection_test_2() {
    use regex;
    let mut collection = UiTokenCollection::new();
    assert_eq!(collection.len(), 0);

    let re = regex::Regex::new("test").unwrap();
    for capture in re.captures_iter(&"test data") {
        collection.add_from_regex_match(capture.get(0), UiTokenType::Text);
    }
    assert_eq!(collection.len(), 1);

    for capture in re.captures_iter(&"test data") {
        collection.add_from_regex_match(capture.get(0), UiTokenType::Money);
    }
    assert_eq!(collection.len(), 1);
}


#[cfg(test)]
#[test]
fn collection_test_3() {
    use regex;
    let mut collection = UiTokenCollection::new();
    assert_eq!(collection.len(), 0);

    let re = regex::Regex::new("test").unwrap();
    for capture in re.captures_iter(&"test test test") {
        collection.add_from_regex_match(capture.get(0), UiTokenType::Text);
    }
    assert_eq!(collection.len(), 3);

    let mut tokens = Vec::new();
    tokens.push(UiToken {
        start: 0,
        end: 4,
        ui_type: UiTokenType::Text
    });
    tokens.push(UiToken {
        start: 5,
        end: 9,
        ui_type: UiTokenType::Text
    });
    tokens.push(UiToken {
        start: 10,
        end: 14,
        ui_type: UiTokenType::Text
    });

    for (index, token) in collection.iter().enumerate() {
        assert_eq!(token, &tokens[index]);
    }
}
