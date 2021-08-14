use crate::app::{
    App,
    ImDraw,
};
use super::*;

#[derive(Copy, Clone, Debug, ImDraw)]
pub(super) enum SliderVariant {
    I8   { value: i8,   min: i8,   max: i8 },
    U8   { value: u8,   min: u8,   max: u8 },
    I16  { value: i16,  min: i16,  max: i16 },
    U16  { value: u16,  min: u16,  max: u16 },
    I32  { value: i32,  min: i32,  max: i32 },
    U32  { value: u32,  min: u32,  max: u32 },
    I64  { value: i64,  min: i64,  max: i64 },
    U64  { value: u64,  min: u64,  max: u64 },
    //I128 { value: i128, min: i128, max: i128 },
    //U128 { value: u128, min: u128, max: u128 },
}

macro_rules! slider_variant_integer_impl {
    ($state_fn:ident, $pub_fn:ident, $var:ident, $type:ident) => {
        impl State {
            fn $state_fn(value: $type, min: $type, max: $type) -> Self {
                let mut percent = (value - min) as f32 / (max - min) as f32;
                if percent < 0.0 { percent = 0.0; }
                if percent > 1.0 { percent = 1.0; }

                Self {
                    pressed: false,
                    down: false,
                    hovering: false,
                    variant: ElementVariant::Slider {
                        percent,
                        variant: SliderVariant::$var { value, min, max },
                    },
                }
            }
        }

        impl<S> App<'_, S> {
            // @TODO accept a format string for the type
            pub fn $pub_fn(&mut self, label: &str, value: &mut $type, min: $type, max: $type) {
                // Add label
                self.text(label);
                self.same_line();

                let id = Id::new(label).add("#slider");

                let ui = &self.ui_system.uis.last().unwrap();
                let size = Vec2i {
                    x: ui.style.slider_box_width as i32,
                    y: ui.style.slider_box_height as i32,
                };
                let layout = Layout {
                    pos: Vec2i {
                        x: self.ui_system.cursor.x,
                        y: self.ui_system.cursor.y +
                            (ui.style.font_size as i32 - ui.style.slider_box_height as i32) / 2,
                    },
                    size
                };

                // @TODO we should update the input state in case referenced value changed
                self.ui_system.states.entry(id)
                    .and_modify(|state| {
                        if let ElementVariant::Slider {
                            percent,
                            variant: SliderVariant::$var { value: v, min, max },
                        } = &mut state.variant {
                            if *v != *value {
                                *v = *value;
                                *percent = (*v - *min) as f32 / (*max - *min) as f32;
                                if *percent < 0.0 { *percent = 0.0; }
                                if *percent > 1.0 { *percent = 1.0; }
                            }
                        } else {
                            unreachable!();
                        }
                    })
                    .or_insert_with(|| State::$state_fn(*value, min, max));

                // Copy values that require self reference access
                let mouse_pos_x = self.get_mouse_position().0 as i32;
                let ui = &self.ui_system.uis.last().unwrap();
                let slider_box_padding = ui.style.slider_box_padding;
                let slider_cursor_width = ui.style.slider_cursor_width;

                let state = self.update_state_interaction(id, layout);
                if state.down {
                    let mouse_pos_x = mouse_pos_x - layout.pos.x - slider_box_padding;
                    let mouse_pos_x = mouse_pos_x as f32 - slider_cursor_width as f32 / 2.0;
                    let cursor_horizontal_space =
                        layout.size.x - 2 * slider_box_padding - slider_cursor_width as i32;

                    let mut new_percent = mouse_pos_x / cursor_horizontal_space as f32;
                    if new_percent < 0.0 { new_percent = 0.0; }
                    if new_percent > 1.0 { new_percent = 1.0; }

                    if let ElementVariant::Slider {
                        percent,
                        variant: SliderVariant::$var { value: v, min, max },
                    } = &mut state.variant {
                        *v = ((*max - *min) as f32 * new_percent + (*min as f32)).round() as $type;
                        *percent = (*v - *min) as f32 / (*max - *min) as f32;
                        *value = *v;
                    } else {
                        unreachable!();
                    }
                }

                self.add_element(id, layout);

                // Add number value
                self.same_line();
                // @TODO cache this string
                self.text(&format!("{}", value));
            }
        }
    }
}

slider_variant_integer_impl!(new_slider_i8,   slider_i8,   I8,   i8);
slider_variant_integer_impl!(new_slider_u8,   slider_u8,   U8,   u8);
slider_variant_integer_impl!(new_slider_i16,  slider_i16,  I16,  i16);
slider_variant_integer_impl!(new_slider_u16,  slider_u16,  U16,  u16);
slider_variant_integer_impl!(new_slider_i32,  slider_i32,  I32,  i32);
slider_variant_integer_impl!(new_slider_u32,  slider_u32,  U32,  u32);
slider_variant_integer_impl!(new_slider_i64,  slider_i64,  I64,  i64);
slider_variant_integer_impl!(new_slider_u64,  slider_u64,  U64,  u64);
//slider_variant_integer_impl!(new_slider_i128, slider_i128, I128, i128);
//slider_variant_integer_impl!(new_slider_u128, slider_u128, U128, u128);
