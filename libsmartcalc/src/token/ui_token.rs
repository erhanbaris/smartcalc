use alloc::vec::Vec;
use alloc::rc::Rc;

use crate::types::{UiToken, UiTokenType};

pub struct UiTokenCollection {
    tokens: Vec<UiToken>,
    current: usize
}

impl UiTokenCollection {
    pub fn new() -> UiTokenCollection {
        UiTokenCollection {
            tokens: Vec::new(),
            current: 0
        }
    }

    pub fn add(&mut self, start: usize, end: usize, ui_type: UiTokenType) {
        self.tokens.push(UiToken { start, end, ui_type })
    }
}
