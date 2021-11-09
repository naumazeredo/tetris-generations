use crate::app::App;
use super::*;

pub struct ButtonState {
    pub pressed: bool,
    pub down: bool,
    pub hovering: bool,
}

pub struct Button<'a> {
    text: &'a str,
    disabled: bool,
}

impl<'a> Button<'a> {
    pub fn new(text: &str, app: &mut App) -> ButtonState {
        Button::builder(text).build(app)
    }

    pub fn builder<'b: 'a>(text: &'b str) -> Self {
        Self {
            text,
            disabled: false,
        }
    }

    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            disabled,
            ..self
        }
    }

    pub fn build(self, app: &mut App<'_>) -> ButtonState {
        let state = app.button_internal(self);

        ButtonState {
            pressed: state.pressed,
            down: state.down,
            hovering: state.hovering
        }
    }
}

// ------------------

fn new_button(text: &str, disabled: bool) -> State {
    State {
        pressed: false,
        down: false,
        hovering: false,
        disabled,
        variant: ElementVariant::Button { text: text.to_owned() },
    }
}

impl App<'_> {
    fn button_internal(&mut self, button: Button) -> &State {
        // @Maybe add text using the app.text method instead of calculating everything

        let id = Id::new(button.text).add("#__button");

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
            .and_modify(|state| {
                state.disabled = button.disabled;
            })
            .or_insert_with(|| new_button(button.text, button.disabled));

        if !button.disabled {
            self.update_state_interaction(id, layout)
        } else {
            self.ui_system.states.get(&id).unwrap()
        }
    }
}
