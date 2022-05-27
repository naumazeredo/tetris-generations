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

const TEXT_LINE_SPACING_FACTOR : f32 = 1.5;

// @Refactor avoid reusing the almost same code inside calculate and queue functions

// @TODO split on words
// @TODO return offset. Some glyphs can have negative minx or miny
pub(in crate::app) fn calculate_draw_text_size_with_callback<F>(
    font_system: &FontSystem,
    text: &str,
    font_size: f32,
    font_id: Option<FontId>,
    max_width: Option<u32>,
    mut callback: F,
) -> Vec2
where
    F: FnMut(Subtexture, Vec2, Vec2) // subtexture, pos, size
{
    let scale = font_size / FONT_SCALE as f32;
    let line_spacing = (font_size as f32 * TEXT_LINE_SPACING_FACTOR).round();

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
                        pos.y += line_spacing;
                    }
                }

                // Callback
                let font_texture = font_system.fonts.get(&font_id).unwrap().texture.clone();
                let uvs = char_data.get_uvs();

                callback(
                    Subtexture {
                        texture: font_texture,
                        uvs,
                    },
                    pos + char_top_left,
                    size,
                );

                // Update max size
                max_size.x = max_size.x.max(pos.x + char_top_left.x + size.x);
            }

            max_size.y = max_size.y.max(pos.y + font_size);

            start_of_line = false;
            pos.x += advance;
            if let Some(max_width) = max_width {
                if pos.x + advance > max_width as f32 {
                    pos.x = 0.0;
                    pos.y += line_spacing;
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

        text:      &str,
        transform: Transform,
        font_size: f32,
        color:     Color,

        font_id:   Option<FontId>,
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

        text:      &str,
        transform: Transform,
        font_size: f32,
        color:     Color,

        font_id:   Option<FontId>,
        max_width: Option<u32>,
    ) {
        calculate_draw_text_size_with_callback(
            font_system,
            text,
            font_size,
            font_id,
            max_width,
            |subtexture, pos: Vec2, size| {
                self.queue_draw_sprite(
                    transform,
                    Sprite {
                        subtexture,
                        texture_flip: TextureFlip::NO,
                        pivot: -pos,
                        size,
                    },
                    color,
                );
            }
        );
    }
}

impl App<'_> {
    pub fn queue_draw_text(
        &mut self,

        text:      &str,
        transform: Transform,
        font_size: f32,
        color:     Color,

        font_id:   Option<FontId>,
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
        text:      &str,
        font_size: f32,
        font_id:   Option<FontId>,
        max_width: Option<u32>,
    ) -> Vec2 {
        calculate_draw_text_size_with_callback(
            &self.font_system,
            text,
            font_size,
            font_id,
            max_width,
            |_subtexture, _pos, _size| {}
        )
    }
}
