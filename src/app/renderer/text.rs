use crate::app::{
    font_system::{
        FONT_SCALE,
        FontId,
        FontSystem,
    },
    transform::Transform,
};
use super::{
    Color,
    Renderer,
    TextureFlip,
    sprite::{
        queue_draw_sprite,
        Sprite,
    },
};
use crate::linalg::Vec2;

pub(in crate::app) fn queue_draw_text(
    renderer: &mut Renderer,
    font_system: &FontSystem,

    //program: ShaderProgram,
    text: &str,
    font: FontId,
    transform: &Transform,
    font_size: f32,
    color: Color,
) {
    // @Refactor hold the font reference instead of getting it every char
    let font_texture = font_system.fonts.get(&font).unwrap().texture;

    let mut pos = Vec2::new();
    for ch in text.chars() {
        if let Some(&char_data) = font_system.fonts.get(&font).unwrap().get_char_data(ch) {
            let uvs = char_data.get_uvs();

            let scale = font_size / FONT_SCALE as f32;
            let char_top_left = Vec2 {
                x:  char_data.metrics.minx as f32 * scale,
                y: -char_data.metrics.maxy as f32 * scale
            };
            let size = Vec2 {
                x: char_data.metrics.w as f32 * scale,
                y: char_data.metrics.h as f32 * scale
            };

            queue_draw_sprite(
                renderer,

                transform,
                &Sprite {
                    texture: font_texture,
                    texture_flip: TextureFlip::NO,
                    uvs,
                    pivot: - (pos + char_top_left),
                    size,
                },
                color,
            );

            let advance = char_data.metrics.advance as f32 * scale;
            pos.x += advance;
        }
    }
}
