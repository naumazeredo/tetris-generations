use super::*;
use crate::app::{
    App,
    Transform,
    renderer::text::queue_draw_text,
    renderer::draw_command::{
        queue_draw_solid,
        push_clip,
        pop_clip,
    },
};
use crate::linalg::{Vec2, Vec2i};

impl App<'_> {
    pub fn queue_draw_uis(&mut self) {
        let uis = std::mem::take(&mut self.ui_system.uis);

        for mut ui in uis.into_iter() {
            ui.queue_draw(self);
        }

        // new frame update
        // @XXX Dear ImGUI is starting the text input for us. Is this a problem?
        /*
        let text_input = self.sdl_context.video_subsystem.text_input();

        if self.ui_system.found_input_focus && !text_input.is_active() {
            text_input.start();
            println!("text input start");
        }

        if !self.ui_system.found_input_focus && text_input.is_active() {
            self.ui_system.input_focus = None;
            text_input.stop();
            println!("text input stop");
        }
        */

        if !self.ui_system.found_input_focus && self.ui_system.input_focus.is_some() {
            self.ui_system.input_focus = None;
        }

        self.ui_system.found_input_focus = false;
        self.ui_system.input_complete = false;
    }
}

impl Ui {
    pub fn queue_draw(&mut self, app: &mut App) {
        // Draw ui window background
        queue_draw_solid(
            &mut app.renderer,
            &Transform {
                pos: self.layout.pos.into(),
                scale: Vec2 { x: 1.0, y: 1.0 },
                rot: 0.0,
                layer: 900,
            },
            self.layout.size.into(),
            self.style.background_color,
        );

        // Draw elements
        let padding = Vec2i { x: self.style.padding, y: self.style.padding };
        push_clip(&mut app.renderer, self.layout.pos + padding, self.layout.size - 2 * padding);

        let elements = std::mem::take(&mut self.elements);

        // @XXX how to consume the iterator???
        for element in elements.into_iter() {
            self.queue_draw_element(element, app);
        }

        pop_clip(&mut app.renderer);

        // Draw modal separately (without clipping)
        let modal_elements = std::mem::take(&mut self.modal_elements);
        for element in modal_elements.into_iter() {
            self.queue_draw_element(element, app);
        }
    }

