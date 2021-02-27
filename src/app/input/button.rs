use std::cmp::max;

use crate::app::imdraw::ImDraw;
use crate::app::sdl2::{
    //controller::{Button, Axis},
    //keyboard::{Keycode, Scancode},
    keyboard::Scancode,
};

use super::InputSystem;

#[derive(ImDraw)]
pub struct Button {
    keys: Vec<KeyInput>,
    //controller_buttons: Vec<ControllerButtonInput>,
    //axis: Vec<AxisInput>,
    //mouse_button: Vec<MouseButtonInput>,

    // @TODO bitflags
    down: bool,
    pressed: bool,
    released: bool,

    timestamp: u64,
}

impl Button {
    // @XXX maybe create a succint way to initialize a button
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            down: false,
            pressed: false,
            released: false,
            timestamp: 0u64,
        }
    }

    pub fn add_key(&mut self, code: Scancode) {
        self.keys.push(KeyInput { code });
    }

    pub(super) fn update(&mut self, input_system: &InputSystem, timestamp: u64) {
        let mut last_pressed  = 0u64;
        let mut last_released = 0u64;

        let mut total_down = 0;

        for key in self.keys.iter() {
            let key_state = input_system.keyboard.get(key.code);

            total_down += key_state.down as i32;
            last_pressed = max(last_pressed, (key_state.down as u64) * key_state.timestamp);
            last_released = max(last_released, (!key_state.down as u64) * key_state.timestamp);
        }

        // Multiple keys per button logic
        // down = any key is down
        // up   = no key is down
        // In case two keys, A and B, are mapped to the same button and we have the following
        // sequence of events:
        // A down, B down, A up, B up
        // we should see the following states
        // pressed+down, no change (down), no change (down), released

        if total_down > 0 {
            // pressed

            // If the button was down already, we don't update the timestamp to avoid false
            // pressed states. It's only a problem when using multiple keys for the same button
            if !self.down {
                self.timestamp = last_pressed;
                self.down = true;
            }

            self.pressed = self.timestamp == timestamp;
        } else {
            //released

            self.timestamp = last_released;
            self.down = false;
            self.released = self.timestamp == timestamp;
        }
    }

    pub fn down(&self)     -> bool { self.down }
    pub fn pressed(&self)  -> bool { self.pressed }
    pub fn released(&self) -> bool { self.released } // @TODO return release duration?

    pub fn pressed_for(&self, duration: u64, timestamp: u64)  -> bool {
        self.down && timestamp - self.timestamp >= duration
    }

    pub fn released_for(&self, duration: u64, timestamp: u64)  -> bool {
        !self.down && timestamp - self.timestamp >= duration
    }
}

#[derive(ImDraw)]
struct KeyInput {
    code: Scancode,
}

/*
struct ControllerButtonInput {
    controller_index: u32,
    key: Keycode,
}

struct AxisInput {
    controller_index: u32,
    axis: Axis,
    threshold: f32,
    positive: bool,
}

struct MouseButtonInput {
    button: MouseButton,
}
*/
