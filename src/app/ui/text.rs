use crate::app::App;
use super::*;

fn new_text(text: &str) -> State {
    State {
        pressed: false,
        down: false,
        hovering: false,
        variant: ElementVariant::Text { text: text.to_owned() },
    }
}

impl<S> App<'_, S> {
    pub fn text(&mut self, text: &str) {
        let id = Id::new(text);
        let layout = self.new_layout(self.calculate_text_size(text));

        self.add_element(id, layout);

        // @TODO check if text should be updated
        //       Maybe create a function that compares the strings (or the string ids) and swap the
        //       contents in case they are different
        self.ui_system.states.entry(id)
            .or_insert_with(|| new_text(text));
    }
}