    fn queue_draw_element(&mut self, element: Element, app: &mut App) {
        let state = app.ui_system.states.get(&element.id).unwrap();
        let layout = element.layout;

        match &state.variant {
            ElementVariant::Text { text } => {
                // 
                let pos = layout.pos + Vec2i { x: 0, y: self.style.font_size as i32 };

                let text_color;
                if state.disabled {
                    text_color = self.style.text_disabled_color;
                } else {
                    text_color = self.style.text_color;
                }

                queue_draw_text(
                    &mut app.renderer,
                    &app.font_system,

                    text,
                    app.font_system.default_font_id,
                    &Transform {
                        pos: pos.into(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    self.style.font_size as f32,
                    text_color,
                );
            }

            ElementVariant::Button { text } => {
                // Draw button background
                let box_color;
                if state.disabled {
                    box_color = self.style.box_disabled_color;
                } else if state.down {
                    box_color = self.style.box_down_color;
                } else if state.hovering {
                    box_color = self.style.box_hover_color;
                } else {
                    box_color = self.style.box_color;
                }

                queue_draw_solid(
                    &mut app.renderer,
                    &Transform {
                        pos: layout.pos.into(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    layout.size.into(),
                    box_color,
                );

                // Draw text
                // Fix text position since it's rendered from the bottom
                let padding = Vec2i { x: self.style.box_padding, y: self.style.box_padding };
                let pos = layout.pos + Vec2i { x: 0, y: self.style.font_size as i32 } + padding;

                let text_color;
                if state.disabled {
                    text_color = self.style.text_disabled_color;
                } else {
                    text_color = self.style.text_color;
                }

                push_clip(&mut app.renderer, layout.pos + padding, layout.size - 2 * padding);

                queue_draw_text(
                    &mut app.renderer,
                    &app.font_system,

                    text,
                    app.font_system.default_font_id,
                    &Transform {
                        pos: pos.into(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    self.style.font_size as f32,
                    text_color,
                );

                pop_clip(&mut app.renderer);
            }

            ElementVariant::Checkbox { value } => {
                let color;
                if state.disabled {
                    if *value {
                        color = self.style.checkbox_selected_disabled_color;
                    } else {
                        color = self.style.checkbox_unselected_disabled_color;
                    }
                } else {
                    if *value {
                        if state.hovering {
                            color = self.style.checkbox_selected_hover_color;
                        } else {
                            color = self.style.checkbox_selected_color;
                        }
                    } else {
                        if state.hovering {
                            color = self.style.checkbox_unselected_hover_color;
                        } else {
                            color = self.style.checkbox_unselected_color;
                        }
                    }
                }

                queue_draw_solid(
                    &mut app.renderer,
                    &Transform {
                        pos: layout.pos.into(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    layout.size.into(),
                    color,
                );
            }

            ElementVariant::Input { value_str, is_input_focus, .. } => {
                // Calculate input box color
                let color;
                if state.disabled {
                    color = self.style.box_disabled_color;
                } else if state.down {
                    color = self.style.box_down_color;
                } else if state.hovering {
                    color = self.style.box_hover_color;
                } else if *is_input_focus {
                    color = self.style.input_focused_color;
                } else {
                    color = self.style.box_color;
                }

                queue_draw_solid(
                    &mut app.renderer,
                    &Transform {
                        pos: layout.pos.into(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    layout.size.into(),
                    color,
                );

                // Draw input text
                let padding = Vec2i { x: self.style.box_padding, y: self.style.box_padding };
                let pos = layout.pos + padding + Vec2i { x: 0, y: self.style.font_size as i32 };

                let text;
                let text_color;
                if state.disabled {
                    text_color = self.style.text_disabled_color;
                    text = value_str;
                } else {
                    text_color = self.style.text_color;
                    if *is_input_focus {
                        text = &app.ui_system.input_state;
                    } else {
                        text = value_str;
                    }
                }

                push_clip(&mut app.renderer, layout.pos + padding, layout.size - 2 * padding);

                queue_draw_text(
                    &mut app.renderer,
                    &app.font_system,

                    text,
                    app.font_system.default_font_id,
                    &Transform {
                        pos: pos.into(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    self.style.font_size as f32,
                    text_color,
                );

                // Draw cursor
                if !state.disabled {
                    let cursor_duration = self.style.input_cursor_duration;
                    let cursor_timestamp = app.ui_system.input_cursor_timestamp;
                    let current_timestamp = app.real_timestamp();

                    if *is_input_focus &&
                        ((current_timestamp - cursor_timestamp) / cursor_duration) % 2 == 0 {

                            let text_draw_size: Vec2i = calculate_draw_text_size(
                                &app.font_system,
                                text,
                                app.font_system.default_font_id,
                                self.style.font_size as f32,
                            ).into();

                            let pos = layout.pos + padding;
                            let pos = pos + Vec2i {
                                x: text_draw_size.x + self.style.input_cursor_padding,
                                y: -self.style.input_cursor_padding,
                            };

                            let size = Vec2i {
                                x: self.style.input_cursor_size as i32,
                                y: self.style.font_size as i32 + 2 * self.style.input_cursor_padding,
                            };

                            queue_draw_solid(
                                &mut app.renderer,
                                &Transform {
                                    pos: pos.into(),
                                    scale: Vec2 { x: 1.0, y: 1.0 },
                                    rot: 0.0,
                                    layer: 910,
                                },
                                size.into(),
                                self.style.text_color,
                            );
                    }
                }

                pop_clip(&mut app.renderer);
            }

            ElementVariant::Slider { percent, variant } => {
                queue_draw_solid(
                    &mut app.renderer,
                    &Transform {
                        pos: layout.pos.into(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    layout.size.into(),
                    self.style.slider_box_color,
                );

                // Draw cursor
                let color;
                if state.hovering {
                    color = self.style.slider_cursor_hover_color;
                } else {
                    color = self.style.slider_cursor_unfocused_color;
                }

                // @Refactor this into a function since it's used for both rendering and state
                //           update
                let cursor_horizontal_space = layout.size.x -
                    2 * self.style.slider_box_padding - self.style.slider_cursor_width as i32;
                let pos = layout.pos + Vec2i {
                    x: self.style.slider_box_padding +
                        (cursor_horizontal_space as f32 * (*percent)).round() as i32,
                        y: self.style.slider_box_padding,
                };

                let size = Vec2i {
                    x: self.style.slider_cursor_width as i32,
                    y: layout.size.y - 2 * self.style.slider_box_padding,
                };

                queue_draw_solid(
                    &mut app.renderer,
                    &Transform {
                        pos: pos.into(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    size.into(),
                    color,
                );

                // Value
                let text = &variant.to_str();

                let text_draw_size: Vec2i = calculate_draw_text_size(
                    &app.font_system,
                    text,
                    app.font_system.default_font_id,
                    self.style.font_size as f32,
                ).into();

                let pos = layout.pos +
                    Vec2i {
                        x: (layout.size.x - text_draw_size.x) / 2,
                        y: self.style.box_padding + self.style.font_size as i32
                    };

                queue_draw_text(
                    &mut app.renderer,
                    &app.font_system,

                    text,
                    app.font_system.default_font_id,
                    &Transform {
                        pos: pos.into(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    self.style.font_size as f32,
                    self.style.text_color,
                );
            }

            ElementVariant::Combobox { text, .. } => {
                // Draw button background
                let color;
                if state.disabled {
                    color = self.style.box_disabled_color;
                } else if state.down {
                    color = self.style.box_down_color;
                } else if state.hovering {
                    color = self.style.box_hover_color;
                } else {
                    color = self.style.box_color;
                }

                queue_draw_solid(
                    &mut app.renderer,
                    &Transform {
                        pos: layout.pos.into(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    layout.size.into(),
                    color,
                );

                // Fix text position since it's rendered from the bottom
                let padding = Vec2i { x: self.style.box_padding, y: self.style.box_padding };
                let pos = layout.pos + padding +
                    Vec2i { x: 0, y: self.style.font_size as i32 };

                let text_color;
                if state.disabled {
                    text_color = self.style.text_disabled_color;
                } else {
                    text_color = self.style.text_color;
                }

                push_clip(&mut app.renderer, layout.pos + padding, layout.size - 2 * padding);

                // Draw text
                queue_draw_text(
                    &mut app.renderer,
                    &app.font_system,

                    text,
                    app.font_system.default_font_id,
                    &Transform {
                        pos: pos.into(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    self.style.font_size as f32,
                    text_color,
                );

                pop_clip(&mut app.renderer);
            }

            ElementVariant::ComboboxOption { selected, text } => {
                // Draw button background
                let color;
                if state.disabled {
                    color = self.style.box_disabled_color;
                } else if state.down {
                    color = self.style.box_down_color;
                } else if state.hovering {
                    color = self.style.box_hover_color;
                } else if *selected {
                    color = self.style.combobox_selected_option_color;
                } else {
                    color = self.style.combobox_option_background_color;
                }

                queue_draw_solid(
                    &mut app.renderer,
                    &Transform {
                        pos: layout.pos.into(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 920,
                    },
                    layout.size.into(),
                    color,
                );

                // Fix text position since it's rendered from the bottom
                let padding = Vec2i { x: self.style.box_padding, y: self.style.box_padding };
                let pos = layout.pos + padding +
                    Vec2i { x: 0, y: self.style.font_size as i32 };

                let text_color;
                if state.disabled {
                    text_color = self.style.text_disabled_color;
                } else {
                    text_color = self.style.text_color;
                }

                push_clip(&mut app.renderer, layout.pos + padding, layout.size - 2 * padding);

                // Draw text
                queue_draw_text(
                    &mut app.renderer,
                    &app.font_system,

                    text,
                    app.font_system.default_font_id,
                    &Transform {
                        pos: pos.into(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 920,
                    },
                    self.style.font_size as f32,
                    text_color,
                );

                pop_clip(&mut app.renderer);
            }

            ElementVariant::Separator => {
                todo!();
            }

            ElementVariant::Scrollbar => {
                queue_draw_solid(
                    &mut app.renderer,
                    &Transform {
                        pos: layout.pos.into(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 920,
                    },
                    layout.size.into(),
                    self.style.scrollbar_color,
                );
            }
        }
    }
}
