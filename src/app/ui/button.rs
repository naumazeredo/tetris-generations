use crate::app::App;
use super::*;

impl State {
    fn new_button(text: &str) -> Self {
        Self {
            pressed: false,
            down: false,
            hovering: false,
            variant: ElementVariant::Button { text: text.to_owned() },
        }
    }
}

impl<S> App<'_, S> {
    pub fn button(&mut self, text: &str) -> bool {
        // @Maybe add text using the app.text method instead of calculating everything

        let id = Id::new(text);

        // @TODO cleanup these ui.last_mut().unwrap() calls
        // Calculate element size
        let button_padding = self.ui_system.uis.last().unwrap().style.button_padding;
        let padding = Vec2i { x: button_padding, y: button_padding };
        let size = self.calculate_text_size(text) + 2 * padding;
        let layout = self.new_layout(size);

        self.add_element(id, layout);

        self.ui_system.states.entry(id)
            .or_insert_with(|| State::new_button(text));

        let state = self.update_state_interaction(id, layout);
        state.pressed
    }
}
