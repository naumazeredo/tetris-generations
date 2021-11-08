use crate::app::App;
use super::*;

pub struct Text<'a> {
    text: &'a str,
    //max_width: Option<u32>,
}

impl<'a> Text<'a> {
    pub fn new(text: &str, app: &mut App) {
        Text::builder(text).build(app)
    }

    pub fn builder<'b: 'a>(text: &'b str) -> Self {
        Self {
            text,
            //max_width: None,
        }
    }

    pub fn build(self, app: &mut App) {
        let id = Id::new(self.text);
        app.text_with_id(id, self.text);
    }
}

// ----

fn new_text(text: &str) -> State {
    State {
        pressed: false,
        down: false,
        hovering: false,
        variant: ElementVariant::Text { text: text.to_owned() },
    }
}

impl App<'_> {
    pub fn text(&mut self, text: &str) {
        let id = Id::new(text);
        self.text_with_id(id, text);
    }

    pub(in super) fn text_with_id(&mut self, id: Id, text: &str) {
        let size = self.calculate_text_size(text);
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
            .or_insert_with(|| new_text(text));
    }
}
