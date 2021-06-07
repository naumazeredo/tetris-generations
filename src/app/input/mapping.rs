use std::collections::BTreeMap;

use crate::app::imdraw::ImDraw;
use super::button::Button;

// @XXX Somehow we should be able to verify if the last key pressed was on the keyboard/mouse
//      or gamepad

/*
// @TODO bind the controller to the input mapping, instead of to the button
enum ControllerBind {
    No,
    Any,
    Index(usize),
}
*/

// @TODO use hashed string
#[derive(ImDraw)]
pub struct InputMapping {
    pub(super) button_mapping: BTreeMap<String, Button>,
    //controller_bind: ControllerBind,
}

impl InputMapping {
    pub fn new() -> Self {
        Self {
            button_mapping: BTreeMap::new(),
            //controller_bind: ControllerBind::No,
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

    pub fn button(&self, name: String) -> &Button {
        self.button_mapping.get(&name)
            .expect(&format!("[input_system mapping] No mapping found for button: {}", name))
    }
}
