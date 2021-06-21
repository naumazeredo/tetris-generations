use std::cmp::max;
use crate::app::{
    App,
    imdraw::ImDraw,
    sdl2::{
        keyboard::Scancode,
        mouse::MouseButton,
    },
};

use super::{
    ControllerAxisThreshold,
    system::InputSystem,
};

/*
    let mut button = Button::new();
    button.add_key(sdl2::keyboard::Scancode::W);
    button.add_controller_button(0, sdl2::controller::Button::DPadUp);
    button.add_controller_axis(
        0,
        sdl2::controller::Axis::LeftY,
        ControllerAxisThreshold::lesser_than(-0.5)
    );

    input_mapping.add_button_mapping("UP".to_string(), button);

    let down = button.down();
    let pressed = button.pressed();
    let released = button.released();
    let long_press = button.pressed_for(1_000_000, app.time_system.game_time);

    if down { println!("down!"); }
    if pressed { println!("pressed!"); }
    if released { println!("released"); }
    if long_press { println!("long_press"); }
*/

#[derive(ImDraw)]
pub struct Button {
    keys: Vec<KeyInput>,
    mouse_buttons: Vec<MouseButtonInput>,
    controller_buttons: Vec<ControllerButtonInput>,
    controller_axes: Vec<ControllerAxisInput>,

    // @TODO bitflags
    down: bool,
    pressed: bool,
    released: bool,

    // If game is paused, only checking for the timestamp will make the button seem always pressed.
    // To avoid this, we use a flip-flop to store the last press/release state and only verify
    // pressed/release states once per click.
    last_state: bool,

    timestamp: u64,

}

impl Button {
    // @XXX maybe create a succint way to initialize a button with a list of keys, buttons, etc
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            mouse_buttons: Vec::new(),
            controller_buttons: Vec::new(),
            controller_axes: Vec::new(),

