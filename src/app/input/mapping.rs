use std::collections::BTreeMap;

use crate::app::{
    App,
    imdraw::ImDraw,
};
use super::button::{
    Button,
    RemappableButton,
};

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
pub struct RegularInputMapping {
    pub(super) button_mapping: BTreeMap<String, RemappableButton>,
    //controller_bind: ControllerBind,
}

impl RegularInputMapping {
    pub fn new() -> Self {
        Self {
            button_mapping: BTreeMap::new(),
            //controller_bind: ControllerBind::No,
        }
    }

    pub fn add_button_mapping(&mut self, name: String, button: RemappableButton) {
        // @XXX nightly
        //.expect_none("[input mapping] overwriting button mapping");

        match self.button_mapping.insert(name, button) {
            None => {},
            Some(_) => panic!("[input mapping] overwriting button mapping"),
        }
    }
}

impl InputMapping for RegularInputMapping {
    type ButtonType = RemappableButton;

    fn button(&self, name: String) -> &Self::ButtonType {
        self.button_mapping.get(&name)
            .expect(&format!("[input_system mapping] No mapping found for button: {}", name))
    }

    fn update(&mut self, app: &App) {
        for button in self.button_mapping.values_mut() {
            button.update(&app.input_system, app.time_system.real_time);
        }
    }
}

pub trait InputMapping {
    type ButtonType: Button;
    fn button(&self, name: String) -> &Self::ButtonType;
    fn update(&mut self, app: &App);
}
