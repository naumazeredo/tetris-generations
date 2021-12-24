use crate::app::input::{
    ControllerAxisThreshold,
    mapping::RegularMapping,
    button::RemappableButton,
};

// @TODO Tetris keys + UI keys
pub const KEY_DOWN       : &str = "down";
pub const KEY_LEFT       : &str = "left";
pub const KEY_RIGHT      : &str = "right";
pub const KEY_ROTATE_CW  : &str = "rotate_cw";
pub const KEY_ROTATE_CCW : &str = "rotate_ccw";
pub const KEY_HOLD       : &str = "hold";
pub const KEY_HARD_DROP  : &str = "hard_drop";

pub const KEY_UP         : &str = "up"; // not used on Tetris
pub const KEY_OPTIONS    : &str = "options";

pub fn get_default_input_mapping() -> RegularMapping {
    let mut input_mapping = RegularMapping::new();

    {
        let mut button = RemappableButton::new();
        button.add_key(sdl2::keyboard::Scancode::K);
        button.add_key(sdl2::keyboard::Scancode::Down);
        button.add_controller_button(0, sdl2::controller::Button::DPadDown);
        button.add_controller_axis(
            0,
            sdl2::controller::Axis::LeftY,
            ControllerAxisThreshold::greater_than(0.5)
        );

        input_mapping.add_button_mapping(KEY_DOWN.to_string(), button);
    }

    {
        let mut button = RemappableButton::new();
        button.add_key(sdl2::keyboard::Scancode::L);
        button.add_key(sdl2::keyboard::Scancode::Right);
        button.add_controller_button(0, sdl2::controller::Button::DPadRight);
        button.add_controller_axis(
            0,
            sdl2::controller::Axis::LeftX,
            ControllerAxisThreshold::greater_than(0.5)
        );

        input_mapping.add_button_mapping(KEY_RIGHT.to_string(), button);
    }

    {
        let mut button = RemappableButton::new();
        button.add_key(sdl2::keyboard::Scancode::J);
        button.add_key(sdl2::keyboard::Scancode::Left);
        button.add_controller_button(0, sdl2::controller::Button::DPadLeft);
        button.add_controller_axis(
            0,
            sdl2::controller::Axis::LeftX,
            ControllerAxisThreshold::lesser_than(-0.5)
        );

        input_mapping.add_button_mapping(KEY_LEFT.to_string(), button);
    }

    {
        let mut button = RemappableButton::new();
        button.add_key(sdl2::keyboard::Scancode::LCtrl);
        button.add_key(sdl2::keyboard::Scancode::RCtrl);
        button.add_key(sdl2::keyboard::Scancode::Z);
        button.add_controller_button(0, sdl2::controller::Button::A);

        input_mapping.add_button_mapping(KEY_ROTATE_CCW.to_string(), button);
    }

    {
        let mut button = RemappableButton::new();
        button.add_key(sdl2::keyboard::Scancode::Up);
        button.add_key(sdl2::keyboard::Scancode::I);
        button.add_key(sdl2::keyboard::Scancode::X);
        button.add_controller_button(0, sdl2::controller::Button::B);

        input_mapping.add_button_mapping(KEY_ROTATE_CW.to_string(), button);
    }

    {
        let mut button = RemappableButton::new();
        button.add_key(sdl2::keyboard::Scancode::C);
        button.add_key(sdl2::keyboard::Scancode::LShift);
        button.add_key(sdl2::keyboard::Scancode::RShift);
        button.add_controller_button(0, sdl2::controller::Button::X);
        button.add_controller_button(0, sdl2::controller::Button::LeftShoulder);
        button.add_controller_button(0, sdl2::controller::Button::RightShoulder);

        input_mapping.add_button_mapping(KEY_HOLD.to_string(), button);
    }

    {
        let mut button = RemappableButton::new();
        button.add_key(sdl2::keyboard::Scancode::Space);
        button.add_controller_button(0, sdl2::controller::Button::DPadUp);
        button.add_controller_axis(
            0,
            sdl2::controller::Axis::LeftY,
            ControllerAxisThreshold::lesser_than(-0.5)
        );

        input_mapping.add_button_mapping(KEY_HARD_DROP.to_string(), button);
    }

    {
        let mut button = RemappableButton::new();
        button.add_key(sdl2::keyboard::Scancode::Escape);
        button.add_controller_button(0, sdl2::controller::Button::Start);

        input_mapping.add_button_mapping(KEY_OPTIONS.to_string(), button);
    }

    input_mapping
}
