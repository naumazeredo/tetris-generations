use super::*;
use crate::app::{
    App,
    font_system::{
        FONT_SCALE,
        FontId,
        FontSystem,
    },
    transform::Transform,
};
use crate::linalg::Vec2;

// @Refactor avoid reusing the almost same code inside calculate and queue functions

// @TODO split on words
// @TODO return offset. Some glyphs can have negative minx or miny
pub(in crate::app) fn calculate_draw_text_size(
    font_system: &FontSystem,
    text: &str,
    font_size: f32,
    font_id: Option<FontId>,
    max_width: Option<u32>,
    //line_spacing: i32,
) -> Vec2 {
    let scale = font_size / FONT_SCALE as f32;

    let font_id = font_id.unwrap_or(font_system.default_font_id);
    let font = font_system.fonts.get(&font_id).unwrap();

    let mut pos = Vec2 { x: 0.0, y: font_size };
    let mut max_size = Vec2 {
        x: max_width.unwrap_or(0) as f32,
        y: font_size
    };
    let mut start_of_line = false;

    for ch in text.chars() {
        if let Some(char_data) = font.get_char_data(ch) {
            let advance = char_data.metrics.advance as f32 * scale;

            if ch.is_whitespace() {
                if start_of_line { continue; }
                max_size.x = max_size.x.max(pos.x + advance);
            } else {
                let char_top_left = Vec2 {
                    x:  char_data.metrics.minx as f32 * scale,
                    y: -char_data.metrics.maxy as f32 * scale
                };
                let size = Vec2 {
                    x: char_data.metrics.w as f32 * scale,
                    y: char_data.metrics.h as f32 * scale
                };

                // Check if there's space to place the current character
                if let Some(max_width) = max_width {
                    if pos.x + char_top_left.x + size.x > max_width as f32 {
                        pos.x = 0.0;
                        pos.y += font_size;
                    }
                }

                max_size.x = max_size.x.max(pos.x + char_top_left.x + size.x);
            }

            max_size.y = max_size.y.max(pos.y + font_size);

            start_of_line = false;
            pos.x += advance;
            if let Some(max_width) = max_width {
                if pos.x + advance > max_width as f32 {
                    pos.x = 0.0;
                    pos.y += font_size;
                    start_of_line = true;
                }
            }
        }
    }

    max_size
}


impl Batch {
    pub fn queue_draw_text(
        &mut self,
        //program: ShaderProgram,
        text: &str,
        transform: &Transform,
        font_size: f32,
        color: Color,

        font_id: Option<FontId>,
        max_width: Option<u32>,

        app: &App,
    ) {
        self.queue_draw_text_internal(
            &app.font_system,

            //program,
            text,
            transform,
            font_size,
            color,
            font_id,
            max_width,
        );
    }

    pub(in crate::app) fn queue_draw_text_internal(
        &mut self,
        font_system: &FontSystem,

        //program: ShaderProgram,
        text: &str,
        transform: &Transform,
        font_size: f32,
        color: Color,

        font_id: Option<FontId>,
        max_width: Option<u32>,
    ) {
        let scale = font_size / FONT_SCALE as f32;

        // @Refactor hold the font reference instead of getting it every char
        let font_id = font_id.unwrap_or(font_system.default_font_id);
        let font_texture = font_system.fonts.get(&font_id).unwrap().texture;

        let mut start_of_line = false;
        let mut pos = Vec2 { x: 0.0, y: font_size };
        for ch in text.chars() {
            if let Some(&char_data) = font_system.fonts.get(&font_id).unwrap().get_char_data(ch) {
                let advance = char_data.metrics.advance as f32 * scale;

                if ch.is_whitespace() {
                    if start_of_line { continue; }
                } else {
                    let uvs = char_data.get_uvs();

                    let char_top_left = Vec2 {
                        x:  char_data.metrics.minx as f32 * scale,
                        y: -char_data.metrics.maxy as f32 * scale
                    };
                    let size = Vec2 {
                        x: char_data.metrics.w as f32 * scale,
                        y: char_data.metrics.h as f32 * scale
                    };

                    if let Some(max_width) = max_width {
                        if pos.x + char_top_left.x + size.x > max_width as f32 {
                            pos.x = 0.0;
                            pos.y += font_size;
                        }
                    }

                    self.queue_draw_sprite(
                        transform,
                        &Sprite {
                            subtexture: Subtexture {
                                texture: font_texture,
                                uvs,
                            },
                            texture_flip: TextureFlip::NO,
                            pivot: - (pos + char_top_left),
                            size,
                        },
                        color,
                    );
                }

                start_of_line = false;
                pos.x += advance;
                if let Some(max_width) = max_width {
                    if pos.x + advance > max_width as f32 {
                        pos.x = 0.0;
                        pos.y += font_size;
                        start_of_line = true;
                    }
                }
            }
        }
    }
}

impl App<'_> {
    pub fn queue_draw_text(
        &mut self,
        //program: ShaderProgram,
        text: &str,
        transform: &Transform,
        font_size: f32,
        color: Color,
        font_id: Option<FontId>,
        max_width: Option<u32>,
    ) {
        self.renderer.batch.queue_draw_text_internal(
            &self.font_system,
            //program,
            text,
            transform,
            font_size,
            color,
            font_id,
            max_width,
        );
    }

    // @Maybe change options to builders?

    pub fn calculate_draw_text_size(
        &self,
        text: &str,
        font_size: f32,
        font_id: Option<FontId>,
        max_width: Option<u32>,
    ) -> Vec2 {
        calculate_draw_text_size(
            &self.font_system,
            text,
            font_size,
            font_id,
            max_width,
        )
    }
}
