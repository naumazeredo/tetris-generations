/* Usage

// construction

let mut input_mapping = InputMapping::new();

{
    let mut button = Button::new();
    button.add_key(sdl2::keyboard::Scancode::W);
    button.add_controller_button(0, sdl2::controller::Button::DPadUp);
    button.add_controller_axis(
        0,
        sdl2::controller::Axis::LeftY,
        ControllerAxisThreshold::lesser_than(-0.5)
    );

    input_mapping.add_button_mapping("UP".to_string(), button);
}

{
    let mut button = Button::new();
    button.add_key(sdl2::keyboard::Scancode::S);
    button.add_controller_button(0, sdl2::controller::Button::DPadDown);
    button.add_controller_axis(
        0,
        sdl2::controller::Axis::LeftY,
        ControllerAxisThreshold::greater_than(0.5)
    );

    input_mapping.add_button_mapping("DOWN".to_string(), button);
}

{
    let mut button = Button::new();
    button.add_key(sdl2::keyboard::Scancode::D);
    button.add_controller_button(0, sdl2::controller::Button::DPadRight);
    button.add_controller_axis(
        0,
        sdl2::controller::Axis::LeftX,
        ControllerAxisThreshold::greater_than(0.5)
    );

    input_mapping.add_button_mapping("RIGHT".to_string(), button);
}

{
    let mut button = Button::new();
    button.add_key(sdl2::keyboard::Scancode::A);
    button.add_controller_button(0, sdl2::controller::Button::DPadLeft);
    button.add_controller_axis(
        0,
        sdl2::controller::Axis::LeftX,
        ControllerAxisThreshold::lesser_than(-0.5)
    );

    input_mapping.add_button_mapping("LEFT".to_string(), button);
}

// update

app.update_input_mapping(&mut self.input_mapping);

let u_button = self.input_mapping.button("UP".to_string()).down();
let d_button = self.input_mapping.button("DOWN".to_string()).down();
let r_button = self.input_mapping.button("RIGHT".to_string()).down();
let l_button = self.input_mapping.button("LEFT".to_string()).down();
let move_direction = Vec2 {
    x: ((r_button as i32) - (l_button as i32)) as f32,
    y: ((d_button as i32) - (u_button as i32)) as f32,
};

*/

pub mod button;
pub mod mapping;
pub mod system;

pub use button::*;
pub use mapping::*;
pub(in crate::app) use system::*; // @XXX how to avoid this???

use crate::app::imdraw::ImDraw;


#[derive(Copy, Clone, Debug, ImDraw)]
pub struct ControllerAxisThreshold {
    value: f32,
    direction: ControllerAxisDirection
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ControllerAxisDirection {
    GreaterThan,
    LesserThan,
}

impl ControllerAxisThreshold {
    pub fn greater_than(value: f32) -> ControllerAxisThreshold {
        Self {
            value,
            direction: ControllerAxisDirection::GreaterThan,
        }
    }

    pub fn lesser_than(value: f32) -> ControllerAxisThreshold {
        Self {
            value,
            direction: ControllerAxisDirection::LesserThan,
        }
    }
}

impl_imdraw_todo!(ControllerAxisDirection);
