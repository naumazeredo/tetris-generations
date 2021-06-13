use crate::app::{
    InputMapping,
    Button,
    ControllerAxisThreshold,
};

pub fn get_default_input_mapping() -> InputMapping {
    let mut input_mapping = InputMapping::new();

    {
        let mut button = Button::new();
        button.add_key(sdl2::keyboard::Scancode::K);
        button.add_key(sdl2::keyboard::Scancode::Down);
        button.add_controller_button(0, sdl2::controller::Button::DPadDown);
        button.add_controller_axis(
            0,
            sdl2::controller::Axis::LeftY,
            ControllerAxisThreshold::greater_than(0.5)
        );

        input_mapping.add_button_mapping("down".to_string(), button);
    }

    {
        let mut button = Button::new();
        button.add_key(sdl2::keyboard::Scancode::L);
        button.add_key(sdl2::keyboard::Scancode::Right);
        button.add_controller_button(0, sdl2::controller::Button::DPadRight);
        button.add_controller_axis(
            0,
            sdl2::controller::Axis::LeftX,
            ControllerAxisThreshold::greater_than(0.5)
        );

        input_mapping.add_button_mapping("right".to_string(), button);
    }

    {
        let mut button = Button::new();
        button.add_key(sdl2::keyboard::Scancode::J);
        button.add_key(sdl2::keyboard::Scancode::Left);
        button.add_controller_button(0, sdl2::controller::Button::DPadLeft);
        button.add_controller_axis(
            0,
            sdl2::controller::Axis::LeftX,
            ControllerAxisThreshold::lesser_than(-0.5)
        );

        input_mapping.add_button_mapping("left".to_string(), button);
    }

    {
        let mut button = Button::new();
        button.add_key(sdl2::keyboard::Scancode::LCtrl);
        button.add_key(sdl2::keyboard::Scancode::RCtrl);
        button.add_key(sdl2::keyboard::Scancode::Z);
        button.add_controller_button(0, sdl2::controller::Button::A);

        input_mapping.add_button_mapping("rotate_ccw".to_string(), button);
    }

    {
        let mut button = Button::new();
        button.add_key(sdl2::keyboard::Scancode::Up);
        button.add_key(sdl2::keyboard::Scancode::I);
        button.add_key(sdl2::keyboard::Scancode::X);
        button.add_controller_button(0, sdl2::controller::Button::B);

        input_mapping.add_button_mapping("rotate_cw".to_string(), button);
    }

    {
        let mut button = Button::new();
        button.add_key(sdl2::keyboard::Scancode::C);
        button.add_key(sdl2::keyboard::Scancode::LShift);
        button.add_key(sdl2::keyboard::Scancode::RShift);
        button.add_controller_button(0, sdl2::controller::Button::X);
        // Shoulder buttons

        input_mapping.add_button_mapping("hold".to_string(), button);
    }

    {
        let mut button = Button::new();
        button.add_key(sdl2::keyboard::Scancode::Space);
        button.add_controller_button(0, sdl2::controller::Button::DPadUp);
        button.add_controller_axis(
            0,
            sdl2::controller::Axis::LeftY,
            ControllerAxisThreshold::lesser_than(-0.5)
        );

        input_mapping.add_button_mapping("hard_drop".to_string(), button);
    }

    input_mapping
}
