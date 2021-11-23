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
    disabled: bool,
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
            disabled: false,
        }
    }

    // @XXX this is a custom layout, that will only be implemented throughtout widgets later
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

    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            disabled,
            ..self
        }
    }

    #[inline(always)] pub fn build(
        self,
        index: &mut usize,
        app: &mut App
    ) -> ComboboxState {
        self.build_with_placer(index, &mut app.ui_system.top_ui().index(), app)
    }

    #[inline(always)] pub fn build_with_placer<P: Placer>(
        self,
        index: &mut usize,
        placer: &mut P,
        app: &mut App
    ) -> ComboboxState {
        if let Some(state) = combobox_internal(index, self, placer, app) {
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
        } else {
            ComboboxState {
                pressed:  false,
                down:     false,
                hovering: false,
                changed:  false,
            }
        }
    }
}

// ---------------

fn new_combobox(text: &str, index: usize, disabled: bool) -> State {
    State {
        disabled,
        pressed:  false,
        down:     false,
        hovering: false,
        scroll:   0,
        variant: ElementVariant::Combobox {
            index,
            text: text.to_owned(),
            changed: false,
            scroll_top_index: 0,
        },
    }
}

fn new_combobox_option(text: &str, selected: bool) -> State {
    State {
        disabled: false, // Never disabled?
        pressed:  false,
        down:     false,
        hovering: false,
        scroll:   0,
        variant: ElementVariant::ComboboxOption {
            selected,
            text: text.to_owned(),
        },
    }
}

fn new_scrollbar() -> State {
    State {
        disabled: false, // Never disabled?
        pressed:  false,
        down:     false,
        hovering: false,
        scroll:   0,
        variant: ElementVariant::Scrollbar,
    }
}

fn combobox_internal<'a, T: AsRef<str>, P: Placer>(
    index: &mut usize,
    combobox: Combobox<T>,
    placer: &mut P,
    app: &'a mut App,
) -> Option<&'a mut State> {
    let ui = placer.ui(app);
    let spacing = ui.style.spacing;
    let line_padding = ui.style.line_padding;

    placer.add_padding(line_padding, app);
    let col_width = (placer.draw_width(app) - spacing) / 2;

    // Add label
    text_internal(
        Text::builder(combobox.label)
            .disabled(combobox.disabled)
            .max_width(col_width as u32),
        placer,
        app
    );

    placer.same_line(app);
    placer.add_spacing(app);

    // Combobox
    let id = Id::new(combobox.label).add("#__combobox");

    let ui = placer.ui(app);
    let box_width = combobox.box_width.unwrap_or(col_width as u32);
    let size = Vec2i {
        x: box_width as i32,
        y: ui.line_draw_height() as i32,
    };
    let layout = placer.place_element(id, size, app);

    placer.remove_padding(app);

    if layout.is_none() { return None; }
    let layout = layout.unwrap();

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
    ) = app.ui_system.states.get(&id) {
        scroll_top_index = *top_index;
    } else {
        scroll_top_index = 0;
    }

    // Modal options
    let modal_id = id.add("#__modal");
    let mut changed = false;
    if let Some(open_modal_id) = app.ui_system.modal_open {
        if open_modal_id == modal_id {
            if combobox.disabled {
                // Disabled modal in case combobox was disabled
                app.ui_system.modal_change = None;
            } else {
                // Update top index on mouse scroll
                let modal_layout = Layout {
                    pos: layout.pos + Vec2i { x: 0, y: size.y },
                    size: Vec2i { x: size.x, y: combobox.scroll_size as i32 * size.y }
                };

                let scroll = app.mouse_scroll();
                if combobox.options.len() > combobox.scroll_size &&
                    scroll != 0 &&
                    app.is_mouse_hovering_layout(modal_layout)
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

                        let id = modal_id.add(&format!("#{}", option_text.as_ref()));
                        let layout = Layout {
                            pos: layout.pos + Vec2i { x: 0, y: size.y * (delta_index + 1) },
                            size
                        };
                        let ui = placer.ui(app);
                        ui.add_modal_element(id, layout);

                        app.ui_system.states.entry(id)
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
                            .or_insert_with(|| new_combobox_option(option_text.as_ref(), *index == i));

                        let state = app.update_modal_state_interaction(id, layout);
                        if state.pressed {
                            *index = i;
                            changed = true;
                        }
                    });

                // Scroll bar
                if combobox.options.len() > combobox.scroll_size {
                    let ui = placer.ui(app);

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
                    let ui = placer.ui(app);
                    ui.add_modal_element(id, layout);
                    app.ui_system.states.entry(id).or_insert_with(|| new_scrollbar());
                }
            }
        }
    } else {
        // In case the modal is not open, reset the top index
        scroll_top_index = 0;
    }

    // State update
    app.ui_system.states.entry(id)
        .and_modify(|state| {
            state.disabled = combobox.disabled;

            // Update the value
            if let ElementVariant::Combobox {
                index: state_index,
                text: state_text,
                changed,
                scroll_top_index: state_scroll_top_index,
            } = &mut state.variant {
                *state_scroll_top_index = scroll_top_index;

                // @XXX is updating the options a usual thing to do? If it is, we need to detect
                //      that the string changed (if it's in the same index, does it count as
                //      changed?)
                if *state_index != *index {
                    *state_text = combobox.options[*index].as_ref().to_string();
                    *state_index = *index;
                    *changed = true;
                } else {
                    *changed = false;
                }
            }
        })
        .or_insert_with(||
            new_combobox(combobox.options[*index].as_ref(), *index, combobox.disabled)
        );

    if !combobox.disabled {
        let state = app.update_state_interaction(id, layout);
        if state.pressed {
            app.ui_system.modal_change = Some(modal_id);
        }
    }

    // borrow-checker doesn't allow returning state (it's not that smart...)
    Some(app.ui_system.states.get_mut(&id).unwrap())
}
