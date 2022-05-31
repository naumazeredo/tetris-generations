use super::*;
use crate::app::{
    App,
    Transform,
    renderer::{
        batch::Batch,
        color::WHITE,
        sprite::Sprite,
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
        let mut batch = Batch::new();

        // Draw ui window background
        batch.queue_draw_solid(
            Transform {
                pos: self.layout.pos.into(),
                pivot: Vec2::new(),
                scale: Vec2 { x: 1.0, y: 1.0 },
                rot: 0.0,
                layer: 900,
            },
            self.layout.size.into(),
            self.style.background_color,
        );

        // Draw line
        if let Some(line) = self.focused_line {
            let layout = self.lines[line as usize].layout;
            batch.queue_draw_solid(
                Transform {
                    pos: layout.pos.into(),
                    pivot: Vec2::new(),
                    scale: Vec2 { x: 1.0, y: 1.0 },
                    rot: 0.0,
                    layer: 905,
                },
                layout.size.into(),
                self.style.line_focus_background_color,
            );
        }

        // Draw elements

        // @Cleanup not clipping to avoid multiple draw calls
        //let padding = self.style.padding;
        //batch.push_clip(self.layout.pos + padding, self.layout.size - 2 * padding);

        let elements = std::mem::take(&mut self.elements);
        for element in elements {
            self.queue_draw_element(element, &mut batch, app);
        }

        //batch.pop_clip();

        // Draw modal separately (without clipping)
        let modal_elements = std::mem::take(&mut self.modal_elements);
        for element in modal_elements {
            self.queue_draw_element(element, &mut batch, app);
        }

        app.render_batch(batch, None);
    }

    fn queue_draw_element(&mut self, element: Element, batch: &mut Batch, app: &mut App) {
        let state = app.ui_system.states.get(&element.id).unwrap();
        let layout = element.layout;

        match &state.variant {
            ElementVariant::Text { text, multiline } => {
                // @Refactor allow custom alignments
                let pos = layout.pos + if *multiline {
                    Vec2i::new()
                } else {
                    Vec2i { x: 0, y: (layout.size.y - self.style.text_size as i32) / 2 }
                };

                let text_color;
                if state.disabled {
                    text_color = self.style.text_disabled_color;
                } else {
                    text_color = self.style.text_color;
                }

                let max_width = if *multiline { Some(layout.size.x as u32) } else { None };

                //batch.push_clip(layout.pos, layout.size);

                batch.queue_draw_text(
                    text,
                    Transform {
                        pos: pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    self.style.text_size as f32,
                    text_color,
                    None,
                    max_width,
                    &app,
                );

                //batch.pop_clip();
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

                batch.queue_draw_solid(
                    Transform {
                        pos: layout.pos.into(),
                        pivot: Vec2::new(),
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
                let pos = layout.pos + padding;

                let text_color;
                if state.disabled {
                    text_color = self.style.text_disabled_color;
                } else {
                    text_color = self.style.text_color;
                }

                //batch.push_clip(layout.pos + padding, layout.size - 2 * padding);

                batch.queue_draw_text(
                    text,
                    Transform {
                        pos: pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    self.style.text_size as f32,
                    text_color,
                    None,
                    None,
                    &app,
                );

                //batch.pop_clip();
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

                batch.queue_draw_solid(
                    Transform {
                        pos: layout.pos.into(),
                        pivot: Vec2::new(),
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

                batch.queue_draw_solid(
                    Transform {
                        pos: layout.pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    layout.size.into(),
                    color,
                );

                // Draw input text
                let padding = Vec2i { x: self.style.box_padding, y: self.style.box_padding };
                let pos = layout.pos + padding;

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

                //batch.push_clip(layout.pos + padding, layout.size - 2 * padding);

                batch.queue_draw_text(
                    text,
                    Transform {
                        pos: pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    self.style.text_size as f32,
                    text_color,
                    None,
                    None,
                    &app,
                );

                // Draw cursor
                if !state.disabled {
                    let cursor_duration = self.style.input_cursor_duration;
                    let cursor_timestamp = app.ui_system.input_cursor_timestamp;
                    let current_timestamp = app.real_timestamp();

                    if *is_input_focus &&
                        ((current_timestamp - cursor_timestamp) / cursor_duration) % 2 == 0 {

                            let text_draw_size: Vec2i = calculate_draw_text_size_with_callback(
                                &app.font_system,
                                text,
                                self.style.text_size as f32,
                                None,
                                None,
                                |_,_,_| {}
                            ).into();

                            let pos = layout.pos + padding;
                            let pos = pos + Vec2i {
                                x: text_draw_size.x + self.style.input_cursor_padding,
                                y: -self.style.input_cursor_padding,
                            };

                            let size = Vec2i {
                                x: self.style.input_cursor_size as i32,
                                y: 2 * self.style.input_cursor_padding,
                            };

                            batch.queue_draw_solid(
                                Transform {
                                    pos: pos.into(),
                                    pivot: Vec2::new(),
                                    scale: Vec2 { x: 1.0, y: 1.0 },
                                    rot: 0.0,
                                    layer: 910,
                                },
                                size.into(),
                                self.style.text_color,
                            );
                    }
                }

                //batch.pop_clip();
            }

            ElementVariant::Slider { percent, variant, .. } => {
                let box_color = if state.disabled {
                    self.style.box_disabled_color
                } else {
                    self.style.box_color
                };

                batch.queue_draw_solid(
                    Transform {
                        pos: layout.pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    layout.size.into(),
                    box_color,
                );

                // Draw cursor
                let color = if state.disabled {
                    self.style.slider_cursor_disabled_color
                } else if state.down {
                    self.style.slider_cursor_focused_color
                } else if state.hovering {
                    self.style.slider_cursor_hover_color
                } else {
                    self.style.slider_cursor_unfocused_color
                };

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

                batch.queue_draw_solid(
                    Transform {
                        pos: pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    size.into(),
                    color,
                );

                // Value
                let text_color = if state.disabled {
                    self.style.text_disabled_color
                } else {
                    self.style.text_color
                };

                let text = &variant.to_str();

                let text_draw_size: Vec2i = calculate_draw_text_size_with_callback(
                    &app.font_system,
                    text,
                    self.style.text_size as f32,
                    None,
                    None,
                    |_,_,_| {}
                ).into();

                let pos = layout.pos +
                    Vec2i {
                        x: (layout.size.x - text_draw_size.x) / 2,
                        y: self.style.box_padding
                    };

                batch.queue_draw_text(
                    text,
                    Transform {
                        pos: pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    self.style.text_size as f32,
                    text_color,
                    None,
                    None,
                    &app,
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

                batch.queue_draw_solid(
                    Transform {
                        pos: layout.pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    layout.size.into(),
                    color,
                );

                // @Cleanup not clipping to avoid multiple draw calls
                // Fix text position since it's rendered from the bottom
                //let padding = Vec2i { x: self.style.box_padding, y: self.style.box_padding };
                //let pos = layout.pos + padding + Vec2i { x: 0, y: self.style.text_size as i32 };

                let text_color;
                if state.disabled {
                    text_color = self.style.text_disabled_color;
                } else {
                    text_color = self.style.text_color;
                }

                //batch.push_clip(layout.pos + padding, layout.size - 2 * padding);

                // Draw text
                let text_draw_size: Vec2i = calculate_draw_text_size_with_callback(
                    &app.font_system,
                    text,
                    self.style.text_size as f32,
                    None,
                    None,
                    |_,_,_| {}
                ).into();

                let pos = layout.pos +
                    Vec2i {
                        x: (layout.size.x - text_draw_size.x) / 2,
                        y: self.style.box_padding
                    };

                batch.queue_draw_text(
                    text,
                    Transform {
                        pos: pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    self.style.text_size as f32,
                    text_color,
                    None,
                    None,
                    &app,
                );

                //batch.pop_clip();
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

                batch.queue_draw_solid(
                    Transform {
                        pos: layout.pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 920,
                    },
                    layout.size.into(),
                    color,
                );

                // @Cleanup not clipping to avoid multiple draw calls
                // Fix text position since it's rendered from the bottom
                //let padding = Vec2i { x: self.style.box_padding, y: self.style.box_padding };
                //let pos = layout.pos + padding + Vec2i { x: 0, y: self.style.text_size as i32 };

                let text_color;
                if state.disabled {
                    text_color = self.style.text_disabled_color;
                } else {
                    text_color = self.style.text_color;
                }

                //batch.push_clip(layout.pos + padding, layout.size - 2 * padding);

                // Draw text
                let text_draw_size: Vec2i = calculate_draw_text_size_with_callback(
                    &app.font_system,
                    text,
                    self.style.text_size as f32,
                    None,
                    None,
                    |_,_,_| {}
                ).into();

                let pos = layout.pos +
                    Vec2i {
                        x: (layout.size.x - text_draw_size.x) / 2,
                        y: self.style.box_padding
                    };

                batch.queue_draw_text(
                    text,
                    Transform {
                        pos: pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 920,
                    },
                    self.style.text_size as f32,
                    text_color,
                    None,
                    None,
                    &app,
                );

                //batch.pop_clip();
            }

            ElementVariant::Scrollbar => {
                batch.queue_draw_solid(
                    Transform {
                        pos: layout.pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 920,
                    },
                    layout.size.into(),
                    self.style.scrollbar_color,
                );
            }

            ElementVariant::PagedBox { lines_per_page, current_page, num_lines } => {
                // Top border

                let pos = layout.pos;

                let size = Vec2i {
                    x: layout.size.x,
                    y: self.style.paged_box_border as i32,
                };

                batch.queue_draw_solid(
                    Transform {
                        pos: pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 901,
                    },
                    size.into(),
                    self.style.paged_box_index_background,
                );

                let pos = pos + Vec2i { x: 0, y: self.style.paged_box_border as i32 };

                let size = Vec2i {
                    x: layout.size.x,
                    y: layout.size.y - self.style.line_height,
                };

                batch.queue_draw_solid(
                    Transform {
                        pos: pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 901,
                    },
                    size.into(),
                    self.style.paged_box_background,
                );

                // Page index
                let pos = pos +
                    Vec2i { x: 0, y: self.style.line_height * (*lines_per_page) as i32 };

                let size = Vec2i {
                    x: layout.size.x,
                    y: self.style.line_height,
                };

                batch.queue_draw_solid(
                    Transform {
                        pos: pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 901,
                    },
                    size.into(),
                    self.style.paged_box_index_background,
                );

                let left_text = "< ";
                let right_text = &format!(" {}/{} >",
                    current_page + 1,
                    (num_lines.saturating_sub(1) / lines_per_page) + 1
                );

                let left_text_draw_size: Vec2i = calculate_draw_text_size_with_callback(
                    &app.font_system,
                    left_text,
                    self.style.text_size as f32,
                    None,
                    None,
                    |_,_,_| {}
                ).into();

                let right_text_draw_size: Vec2i = calculate_draw_text_size_with_callback(
                    &app.font_system,
                    right_text,
                    self.style.text_size as f32,
                    None,
                    None,
                    |_,_,_| {}
                ).into();

                let index_draw_size = Vec2i {
                    x: left_text_draw_size.x + right_text_draw_size.x + 16,
                    y: self.style.text_size as i32,
                };

                let pos = pos +
                    Vec2i {
                        x: (layout.size.x - index_draw_size.x) / 2,
                        y: (size.y - index_draw_size.y) / 2,
                    };

                batch.queue_draw_text(
                    left_text,
                    Transform {
                        pos: pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    self.style.text_size as f32,
                    self.style.text_color,
                    None,
                    None,
                    &app
                );

                // @TODO this should be using the Asset system
                //let mouse_texture = app.get_texture_or_load("assets/gfx/inputs/tile_0082.png");
                let mouse_texture = app.get_texture_or_load("assets/gfx/inputs/tile_0116.png");
                //let mouse_texture = app.get_texture_or_load("assets/gfx/inputs/tile_0277.png");
                let mouse_sprite = Sprite::new(mouse_texture, 0, 0, 16, 16);

                let mouse_pos = pos + Vec2i { x: left_text_draw_size.x, y: 0 };
                batch.queue_draw_sprite(
                    Transform {
                        pos: mouse_pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    mouse_sprite,
                    WHITE
                );

                let pos = pos + Vec2i { x: left_text_draw_size.x + 16, y: 0 };
                batch.queue_draw_text(
                    right_text,
                    Transform {
                        pos: pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 910,
                    },
                    self.style.text_size as f32,
                    self.style.text_color,
                    None,
                    None,
                    &app,
                );
            }

            ElementVariant::Texture { texture } => {
                let pos = layout.pos +
                    Vec2i {
                        x: (layout.size.x - texture.borrow().w as i32) / 2,
                        y: 0,
                    };

                let size = Vec2i {
                    x: texture.borrow().w as i32,
                    y: texture.borrow().h as i32,
                };

                batch.queue_draw_texture(
                    Transform {
                        pos: pos.into(),
                        pivot: Vec2::new(),
                        scale: Vec2 { x: 1.0, y: 1.0 },
                        rot: 0.0,
                        layer: 920,
                    },
                    texture.clone().into(),
                    size.into(),
                    WHITE,
                );
            }

            ElementVariant::Separator => {
                todo!();
            }
        }
    }
}
