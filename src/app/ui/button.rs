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

    #[inline(always)] pub fn build(self, app: &mut App) -> ButtonState {
        self.build_with_placer(&mut app.ui_system.top_ui().index(), app)
    }

    #[inline(always)] pub fn build_with_placer<P: Placer>(
        self,
        placer: &mut P,
        app: &mut App
    ) -> ButtonState {
        if let Some(state) = button_internal(self, placer, app) {
            ButtonState {
                pressed:  state.pressed,
                down:     state.down,
                hovering: state.hovering
            }
        } else {
            ButtonState {
                pressed:  false,
                down:     false,
                hovering: false,
            }
        }
    }
}

// ------------------

fn new_button(text: &str, disabled: bool) -> State {
    State {
        disabled,
        pressed:  false,
        down:     false,
        hovering: false,
        scroll:   0,
        variant: ElementVariant::Button { text: text.to_owned() },
    }
}

fn button_internal<'a, P: Placer>(
    button: Button,
    placer: &mut P,
    app: &'a mut App,
) -> Option<&'a State> {
    // @Maybe add text using the Text method instead of calculating everything

    let id = Id::new(button.text).add("#__button");

    let line_padding = placer.ui(app).style.line_padding;
    placer.add_padding(line_padding, app);

    // Calculate element size
    let size = Vec2i {
        x: placer.draw_width(app),
        y: placer.ui(app).line_draw_height()
    };
    let layout = placer.place_element(id, size, app);

    placer.remove_padding(app);

    if layout.is_none() { return None; }
    let layout = layout.unwrap();

    app.ui_system.states.entry(id)
        .and_modify(|state| {
            state.disabled = button.disabled;
        })
        .or_insert_with(|| new_button(button.text, button.disabled));

    if !button.disabled {
        Some(app.update_state_interaction(id, layout))
    } else {
        Some(app.ui_system.states.get(&id).unwrap())
    }
}
