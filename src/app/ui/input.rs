use crate::app::{
    App,
    ImDraw,
};
use super::*;

use core::str::FromStr;
use std::fmt::Display;

fn add_input_integer_unsigned<T>(
    input_state: &mut String,
    new_input: &String,
    min: Option<T>,
    max: Option<T>
)
where
    T: FromStr + Ord + Display
{
    match new_input.parse::<T>() {
        Ok(mut num) => {
            if let Some(x) = min { num = std::cmp::max(num, x); }
            if let Some(x) = max { num = std::cmp::min(num, x); }
            *input_state = format!("{}", num);
        }
        Err(_) => {},
    }
}

fn add_input_integer_signed<T>(
    input_state: &mut String,
    new_input: &String,
    min: Option<T>,
    max: Option<T>
)
where
    T: FromStr + Ord + Display
{
    // Accept the minus sign by itself
    if new_input == "-" {
        /*
        if let Some(x) = min {
            if x > 0 { return; }
        }
        */
        *input_state = new_input.to_owned();
    } else {
        // @FixMe without min/max, under/overflowing i32 won't saturate to min/max i32
        match new_input.parse::<T>() {
            Ok(mut num) => {
                if let Some(x) = min { num = std::cmp::max(num, x); }
                if let Some(x) = max { num = std::cmp::min(num, x); }
                *input_state = format!("{}", num);
            }
            Err(_) => {},
        }
    }
}

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

            InputVariant::I8  { min, max, .. } => add_input_integer_signed::<i8> (&mut self.input_state, &new_input, min, max),
            InputVariant::I16 { min, max, .. } => add_input_integer_signed::<i16>(&mut self.input_state, &new_input, min, max),
            InputVariant::I32 { min, max, .. } => add_input_integer_signed::<i32>(&mut self.input_state, &new_input, min, max),
            InputVariant::I64 { min, max, .. } => add_input_integer_signed::<i64>(&mut self.input_state, &new_input, min, max),

            InputVariant::U8  { min, max, .. } => add_input_integer_unsigned::<u8> (&mut self.input_state, &new_input, min, max),
            InputVariant::U16 { min, max, .. } => add_input_integer_unsigned::<u16>(&mut self.input_state, &new_input, min, max),
            InputVariant::U32 { min, max, .. } => add_input_integer_unsigned::<u32>(&mut self.input_state, &new_input, min, max),
            InputVariant::U64 { min, max, .. } => add_input_integer_unsigned::<u64>(&mut self.input_state, &new_input, min, max),
        }
    }
}

#[derive(Copy, Clone, Debug, ImDraw)]
pub(super) enum InputVariant {
    Str { max_length: usize },
    I8  { value: i8,  min: Option<i8>,  max: Option<i8>  },
    U8  { value: u8,  min: Option<u8>,  max: Option<u8>  },
    I16 { value: i16, min: Option<i16>, max: Option<i16> },
    U16 { value: u16, min: Option<u16>, max: Option<u16> },
    I32 { value: i32, min: Option<i32>, max: Option<i32> },
    U32 { value: u32, min: Option<u32>, max: Option<u32> },
    I64 { value: i64, min: Option<i64>, max: Option<i64> },
    U64 { value: u64, min: Option<u64>, max: Option<u64> },
}

impl State {
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
    fn input_str_internal(&mut self, label: &str, value: &mut String, max_length: usize) {
        // Add label
        self.text(label);
        self.same_line();

        let id = Id::new(label).add("#input");

        let ui = &self.ui_system.uis.last().unwrap();
        let size = Vec2i {
            x: ui.style.input_box_width as i32,
            y: ui.style.font_size as i32 + 2 * ui.style.box_padding,
        };
        let layout = Layout {
            pos: Vec2i {
                x: self.ui_system.cursor.x,
                y: self.ui_system.cursor.y - ui.style.box_padding,
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

// @TODO proc_macro this to remove all these function names
macro_rules! input_variant_integer_impl {
    (
        $internal_fn:ident,
        $build_fn:ident,
        $pub_fn:ident,
        $pub_range_fn:ident,
        $var:ident,
        $type:ident
    ) => {
        fn $build_fn(value: $type, min: Option<$type>, max: Option<$type>) -> State {
            State {
                pressed: false,
                down: false,
                hovering: false,
                variant: ElementVariant::Input {
                    input_focus: Some(false),
                    input_complete: false,

                    value_str: format!("{}", value),
                    variant: InputVariant::$var { value, min, max },
                },
            }
        }

        impl<S> App<'_, S> {
            fn $internal_fn(
                &mut self,
                label: &str,
                value: &mut $type,
                min: Option<$type>,
                max: Option<$type>
            ) {
                // Add label
                self.text(label);
                self.same_line();

                let id = Id::new(label).add("#input");

                let ui = &self.ui_system.uis.last().unwrap();
                let size = Vec2i {
                    x: ui.style.input_box_width as i32,
                    y: ui.style.font_size as i32 + 2 * ui.style.box_padding,
                };
                let layout = Layout {
                    pos: Vec2i {
                        x: self.ui_system.cursor.x,
                        y: self.ui_system.cursor.y - ui.style.box_padding,
                    },
                    size
                };

                self.ui_system.states.entry(id)
                    .and_modify(|state| {
                        // Update the value
                        if let ElementVariant::Input {
                            value_str,
                            variant: InputVariant::$var { value: v, .. },
                            ..
                        } = &mut state.variant {
                            if *v != *value {
                                *v = *value;
                                *value_str = format!("{}", *value);
                            }
                        }
                    })
                    .or_insert_with(|| $build_fn(*value, min, max));

                let state = self.update_state_interaction(id, layout);
                if let ElementVariant::Input {
                    input_complete: true,
                    value_str,
                    variant: InputVariant::$var { value: v, .. },
                    ..
                } = &mut state.variant {
                    *value = $type::from_str_radix(&value_str, 10).unwrap_or_default();
                    *value_str = format!("{}", *value);
                    *v = *value;
                }

                self.add_element(id, layout);
            }

            pub fn $pub_fn(&mut self, label: &str, value: &mut $type) {
                self.$internal_fn(label, value, None, None);
            }

            pub fn $pub_range_fn(&mut self, label: &str, value: &mut $type, min: $type, max: $type) {
                self.$internal_fn(label, value, Some(min), Some(max));
            }
        }
    }
}

input_variant_integer_impl!(input_i8_internal,  new_input_i8,  input_i8,  input_i8_range,  I8,  i8);
input_variant_integer_impl!(input_i16_internal, new_input_i16, input_i16, input_i16_range, I16, i16);
input_variant_integer_impl!(input_i32_internal, new_input_i32, input_i32, input_i32_range, I32, i32);
input_variant_integer_impl!(input_i64_internal, new_input_i64, input_i64, input_i64_range, I64, i64);

input_variant_integer_impl!(input_u8_internal,  new_input_u8,  input_u8,  input_u8_range,  U8,  u8);
input_variant_integer_impl!(input_u16_internal, new_input_u16, input_u16, input_u16_range, U16, u16);
input_variant_integer_impl!(input_u32_internal, new_input_u32, input_u32, input_u32_range, U32, u32);
input_variant_integer_impl!(input_u64_internal, new_input_u64, input_u64, input_u64_range, U64, u64);
