use crate::app::{
    App,
    ImDraw,
};
use super::*;

pub struct SliderState {
    pub pressed: bool,
    pub down: bool,
    pub hovering: bool,
    pub changed: bool,
}

pub struct SliderI32<'a> {
    label: &'a str,
    disabled: bool,
    min: i32,
    max: i32,
}

impl<'a> SliderI32<'a> {
    pub fn new(label: &str, min: i32, max: i32, value: &mut i32, app: &mut App) -> SliderState {
        SliderI32::builder(label, min, max).build(value, app)
    }

    pub fn builder<'b: 'a>(label: &'b str, min: i32, max: i32) -> Self {
        Self {
            label,
            min,
            max,
            disabled: false,
        }
    }

    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            disabled,
            ..self
        }
    }

    pub fn build(self, value: &mut i32, app: &mut App) -> SliderState {
        let state = app.slider_i32_internal(self.label, self.min, self.max, self.disabled, value);

        if let ElementVariant::Slider {
            changed,
            ..
        } = state.variant {
            SliderState {
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

// ------------

#[derive(Copy, Clone, Debug, ImDraw)]
pub(super) enum SliderVariant {
    I8   { value: i8,   min: i8,   max: i8 },
    I16  { value: i16,  min: i16,  max: i16 },
    I32  { value: i32,  min: i32,  max: i32 },
    I64  { value: i64,  min: i64,  max: i64 },
    //I128 { value: i128, min: i128, max: i128 },

    U8   { value: u8,   min: u8,   max: u8 },
    U16  { value: u16,  min: u16,  max: u16 },
    U32  { value: u32,  min: u32,  max: u32 },
    U64  { value: u64,  min: u64,  max: u64 },
    //U128 { value: u128, min: u128, max: u128 },
}

impl SliderVariant {
    pub(super) fn to_str(self) -> String {
        match self {
            SliderVariant::I8  { value, .. } => format!("{}", value),
            SliderVariant::I16 { value, .. } => format!("{}", value),
            SliderVariant::I32 { value, .. } => format!("{}", value),
            SliderVariant::I64 { value, .. } => format!("{}", value),

            SliderVariant::U8  { value, .. } => format!("{}", value),
            SliderVariant::U16 { value, .. } => format!("{}", value),
            SliderVariant::U32 { value, .. } => format!("{}", value),
            SliderVariant::U64 { value, .. } => format!("{}", value),
        }
    }
}

macro_rules! slider_variant_integer_impl {
    ($build_fn:ident, $internal_fn:ident, $var:ident, $type:ident) => {
        fn $build_fn(value: $type, min: $type, max: $type, disabled: bool) -> State {
            let mut percent = (value.saturating_sub(min)) as f32 / (max - min) as f32;
            if percent < 0.0 { percent = 0.0; }
            if percent > 1.0 { percent = 1.0; }

            State {
                pressed: false,
                down: false,
                hovering: false,
                disabled,
                variant: ElementVariant::Slider {
                    changed: false,
                    percent,
                    variant: SliderVariant::$var { value, min, max },
                },
            }
        }

        impl App<'_> {
            // @Refactor accept Input* instead of multiple params
            fn $internal_fn(
                &mut self,
                label: &str,
                min: $type,
                max: $type,
                disabled: bool,
                value: &mut $type
            ) -> &State {

                // Add label
                self.text_internal(Text::builder(label).disabled(disabled));
                self.same_line();

                let id = Id::new(label).add("#__slider");

                let ui = &self.ui_system.uis.last().unwrap();
                let size = Vec2i {
                    x: ui.draw_width() / 2,
                    y: ui.style.line_height as i32,
                };
                let layout = self.new_layout_right(size);
                self.add_element(id, layout);

                // @TODO we should update the input state in case referenced value changed
                self.ui_system.states.entry(id)
                    .and_modify(|state| {
                        if let ElementVariant::Slider {
                            changed,
                            percent,
                            variant: SliderVariant::$var { value: v, min, max },
                        } = &mut state.variant {
                            if *v != *value {
                                *v = *value;
                                *percent = ((*v).saturating_sub(*min)) as f32 / (*max - *min) as f32;
                                *percent = percent.clamp(0.0, 1.0);
                                *changed = true;
                            } else {
                                *changed = false;
                            }
                        } else {
                            unreachable!();
                        }
                    })
                    .or_insert_with(|| $build_fn(*value, min, max, disabled));

                if !disabled {
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
                            changed,
                            percent,
                            variant: SliderVariant::$var { value: v, min, max },
                        } = &mut state.variant {
                            *v = ((*max - *min) as f32 * new_percent + (*min as f32)).round() as $type;
                            *percent = (*v - *min) as f32 / (*max - *min) as f32;
                            *value = *v;
                            *changed = true;
                        } else {
                            unreachable!();
                        }
                    }

                    state
                } else {
                    self.ui_system.states.get(&id).unwrap()
                }

                /*
                // Add number value
                self.same_line();
                // @TODO cache this string
                self.text(&format!("{}", value));
                */
            }
        }
    }
}

slider_variant_integer_impl!(new_slider_i8,   slider_i8_internal,   I8,   i8);
slider_variant_integer_impl!(new_slider_i16,  slider_i16_internal,  I16,  i16);
slider_variant_integer_impl!(new_slider_i32,  slider_i32_internal,  I32,  i32);
slider_variant_integer_impl!(new_slider_i64,  slider_i64_internal,  I64,  i64);
//slider_variant_integer_impl!(new_slider_i128, slider_i128_internal, I128, i128);

slider_variant_integer_impl!(new_slider_u8,   slider_u8_internal,   U8,   u8);
slider_variant_integer_impl!(new_slider_u16,  slider_u16_internal,  U16,  u16);
slider_variant_integer_impl!(new_slider_u32,  slider_u32_internal,  U32,  u32);
slider_variant_integer_impl!(new_slider_u64,  slider_u64_internal,  U64,  u64);
//slider_variant_integer_impl!(new_slider_u128, slider_u128_internal, U128, u128);
