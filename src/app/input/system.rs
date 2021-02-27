// Input System

// [ ] InputMapping

use crate::app::{
    App,
    GameState,
};

use crate::app::sdl2::{
    event::Event,
    keyboard::Scancode,
};

use super::mapping::InputMapping;

const MAX_KEYBOARD_KEYS : usize = 512;
const MAX_MOUSE_BUTTONS : usize = 16;

pub struct InputSystem {
    pub(super) keyboard: KeyboardState,
    //pub(super) mouse: MouseState,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            keyboard: KeyboardState::default(),
            //mouse: MouseState::default(),
        }
    }

    pub fn handle_input(&mut self, event: &Event, timestamp: u64) {
        // @XXX window_id may be useful when we add multiple window support

        match event {
            Event::KeyDown { scancode: Some(key), repeat, .. } => {
                if !repeat {
                    self.keyboard.press(*key, timestamp);
                }
            },

            Event::KeyUp { scancode: Some(key), repeat, .. } => {
                if !repeat {
                    self.keyboard.release(*key, timestamp);
                }
            },

            /*
            Event::MouseButtonDown { mouse_btn, clicks } => {
            },
            */

            _ => {}
        }
    }

    pub fn update_input_mapping(&mut self, mapping: &mut InputMapping, timestamp: u64) {
        for button in mapping.button_mapping.values_mut() {
            button.update(self, timestamp);
        }
    }
}

impl<'a, S: GameState> App<'a, S> {
    pub fn update_input_mapping(&mut self, mapping: &mut InputMapping) {
        let timestamp = self.time.game_time;
        self.input.update_input_mapping(mapping, timestamp);
    }
}

#[derive(Copy, Clone)]
pub(super) struct KeyState {
    // @TODO use timestamp type (not created yet) instead of u64
    pub(super) timestamp: u64,

    // @TODO bitflags
    pub(super) down: bool,
}

#[derive(Copy, Clone)]
pub(super) struct KeyboardState {
    pub(super) keys: [KeyState; MAX_KEYBOARD_KEYS],
}

impl Default for KeyboardState {
    fn default() -> Self {
        let key = KeyState {
            timestamp: 0,
            down: false,
        };

        Self {
            keys: [key; MAX_KEYBOARD_KEYS],
        }
    }
}

impl KeyboardState {
    fn press(&mut self, key: Scancode, timestamp: u64) {
        let mut key = &mut self.keys[key as usize];

        key.timestamp = timestamp;
        key.down = true;
    }

    fn release(&mut self, key: Scancode, timestamp: u64) {
        let mut key = &mut self.keys[key as usize];

        key.timestamp = timestamp;
        key.down = false;
    }

    pub(super) fn get<'a>(&'a self, key: Scancode) -> &'a KeyState {
        &self.keys[key as usize]
    }
}

#[derive(Copy, Clone, Default)]
struct MouseButtonState {
    // @TODO use timestamp type (not created yet) instead of u64
    timestamp: u64,

    // @TODO bitflags
    down : bool,
    //double_clicked: bool,
}

impl MouseButtonState {
    fn press(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
        self.down = true;
    }

    fn release(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
        self.down = false;
    }
}

#[derive(Copy, Clone, Default)]
struct MouseState {
    x: i32,
    y: i32,
    buttons: [MouseButtonState; MAX_MOUSE_BUTTONS],
}