            down: false,
            pressed: false,
            released: false,
            last_state: false,
            timestamp: 0u64,
        }
    }

    // Keyboard keys
    // @Maybe abstract Scancode
    pub fn add_key(&mut self, code: Scancode) {
        if self.keys.iter().any(|elem| elem.0 == code) {
            panic!("[button add_key] trying to add repeated key to Button");
        }

        self.keys.push(KeyInput(code));
    }

    pub fn rem_key(&mut self, code: Scancode) {
        // @TODO debug check if we are trying to remove a key not present
        self.keys.retain(|elem| elem.0 != code);
    }

    // Mouse buttons
    pub fn add_mouse_button(&mut self, button: MouseButton) {
        if self.mouse_buttons.iter().any(|elem| elem.0 == button) {
            panic!("[button add_mouse_button] trying to add repeated mouse button to Button");
        }

        self.mouse_buttons.push(MouseButtonInput(button));
    }

    pub fn rem_mouse_button(&mut self, button: MouseButton) {
        self.mouse_buttons.retain(|elem| elem.0 != button);
    }

    // Controller buttons
    // @TODO add the controller id to the input mapping instead of the specific button
    pub fn add_controller_button(&mut self, controller_index: usize, button: sdl2::controller::Button) {
        if self.controller_buttons.iter().any(|elem| {
            elem.controller_index == controller_index && elem.button == button
        }) {
            panic!("[button add_controller_button] trying to add repeated controller button to Button");
        }

        self.controller_buttons.push(ControllerButtonInput{ controller_index, button });
    }

    pub fn rem_controller_button(&mut self, controller_index: usize, button: sdl2::controller::Button) {
        self.controller_buttons.retain(|elem| {
            elem.controller_index != controller_index || elem.button != button
        });
    }

    // Controller axis
    pub fn add_controller_axis(
        &mut self,
        controller_index: usize,
        axis: sdl2::controller::Axis,
        threshold: ControllerAxisThreshold,
    ) {
        if self.controller_axes.iter().any(|elem| {
            elem.controller_index == controller_index &&
            elem.axis == axis &&
            elem.threshold.direction == threshold.direction
        }) {
            panic!("[button add_controller_button] trying to add repeated controller button to Button");
        }

        self.controller_axes.push(
            ControllerAxisInput {
                controller_index,
                axis,
                threshold,

                last_update_value: false,
                last_change_timestamp: 0,
            }
        );
    }

    pub fn rem_controller_axis(&mut self, controller_index: usize, axis: sdl2::controller::Axis) {
        self.controller_axes.retain(|elem| {
            elem.controller_index != controller_index || elem.axis != axis
        });
    }

    pub(super) fn update(&mut self, input_system: &InputSystem, timestamp: u64) {
        let mut last_pressed  = 0u64;
        let mut last_released = 0u64;

        let mut total_down = 0;

        // Keyboard keys
        for key in self.keys.iter() {
            let key_state = input_system.keyboard.button_state(key.0);

            total_down += key_state.down as i32;
            last_pressed = max(last_pressed, (key_state.down as u64) * key_state.timestamp);
            last_released = max(last_released, (!key_state.down as u64) * key_state.timestamp);
        }

        // Mouse buttons
        for button in self.mouse_buttons.iter() {
            let button_state = input_system.mouse.button_state(button.0);

            total_down += button_state.down as i32;
            last_pressed = max(last_pressed, (button_state.down as u64) * button_state.timestamp);
            last_released = max(last_released, (!button_state.down as u64) * button_state.timestamp);
        }

        // Controller buttons
        for button in self.controller_buttons.iter() {
            //println!("{:?}", button);
            match input_system.controllers.controller_state(button.controller_index) {
                Some(controller_state) => {
                    let button_state = controller_state.button_state(button.button);

                    total_down += button_state.down as i32;
                    last_pressed = max(last_pressed, (button_state.down as u64) * button_state.timestamp);
                    last_released = max(last_released, (!button_state.down as u64) * button_state.timestamp);
                },
                None => {}
            }
        }

        // Controller axis
        // Requires extra logic since we want to map the button as pressed only when it crosses the
        // threshold. We can't put this logic in the axis itself since it can be used in multiple
        // buttons with different thresholds
        for axis in self.controller_axes.iter_mut() {
            match input_system.controllers.controller_state(axis.controller_index) {
                Some(controller_state) => {
                    let axis_state = controller_state.axis_state(axis.axis);

                    let axis_down = axis_state.pressed(axis.threshold);

                    let changed_state = axis_down ^ axis.last_update_value;
                    if changed_state {
                        axis.last_change_timestamp = timestamp;
                    }
                    axis.last_update_value = axis_down;

                    total_down += axis_down as i32;
                    last_pressed = max(last_pressed, (axis_down as u64) * axis.last_change_timestamp);
                    last_released = max(last_released, (!axis_down as u64) * axis.last_change_timestamp);
                },
                None => {}
            }
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

            self.pressed = (self.timestamp == timestamp) && !self.last_state;
            self.released = false;

            self.last_state = true;
        } else {
            //released

            self.timestamp = last_released;
            self.down = false;
            self.pressed = false;
            self.released = (self.timestamp == timestamp) && self.last_state;

            self.last_state = false;
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

    pub fn pressed_repeat<S>(&self, repeat_interval: u64, app: &App<'_, S>) -> bool {
        self.pressed_repeat_with_delay(repeat_interval, repeat_interval, app)
    }

    pub fn pressed_repeat_with_delay<S>(
        &self,
        repeat_delay: u64,
        repeat_interval: u64,
        app: &App<'_, S>
    ) -> bool {
        if !self.down { return false; }
        if self.pressed { return true; }

        let game_time = app.time_system.game_time;
        let frame_duration = app.time_system.game_frame_duration;

        // Check underflow cases
        let prev_press_count;
        if game_time < frame_duration + self.timestamp + repeat_delay {
            prev_press_count = 0;
        } else {
            let total_time = game_time - frame_duration - self.timestamp - repeat_delay;
            prev_press_count = 1 + total_time / repeat_interval;
        }

        let curr_press_count;
        if game_time < self.timestamp + repeat_delay {
            curr_press_count = 0;
        } else {
            curr_press_count = 1 + (game_time - self.timestamp - repeat_delay) / repeat_interval;
        }

        prev_press_count < curr_press_count
    }
}

#[derive(ImDraw)]
struct KeyInput(Scancode);

#[derive(ImDraw)]
struct MouseButtonInput(MouseButton);

#[derive(Debug, ImDraw)]
struct ControllerButtonInput {
    controller_index: usize,
    button: sdl2::controller::Button,
}

#[derive(ImDraw)]
struct ControllerAxisInput {
    controller_index: usize,
    axis: sdl2::controller::Axis,
    threshold: ControllerAxisThreshold,

    // Required to handle the last time it changed the threshold boundary
    last_update_value: bool,
    last_change_timestamp: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_keys() {
        let mut button = Button::new();

        assert_eq!(button.keys.len(), 0);

        button.add_key(Scancode::A);

        assert_eq!(button.keys.len(), 1);

        button.rem_key(Scancode::A);

        assert_eq!(button.keys.len(), 0);

        button.add_key(Scancode::A);
        button.add_key(Scancode::B);
        button.rem_key(Scancode::A);

        assert_eq!(button.keys.len(), 1);
    }

    #[test]
    fn test_button_mouse_buttons() {
        let mut button = Button::new();

        assert_eq!(button.mouse_buttons.len(), 0);

        button.add_mouse_button(MouseButton::Left);

        assert_eq!(button.mouse_buttons.len(), 1);

        button.rem_mouse_button(MouseButton::Left);

        assert_eq!(button.mouse_buttons.len(), 0);

        button.add_mouse_button(MouseButton::Left);
        button.add_mouse_button(MouseButton::Right);
        button.rem_mouse_button(MouseButton::Left);

        assert_eq!(button.mouse_buttons.len(), 1);
    }
}
