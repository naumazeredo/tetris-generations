use paste::paste;
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
    ($type:ident) => {
        paste! {
            pub struct [<Slider $type:upper>]<'a> {
                label: &'a str,
                disabled: bool,
                min: $type,
                max: $type,
            }

            impl<'a> [<Slider $type:upper>]<'a> {
                pub fn new<'b: 'a>(label: &'b str, min: $type, max: $type, value: &mut $type, app: &mut App) -> SliderState {
                    Self::builder(label, min, max).build(value, app)
                }

                pub fn builder<'b: 'a>(label: &'b str, min: $type, max: $type) -> Self {
                    Self { label, min, max, disabled: false, }
                }

                pub fn disabled(self, disabled: bool) -> Self {
                    Self { disabled, ..self }
                }

                #[inline(always)] pub fn build(
                    self,
                    value: &mut $type,
                    app: &mut App
                ) -> SliderState {
                    self.build_with_placer(value, &mut app.ui_system.top_ui().index(), app)
                }

                #[inline(always)] pub fn build_with_placer<P: Placer>(
                    self,
                    value: &mut $type,
                    placer: &mut P,
                    app: &mut App
                ) -> SliderState {
                    if let Some(state) = [<slider_ $type _internal>](value, self, placer, app) {
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
                    } else {
                        SliderState {
                            pressed:  false,
                            down:     false,
                            hovering: false,
                            changed:  false,
                        }
                    }
                }
            }

            fn [<new_slider_ $type>](value: $type, min: $type, max: $type, disabled: bool) -> State {
                let mut percent = (value.saturating_sub(min)) as f32 / (max - min) as f32;
                if percent < 0.0 { percent = 0.0; }
                if percent > 1.0 { percent = 1.0; }

                State {
                    disabled,
                    pressed:  false,
                    down:     false,
                    hovering: false,
                    scroll:   0,
                    variant: ElementVariant::Slider {
                        changed: false,
                        percent,
                        variant: SliderVariant::[<$type:upper>]{ value, min, max },
                    },
                }
            }

            fn [<slider_ $type _internal>]<'a, P: Placer>(
                value: &mut $type,
                slider: [<Slider $type:upper>],
                placer: &mut P,
                app: &'a mut App,
            ) -> Option<&'a State> {
                let ui = placer.ui(app);
                let spacing = ui.style.spacing;
                let line_padding = ui.style.line_padding;

                placer.add_padding(line_padding, app);
                let col_width = (placer.draw_width(app) - spacing) / 2;

                // Add label
                text_internal(
                    Text::builder(slider.label)
                        .disabled(slider.disabled)
                        .max_width(col_width as u32),
                    placer,
                    app
                );

                placer.same_line(app);
                placer.add_spacing(app);

                let id = Id::new(slider.label).add("#__slider");

                let ui = placer.ui(app);
                let size = Vec2i {
                    x: col_width,
                    y: ui.line_draw_height() as i32,
                };
                let layout = placer.place_element(id, size, app);

                placer.remove_padding(app);

                if layout.is_none() { return None; }
                let layout = layout.unwrap();

                // @TODO we should update the input state in case referenced value changed
                app.ui_system.states.entry(id)
                    .and_modify(|state| {
                        if let ElementVariant::Slider {
                            changed,
                            percent,
                            variant: SliderVariant::[<$type:upper>]{ value: v, min, max },
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
                    .or_insert_with(|| [<new_slider_ $type>](*value, slider.min, slider.max, slider.disabled));

                if !slider.disabled {
                    // Copy values since borrow-checker doesn't allow multiple references
                    let mouse_pos_x = app.get_mouse_position().0 as i32;
                    let ui = placer.ui(app);
                    let slider_box_padding = ui.style.slider_box_padding;
                    let slider_cursor_width = ui.style.slider_cursor_width;

                    let state = app.update_state_interaction(id, layout);
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
                            variant: SliderVariant::[<$type:upper>] { value: v, min, max },
                        } = &mut state.variant {
                            *v = ((*max - *min) as f32 * new_percent + (*min as f32)).round() as $type;
                            *percent = (*v - *min) as f32 / (*max - *min) as f32;
                            *value = *v;
                            *changed = true;
                        } else {
                            unreachable!();
                        }
                    }

                    Some(state)
                } else {
                    Some(app.ui_system.states.get(&id).unwrap())
                }
            }
        }
    }
}

slider_variant_integer_impl!(i8);
slider_variant_integer_impl!(i16);
slider_variant_integer_impl!(i32);
slider_variant_integer_impl!(i64);
//slider_variant_integer_impl!(i128);

slider_variant_integer_impl!(u8);
slider_variant_integer_impl!(u16);
slider_variant_integer_impl!(u32);
slider_variant_integer_impl!(u64);
//slider_variant_integer_impl!(u128);
