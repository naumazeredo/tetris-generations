// Input System

// [ ] InputMapping

use std::collections::BTreeMap;

use crate::app::{
    App,
    GameState,
};

use crate::app::sdl2::{
    event::Event,
    keyboard::Scancode,
};

use super::mapping::InputMapping;

const MAX_KEYBOARD_KEYS      : usize = 512;
const MAX_MOUSE_BUTTONS      : usize = 16;
const MAX_CONTROLLER_BUTTONS : usize = 16;
const MAX_CONTROLLER_AXIS    : usize = 16;
const MAX_CONTROLLERS        : usize = 16;

pub struct InputSystem {
    controller_subsystem: sdl2::GameControllerSubsystem,
    pub(super) keyboard: KeyboardState,
    pub(super) mouse: MouseState,
    pub(super) controllers: ControllerStateContainer,
}

impl InputSystem {
    pub(in crate::app) fn new(controller_subsystem: sdl2::GameControllerSubsystem) -> Self {
        Self {
            controller_subsystem,
            keyboard: Default::default(),
            mouse: Default::default(),
            controllers: Default::default(),
        }
    }

    pub(in crate::app) fn handle_input(&mut self, event: &Event, timestamp: u64) {
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

            // Mouse
            // @XXX which
            // @XXX clicks
            Event::MouseButtonDown { mouse_btn: button, .. } => {
                self.mouse.press(*button, timestamp);
            },

            Event::MouseButtonUp { mouse_btn: button, .. } => {
                self.mouse.release(*button, timestamp);
            },

            Event::MouseMotion { x, y, .. } => {
                self.mouse.set_pos(*x, *y);
            },

            // Joystick
            /*
            Event::JoyDeviceAdded { which, .. } => {
                println!("joy connected: {}", which);
            }

            Event::JoyDeviceRemoved { which, .. } => {
                println!("joy disconnected: {}", which);
            }
            */

            // Controller
            // Connection
            Event::ControllerDeviceAdded { which, .. } => {
                let index = *which;

                match self.controller_subsystem.open(index) {
                    Ok(c) => self.controllers.add(index, c),
                    Err(_) => println!("connect failed ({})", which)
                };
            },

            Event::ControllerDeviceRemoved { which, .. } => {
                let id = *which;
                self.controllers.remove(id);
            },

            // Buttons
            Event::ControllerButtonDown { which, button, .. } => {
                let id = *which;
                self.controllers.press_button(id, *button, timestamp);
            },

            Event::ControllerButtonUp { which, button, .. } => {
                let id = *which;
                self.controllers.release_button(id, *button, timestamp);
            },

            // Axis
            Event::ControllerAxisMotion { which, axis, value, .. } => {
                let id = *which;
                let axis = *axis;
                let value = if *value > 0 {
                    (*value) as f32 / (i16::MAX as f32)
                } else {
                    (*value) as f32 / (i16::MIN as f32)
                };

                self.controllers.update_axis(id, axis, value, timestamp);
            },

            // Touchpad (not supported in rust-sdl2)

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

// -------
// General
// -------

#[derive(Copy, Clone, Default)]
pub(super) struct ButtonState {
    // @TODO use timestamp type (not created yet) instead of u64
    pub(super) timestamp: u64,

    // @TODO bitflags
    pub(super) down: bool,
}

impl ButtonState {
    fn press(&mut self, timestamp: u64) {
        assert!(!self.down);
        self.timestamp = timestamp;
        self.down = true;
    }

    fn release(&mut self, timestamp: u64) {
        assert!(self.down);
        self.timestamp = timestamp;
        self.down = false;
    }
}

#[derive(Copy, Clone, Default)]
pub(super) struct AxisState {
    // @TODO use timestamp type (not created yet) instead of u64
    pub(super) timestamp: u64,
    pub(super) value: f32,
}

impl AxisState {
    fn update_value(&mut self, value: f32, timestamp: u64) {
        self.timestamp = timestamp;
        self.value = value;
    }

    pub fn pressed(&self, threshold: f32, greater_than: bool) -> bool {
        // if greater_than { return self.value > threshold; } else { return self.value < threshold; }
        (greater_than && (self.value > threshold)) as bool
    }
}

// --------
// Keyboard
// --------

#[derive(Copy, Clone)]
pub(super) struct KeyboardState {
    pub(super) keys: [ButtonState; MAX_KEYBOARD_KEYS],
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self {
            keys: [ButtonState::default(); MAX_KEYBOARD_KEYS],
        }
    }
}

impl KeyboardState {
    fn press(&mut self, key: Scancode, timestamp: u64) {
        self.keys[key as usize].press(timestamp);
    }

    fn release(&mut self, key: Scancode, timestamp: u64) {
        self.keys[key as usize].release(timestamp);
    }

