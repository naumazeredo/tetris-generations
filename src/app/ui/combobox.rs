use crate::app::App;
use super::*;

const COMBOBOX_DEFAULT_SCROLL_SIZE: usize = 5;

pub struct ComboboxState {
    pub pressed: bool,
    pub down: bool,
    pub hovering: bool,
    pub changed: bool,
}

pub struct Combobox<'a, T>
where
    T: AsRef<str>
{
    label: &'a str,
    options: &'a [T],
    box_width: Option<u32>,
    scroll_size: usize,
}

impl<'a, T> Combobox<'a, T>
where
    T: AsRef<str>
{
    pub fn new(
        label: &str,
        options: &[T],
        index: &mut usize,
        app: &mut App,
    ) -> ComboboxState {
        Combobox::builder(label, options).build(index, app)
    }

    pub fn builder<'b: 'a>(label: &'b str, options: &'b [T]) -> Self {
        Self {
            label,
            options,
            box_width: None,
            scroll_size: COMBOBOX_DEFAULT_SCROLL_SIZE,
        }
    }

    pub fn box_width(self, box_width: u32) -> Self {
        Self {
            box_width: Some(box_width),
            ..self
        }
    }

    pub fn scroll_size(self, scroll_size: usize) -> Self {
        Self {
            scroll_size,
            ..self
        }
    }

    pub fn build(self, index: &mut usize, app: &mut App) -> ComboboxState {
        let state = app.combobox_internal(index, self);

        if let ElementVariant::Combobox {
            changed,
            ..
        } = state.variant {
            ComboboxState {
                pressed:  state.pressed,
                down:     state.down,
                hovering: state.hovering,
                changed,
            }
        } else {
            unreachable!();
        }
    }
}

// ---------------

