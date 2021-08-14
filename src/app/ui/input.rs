use crate::app::{
    App,
    ImDraw,
};
use super::*;

impl UiSystem {
    // Verifies if the current input state appending the new text is valid, if it is, update the
    // input state, otherwise do nothing.
    pub(super) fn add_input(&mut self, text: &str) {
        let new_input = [self.input_state.as_str(), text].concat();

        match self.input_variant {
            InputVariant::Str { max_length } => {
                if new_input.len() <= max_length {
                    self.input_state = new_input;
                }
            }

            InputVariant::I32 { min, max, .. } => {
                // Accept the minus sign by itself
                if new_input == "-" {
                    if let Some(x) = min {
                        if x >= 0 { return; }
                    }
                    self.input_state = new_input.to_owned();
                } else {
                    // @FixMe without min/max, under/overflowing i32 won't saturate to min/max i32
                    match new_input.parse::<i32>() {
                        Ok(mut num) => {
                            if let Some(x) = min { num = std::cmp::max(num, x); }
                            if let Some(x) = max { num = std::cmp::min(num, x); }

                            self.input_state = format!("{}", num);
                        }
                        Err(_) => {},
                    }
                }
            }

            /*
            InputVariant::U32 { min, max } => {
                match new_input.parse::<u32>() {
                    Ok(num) => self.input_state = format!("{}", num),
                    Err(_) => {},
                }
            }
            */
        }
    }
}

#[derive(Copy, Clone, Debug, ImDraw)]
pub(super) enum InputVariant {
    Str { max_length: usize },
    I32 { value: i32, min: Option<i32>, max: Option<i32> },
    //U32 { value: u32, min: Option<u32>, max: Option<u32> },
}

impl State {
    // @TODO macro this to multiple types
    fn new_input_i32(value: i32, min: Option<i32>, max: Option<i32>) -> Self {
        Self {
            pressed: false,
            down: false,
            hovering: false,
            variant: ElementVariant::Input {
                input_focus: Some(false),
                input_complete: false,

                value_str: format!("{}", value),
                variant: InputVariant::I32 { value, min, max },
            },
        }
    }

    fn new_input_str(value: &str, max_length: usize) -> Self {
        Self {
            pressed: false,
            down: false,
            hovering: false,
            variant: ElementVariant::Input {
                input_focus: Some(false),
                input_complete: false,

                value_str: value.to_owned(),
                variant: InputVariant::Str { max_length },
            },
        }
    }
}

impl<S> App<'_, S> {
    //------------------
    // Input integer
    //------------------

    fn input_i32_internal(&mut self, label: &str, value: &mut i32, min: Option<i32>, max: Option<i32>) {
        // Add label
        self.text(label);
        self.same_line();

        let id = Id::new(label).add("#input");

        let ui = &self.ui_system.uis.last().unwrap();
        let size = Vec2i {
            x: ui.style.input_box_width as i32,
            y: ui.style.font_size as i32 + 2 * ui.style.input_box_padding,
        };
        let layout = Layout {
            pos: Vec2i {
                x: self.ui_system.cursor.x,
                y: self.ui_system.cursor.y - ui.style.input_box_padding,
            },
            size
        };

        self.ui_system.states.entry(id)
            .and_modify(|state| {
                // Update the value
                if let ElementVariant::Input {
                    value_str,
                    variant,
                    ..
                } = &mut state.variant {
                    if let InputVariant::I32 { value: v, .. } = variant {
                        if *v != *value {
                            *v = *value;
                            *value_str = format!("{}", *value);
                        }
                    } else {
                        unreachable!();
                    }
                }
            })
            .or_insert_with(|| State::new_input_i32(*value, min, max));

        let state = self.update_state_interaction(id, layout);
        if let ElementVariant::Input {
            input_complete: true,
            value_str,
            variant,
            ..
        } = &mut state.variant {
            *value = i32::from_str_radix(&value_str, 10).unwrap_or_default();
            *value_str = format!("{}", *value);

            if let InputVariant::I32 { value: v, .. } = variant {
                *v = *value;
            } else {
                unreachable!();
            }
        }

        self.add_element(id, layout);
    }

    pub fn input_i32(&mut self, label: &str, value: &mut i32) {
        self.input_i32_internal(label, value, None, None);
    }

    pub fn input_i32_range(&mut self, label: &str, value: &mut i32, min: i32, max: i32) {
        self.input_i32_internal(label, value, Some(min), Some(max));
    }

    //------------------
    // Input string
    //------------------

    fn input_str_internal(&mut self, label: &str, value: &mut String, max_length: usize) {
        // Add label
        self.text(label);
        self.same_line();

        let id = Id::new(label).add("#input");

        let ui = &self.ui_system.uis.last().unwrap();
        let size = Vec2i {
            x: ui.style.input_box_width as i32,
            y: ui.style.font_size as i32 + 2 * ui.style.input_box_padding,
        };
        let layout = Layout {
            pos: Vec2i {
                x: self.ui_system.cursor.x,
                y: self.ui_system.cursor.y - ui.style.input_box_padding,
            },
            size
        };

        // @TODO we should update the input state in case referenced value changed
        self.ui_system.states.entry(id)
            .or_insert_with(|| State::new_input_str(&value, max_length));

        let state = self.update_state_interaction(id, layout);
        if let ElementVariant::Input {
            input_complete: true,
            value_str,
            ..
        } = &mut state.variant {
            *value = value_str.clone();
        }

        self.add_element(id, layout);
    }

    pub fn input_str(&mut self, label: &str, value: &mut String) {
        self.input_str_internal(label, value, 64);
    }

    pub fn input_str_with_max_length(&mut self, label: &str, value: &mut String, max_length: usize) {
        self.input_str_internal(label, value, max_length);
    }
}