    pub(super) fn button_state(&self, key: Scancode) -> &ButtonState {
        &self.keys[key as usize]
    }
}

// -----
// Mouse
// -----

// @Maybe MouseButtonState (for double_clicked if needed)

#[derive(Default)]
pub(super) struct MouseState {
    // @TODO relative position + moved (do we need it?)

    // window_id
    pos: (i32, i32),
    buttons: [ButtonState; MAX_MOUSE_BUTTONS],

    //rel_pos: (i32, i32),
    //moved: bool,
}

impl MouseState {
    fn press(&mut self, button: sdl2::mouse::MouseButton, timestamp: u64) {
        self.buttons[button as usize].press(timestamp);
    }

    fn release(&mut self, button: sdl2::mouse::MouseButton, timestamp: u64) {
        self.buttons[button as usize].release(timestamp);
    }

    fn set_pos(&mut self, x: i32, y: i32) {
        // @TODO relative position + moved (do we need it?)
        //self.rel_pos = (x - self.pos.0, y - self.pos.1);
        //self.moved = true;

        self.pos = (x, y);
    }

    pub(super) fn button_state(&self, button: sdl2::mouse::MouseButton) -> &ButtonState {
        &self.buttons[button as usize]
    }

    pub(super) fn get_pos(&self) -> (i32, i32) {
        self.pos
    }
}


// ----------
// Controller
// ----------

#[derive(Default)]
pub(super) struct ControllerStateContainer {
    pub(super) controllers: [ControllerState; MAX_CONTROLLERS],
    id_to_index: BTreeMap<u32, usize>,
}

impl ControllerStateContainer {
    fn add(&mut self, index: u32, controller: sdl2::controller::GameController) {
        let id = controller.instance_id();
        println!("connected ({}, {}): {}", index, id, controller.name());
        self.controllers[index as usize].connect(controller);

        self.id_to_index.insert(id, index as usize);
    }

    fn remove(&mut self, id: u32) {
        println!("disconnected: {}", id);
        let index = *self.id_to_index.get(&id)
            .expect("trying to remove an unmapped controller id");
        self.controllers[index].disconnect();
        self.id_to_index.remove(&id);
    }

    fn press_button(&mut self, id: u32, button: sdl2::controller::Button, timestamp: u64) {
        let index = *self.id_to_index.get(&id)
            .expect("trying to press on an unmapped controller id");
        self.controllers[index].press_button(button, timestamp);
    }

    fn release_button(&mut self, id: u32, button: sdl2::controller::Button, timestamp: u64) {
        let index = *self.id_to_index.get(&id)
            .expect("trying to release on an unmapped controller id");
        self.controllers[index].release_button(button, timestamp);
    }

    fn update_axis(&mut self, id: u32, axis: sdl2::controller::Axis, value: f32, timestamp: u64) {
        let index = *self.id_to_index.get(&id)
            .expect("trying to release on an unmapped controller id");
        self.controllers[index].update_axis(axis, value, timestamp);
    }

    pub(super) fn controller_state(&self, index: usize) -> Option<&ControllerState> {
        match self.controllers[index].controller {
            Some(_) => Some(&self.controllers[index]),
            None => None,
        }
    }
}

#[derive(Default)]
pub(super) struct ControllerState {
    // @TODO controller name, vendor, type (not supported by rust-sdl2 yet)
    controller: Option<sdl2::controller::GameController>,
    buttons: [ButtonState; MAX_CONTROLLER_BUTTONS],
    axes: [AxisState; MAX_CONTROLLER_AXIS],
}

impl ControllerState {
    fn connect(&mut self, controller: sdl2::controller::GameController) {
        assert!(!self.is_connected());
        self.controller = Some(controller);
    }

    fn disconnect(&mut self) {
        assert!(self.is_connected());
        self.controller.take();
    }

    fn is_connected(&self) -> bool {
        self.controller.is_some()
    }

    fn press_button(&mut self, button: sdl2::controller::Button, timestamp: u64) {
        assert!(self.is_connected());
        self.buttons[button as usize].press(timestamp);
    }

    fn release_button(&mut self, button: sdl2::controller::Button, timestamp: u64) {
        assert!(self.is_connected());
        self.buttons[button as usize].release(timestamp);
    }

    fn update_axis(&mut self, axis: sdl2::controller::Axis, value: f32, timestamp: u64) {
        assert!(self.is_connected());
        self.axes[axis as usize].update_value(value, timestamp);
    }

    pub(super) fn button_state(&self, button: sdl2::controller::Button) -> &ButtonState {
        &self.buttons[button as usize]
    }

    pub(super) fn axis_state(&self, axis: sdl2::controller::Axis) -> &AxisState {
        &self.axes[axis as usize]
    }
}