fn new_combobox(index: usize, text: &str) -> State {
    State {
        pressed: false,
        down: false,
        hovering: false,
        variant: ElementVariant::Combobox {
            index,
            text: text.to_owned(),
            changed: false,
            scroll_top_index: 0,
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

fn new_scrollbar() -> State {
    State {
        pressed: false,
        down: false,
        hovering: false,
        variant: ElementVariant::Scrollbar,
    }
}

impl App<'_> {
    fn combobox_internal<T: AsRef<str>>(
        &mut self,
        index: &mut usize,
        combobox: Combobox<T>
    ) -> &mut State {
        // Add label
        self.text(combobox.label);
        self.same_line();

        // Combobox
        let id = Id::new(combobox.label).add("#__combobox");

        let ui = &self.ui_system.uis.last().unwrap();
        let box_width = combobox.box_width.unwrap_or((ui.draw_width() / 2) as u32);
        let size = Vec2i {
            x: box_width as i32,
            y: ui.style.line_height as i32,
        };
        let layout = self.new_layout_right(size);
        self.add_element(id, layout);

        // Be sure index is in a valid value
        *index = (*index).clamp(0, combobox.options.len() - 1);
        // @TODO show some default value in case option slice is empty
        assert!(combobox.options.len() > 0);

        // Get scroll top index
        let mut scroll_top_index;
        if let Some(
            State {
                variant: ElementVariant::Combobox {
                    scroll_top_index: top_index,
                    ..
                },
                ..
            }
        ) = self.ui_system.states.get(&id) {
            scroll_top_index = *top_index;
        } else {
            scroll_top_index = 0;
        }

        // Modal options
        let modal_id = id.add("#__modal");
        let mut changed = false;
        if let Some(open_modal_id) = self.ui_system.modal_open {
            if open_modal_id == modal_id {
                // Update top index on mouse scroll
                let modal_layout = Layout {
                    pos: layout.pos + Vec2i { x: 0, y: size.y },
                    size: Vec2i { x: size.x, y: combobox.scroll_size as i32 * size.y }
                };

                let scroll = self.mouse_scroll();
                if combobox.options.len() > combobox.scroll_size &&
                    scroll != 0 &&
                    self.is_mouse_hovering_layout(modal_layout)
                {
                    scroll_top_index = (scroll_top_index as i32 - scroll)
                        .clamp(
                            0,
                            combobox.options.len() as i32 - combobox.scroll_size as i32
                        ) as usize;
                }

                // Create modal elements for options that are visible
                combobox.options.iter()
                    .enumerate()
                    .skip(scroll_top_index)
                    .take(combobox.scroll_size)
                    .for_each(|(i, option_text)| {
                        let delta_index = i as i32 - scroll_top_index as i32;

                        let id = modal_id.add(&format!("#__index_{}", i));
                        let layout = Layout {
                            pos: layout.pos + Vec2i { x: 0, y: size.y * (delta_index + 1) },
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
                            .or_insert_with(|| new_combobox_option(*index == i, option_text.as_ref()));

                        let state = self.update_modal_state_interaction(id, layout);
                        if state.pressed {
                            *index = i;
                            changed = true;
                        }
                    });

                // Scroll bar
                if combobox.options.len() > combobox.scroll_size {
                    let ui = &self.ui_system.uis.last().unwrap();

                    let size_y = (
                        modal_layout.size.y *
                        combobox.scroll_size as i32 / combobox.options.len() as i32
                    ) as i32;

                    let pos_y = scroll_top_index as i32 * modal_layout.size.y / combobox.options.len() as i32;

                    let layout = Layout {
                        pos: modal_layout.pos + Vec2i {
                            x: size.x - ui.style.scrollbar_width as i32,
                            y: pos_y,
                        },
                        size: Vec2i {
                            x: ui.style.scrollbar_width as i32,
                            y: size_y,
                        }
                    };

                    let id = modal_id.add("#__scrollbar");
                    self.add_modal_element(id, layout);
                    self.ui_system.states.entry(id).or_insert_with(|| new_scrollbar());
                }
            }
        } else {
            // In case the modal is not open, reset the top index
            scroll_top_index = 0;
        }

        // State update
        self.ui_system.states.entry(id)
            .and_modify(|state| {
                // Update the value
                if let ElementVariant::Combobox {
                    index: state_index,
                    text: state_text,
                    changed,
                    scroll_top_index: state_scroll_top_index,
                } = &mut state.variant {
                    *state_scroll_top_index = scroll_top_index;
                    if *state_index != *index {
                        *state_text = combobox.options[*index].as_ref().to_string();
                        *state_index = *index;
                        *changed = true;
                    } else {
                        *changed = false;
                    }
                }
            })
            .or_insert_with(|| new_combobox(*index, combobox.options[*index].as_ref()));

        let state = self.update_state_interaction(id, layout);
        if state.pressed {
            self.ui_system.modal_change = Some(modal_id);
        }

        // borrow-checker doesn't allow returning state (it's not that smart...)
        self.ui_system.states.get_mut(&id).unwrap()
    }

    /*
    fn combowheel_internal<T: AsRef<str>>(
        &mut self,
        index: &mut usize,
        combobox: Combobox<T>
    ) -> &mut State {
        // Add label
        self.text(combobox.label);
        self.same_line();

        // Combobox
        let id = Id::new(combobox.label).add("#__combobox");

        let ui = &self.ui_system.uis.last().unwrap();
        let box_width = combobox.box_width.unwrap_or((ui.draw_width() / 2) as u32);
        let size = Vec2i {
            x: box_width as i32,
            y: ui.style.line_height as i32,
        };
        let layout = self.new_layout_right(size);
        self.add_element(id, layout);

        // Be sure index is in a valid value
        *index = (*index).clamp(0, combobox.options.len() - 1);
        // @TODO show some default value in case option slice is empty
        assert!(combobox.options.len() > 0);

        // Modal options
        let modal_id = id.add("#__modal");
        let mut changed = false;
        if let Some(open_modal_id) = self.ui_system.modal_open {
            if open_modal_id == modal_id {
                combobox.options.iter()
                    .skip((*index).saturating_sub(COMBOWHEEL_PREVIEW_AMOUNT))
                    .take(2 * COMBOWHEEL_PREVIEW_AMOUNT + 1)
                    .enumerate()
                    .for_each(|(i, option_text)| {
                        let delta_index = (*index) as i32 - i as i32;

                        let id = modal_id.add(&format!("#__index_{}", i));
                        let layout = Layout {
                            pos: layout.pos + Vec2i { x: 0, y: size.y * delta_index as i32 },
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
                            .or_insert_with(|| new_combobox_option(*index == i, option_text.as_ref()));

                        let state = self.update_modal_state_interaction(id, layout);
                        if state.pressed {
                            *index = i;
                            changed = true;
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
                    changed,
                } = &mut state.variant {
                    if *state_index != *index {
                        *state_text = combobox.options[*index].as_ref().to_string();
                        *state_index = *index;
                        *changed = true;
                    } else {
                        *changed = false;
                    }
                }
            })
            .or_insert_with(|| new_combobox(*index, combobox.options[*index].as_ref()));

        let state = self.update_state_interaction(id, layout);
        if state.pressed {
            self.ui_system.modal_change = Some(modal_id);
        }

        // borrow-checker doesn't allow returning state (it's not so smart...)
        self.ui_system.states.get_mut(&id).unwrap()
    }
    */
}
