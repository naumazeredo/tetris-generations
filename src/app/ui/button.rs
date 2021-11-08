use crate::app::App;
use super::*;

pub struct ButtonState {
    pub pressed: bool,
    pub down: bool,
    pub hovering: bool,
}

pub struct Button<'a> {
    text: &'a str,
}

impl<'a> Button<'a> {
    pub fn new(text: &str, app: &mut App) -> ButtonState {
        Button::builder(text).build(app)
    }

    pub fn builder<'b: 'a>(text: &'b str) -> Self {
        Self {
            text,
        }
    }

    pub fn build(self, app: &mut App<'_>) -> ButtonState {
        let state = app.button_internal(self.text);

        ButtonState {
            pressed: state.pressed,
            down: state.down,
            hovering: state.hovering
        }
    }
}

// ------------------

fn new_button(text: &str) -> State {
    State {
        pressed: false,
        down: false,
        hovering: false,
        variant: ElementVariant::Button { text: text.to_owned() },
    }
}

impl App<'_> {
    fn button_internal(&mut self, text: &str) -> &mut State {
        // @Maybe add text using the app.text method instead of calculating everything

        let id = Id::new(text).add("#__button");

        // @Cleanup these ui.last_mut().unwrap() calls
        // Calculate element size
        let ui = &self.ui_system.uis.last().unwrap();
        let size = Vec2i {
            x: ui.draw_width(),
            y: ui.style.line_height
        };
        let layout = self.new_layout(size);

        self.add_element(id, layout);

        self.ui_system.states.entry(id)
            .or_insert_with(|| new_button(text));

        self.update_state_interaction(id, layout)
    }
}
