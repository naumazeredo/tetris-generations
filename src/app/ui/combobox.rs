use crate::app::App;
use super::*;

pub static COMBOBOX_TEST_OPTIONS: &'static[&'static str] = &["option 1", "option 2", "option 3", "option 4"];

fn new_combobox(index: usize, text: &str) -> State {
    State {
        pressed: false,
        down: false,
        hovering: false,
        variant: ElementVariant::Combobox {
            index,
            text: text.to_owned(),
        },
    }
}

fn new_combobox_option(selected: bool, text: &str) -> State {
    State {
        pressed: false,
        down: false,
        hovering: false,
        variant: ElementVariant::ComboboxOption {
            selected,
            text: text.to_owned(),
        },
    }
}

impl<S> App<'_, S> {
    pub fn combobox(&mut self, label: &str, index: &mut usize, options: &[&str]) -> bool {
        // Add label
        self.text(label);
        self.same_line();

        // Combobox
        let id = Id::new(label).add("#combobox");

        let ui = &self.ui_system.uis.last().unwrap();
        let size = Vec2i {
            x: ui.style.box_width as i32,
            y: ui.style.line_height as i32,
        };
        let layout = self.new_layout_right(size);
        self.add_element(id, layout);

        // Modal options
        let mut updated = false;
        if let Some(modal_id) = self.ui_system.modal_open {
            if modal_id == id {
                options.iter()
                    .enumerate()
                    .for_each(|(i, option_text)| {
                        let id = id.add(&format!("#{}", i));
                        let layout = Layout {
                            pos: layout.pos + Vec2i { x: 0, y: size.y * (i+1) as i32 },
                            size
                        };
                        self.add_modal_element(id, layout);

                        self.ui_system.states.entry(id)
                            .and_modify(|state| {
                                // Update the value
                                if let ElementVariant::ComboboxOption {
                                    selected: state_selected,
                                    text: _state_text,
                                } = &mut state.variant {
                                    *state_selected = *index == i;

                                    // @TODO update string in case of change: compare hashed strings
                                }
                            })
                            .or_insert_with(|| new_combobox_option(*index == i, &option_text));

                        let state = self.update_modal_state_interaction(id, layout);
                        if state.pressed {
                            *index = i;
                            updated = true;
                        }
                    });
            }
        }

        // State update
        self.ui_system.states.entry(id)
            .and_modify(|state| {
                // Update the value
                if let ElementVariant::Combobox {
                    index: state_index,
                    text: state_text,
                } = &mut state.variant {
                    if *state_index != *index {
                        *state_text = options[*index].to_owned();
                        *state_index = *index;
                    }
                }
            })
            .or_insert_with(|| new_combobox(*index, options[*index]));

        let state = self.update_state_interaction(id, layout);
        if state.pressed {
            self.ui_system.modal_change = Some(id);
        }

        updated
    }
}
