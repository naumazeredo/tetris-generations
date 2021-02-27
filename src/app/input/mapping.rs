use std::collections::BTreeMap;

use crate::app::imdraw::ImDraw;
use super::button::Button;

// @XXX Somehow we should be able to verify if the last key pressed was on the keyboard/mouse
//      or gamepad

// @TODO use hashed string
#[derive(ImDraw)]
pub struct InputMapping {
    pub(super) button_mapping: BTreeMap<String, Button>,
}

impl InputMapping {
    pub fn new() -> Self {
        Self {
            button_mapping: BTreeMap::new(),
        }
    }

    pub fn add_button_mapping(&mut self, name: String, button: Button) {
        // @XXX nightly
        //.expect_none("[input mapping] overwriting button mapping");

        match self.button_mapping.insert(name, button) {
            None => {},
            Some(_) => panic!("[input mapping] overwriting button mapping"),
        }
    }

    pub fn button<'a>(&'a self, name: String) -> &'a Button {
        self.button_mapping.get(&name).unwrap()
    }
}
