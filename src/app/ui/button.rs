use crate::app::App;
use super::*;

fn new_button(text: &str) -> State {
    State {
        pressed: false,
        down: false,
        hovering: false,
        variant: ElementVariant::Button { text: text.to_owned() },
    }
}

impl App<'_> {
    pub fn button(&mut self, text: &str) -> bool {
        // @Maybe add text using the app.text method instead of calculating everything

        let id = Id::new(text).add("#__button");

        // @TODO cleanup these ui.last_mut().unwrap() calls
        // Calculate element size
        let ui = &self.ui_system.uis.last().unwrap();
        let size = Vec2i {
            x: ui.layout.size.x - 2 * ui.style.padding,
            y: ui.style.line_height
        };
        let layout = self.new_layout(size);

        self.add_element(id, layout);

        self.ui_system.states.entry(id)
            .or_insert_with(|| new_button(text));

        let state = self.update_state_interaction(id, layout);
        state.pressed
    }
}
