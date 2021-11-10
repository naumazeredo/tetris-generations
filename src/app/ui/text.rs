use crate::app::App;
use super::*;

pub struct Text<'a> {
    text: &'a str,
    disabled: bool,
    //max_width: Option<u32>,
}

impl<'a> Text<'a> {
    pub fn new(text: &str, app: &mut App) {
        Text::builder(text).build(app)
    }

    pub fn builder<'b: 'a>(text: &'b str) -> Self {
        Self {
            text,
            disabled: false,
            //max_width: None,
        }
    }

    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            disabled,
            ..self
        }
    }

    pub fn build(self, app: &mut App) {
        app.text_internal(self);
    }
}

// ----

fn new_text(text: &str, disabled: bool) -> State {
    State {
        pressed: false,
        down: false,
        hovering: false,
        disabled,
        variant: ElementVariant::Text { text: text.to_owned() },
    }
}

impl App<'_> {
    pub(super) fn text_internal(&mut self, text: Text) {
        let id = Id::new(text.text).add("#__text");

        let size = self.calculate_text_size(text.text);
        let ui = &self.ui_system.uis.last().unwrap();
        let layout = Layout {
            pos: Vec2i {
                x: self.ui_system.cursor.x,
                y: self.ui_system.cursor.y + ui.style.box_padding,
            },
            size
        };

        self.add_element(id, layout);

        // @TODO check if text should be updated
        //       Maybe create a function that compares the strings (or the string ids) and swap the
        //       contents in case they are different
        self.ui_system.states.entry(id)
            .and_modify(|state| {
                state.disabled = text.disabled;
            })
            .or_insert_with(|| new_text(text.text, text.disabled));
    }
}
