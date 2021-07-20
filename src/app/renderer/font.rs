use crate::{
    app::{
        App,
        font_system::FontRef,
    },
    linalg::Vec2,
    transform::Transform,
};

use super::{
    color::Color,
    sprite::Sprite,
    texture::TextureFlip,
};

impl<S> App<'_, S>{
    pub fn queue_draw_text(
        &mut self,
        //program: Program,
        text: &str,
        font_ref: FontRef,
        transform: &Transform,
        font_size: f32,
        color: Color,
    ) {
        let font_texture = self.font_system.fonts.get(&font_ref).unwrap().texture;

        let mut pos = Vec2::new();
        for ch in text.chars() {
            if let Some(&char_data) = self.font_system.fonts.get(&font_ref).unwrap().get_char_data(ch) {
                let uvs = char_data.get_uvs();

                let scale = font_size / 64.;
                let char_top_left = Vec2 {
                    x:  char_data.metrics.minx as f32 * scale,
                    y: -char_data.metrics.maxy as f32 * scale
                };
                let size = Vec2 {
                    x: char_data.metrics.w as f32 * scale,
                    y: char_data.metrics.h as f32 * scale
                };

                self.queue_draw_sprite(
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
}
