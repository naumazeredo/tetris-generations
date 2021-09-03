use crate::app::App;
use super::*;

fn new_checkbox(value: bool) -> State {
    State {
        pressed: false,
        down: false,
        hovering: false,
        variant: ElementVariant::Checkbox { value },
    }
}

impl App<'_> {
    pub fn checkbox(&mut self, label: &str, value: &mut bool) {
        // Add label
        self.text(label);
        self.same_line();

        // Update/create box state
        let id = Id::new(label).add("#checkbox");

        let ui = &self.ui_system.uis.last().unwrap();
        let checkbox_box_size = ui.style.line_height;

        let size = Vec2i { x: checkbox_box_size as i32, y: checkbox_box_size as i32 };
        let layout = self.new_layout_right(size);

        self.ui_system.states.entry(id)
            .and_modify(|state| {
                match &mut state.variant {
                    ElementVariant::Checkbox { value: v } => *v = *value,
                    _ => unreachable!()
                }
            })
            .or_insert_with(|| new_checkbox(*value));

        let state = self.update_state_interaction(id, layout);
        if state.pressed {
            *value = !*value;

            // @XXX should we update the saved state or just be delayed by 1 frame?
            match &mut state.variant {
                ElementVariant::Checkbox { value: v } => *v = !*v,
                _ => unreachable!()
            }
        }

        self.add_element(id, layout);
    }
}
