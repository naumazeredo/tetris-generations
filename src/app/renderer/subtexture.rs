use crate::linalg::{Vec2, Vec2i};
use crate::app::{
    App,
    imgui_wrapper::ImDraw,
    transform::Transform,
};
use super::*;

#[derive(Copy, Clone, Debug, Default, ImDraw)]
pub struct Subtexture {
    pub texture: Texture,
    pub uvs: (Vec2i, Vec2i),
}

impl Subtexture {
    pub fn new(texture: Texture, x: u32, y: u32, w: u32, h: u32) -> Self {
        Self {
            texture,
            uvs: (
                Vec2i { x: x as i32, y: y as i32 },
                Vec2i { x: (w + x) as i32, y: (h + y) as i32 }
            ),
        }
    }
}

impl From<Texture> for Subtexture {
    fn from(texture: Texture) -> Self {
        Self {
            texture,
            uvs: (
                Vec2i { x: 0, y: 0 },
                Vec2i { x: texture.w as i32, y: texture.h as i32 },
            ),
        }
    }
}

impl Batch {
    pub fn queue_draw_texture(
        &mut self,
        transform: &Transform,
        subtexture: Subtexture,
        size: Vec2,
        color: Color,
    ) {
        self.queue_draw_sprite(
            transform,
            &Sprite {
                subtexture,
                // @Refactor this is required for any texture not loaded from files. The texture
                //           coordinates have an inverted y-axis, so we need to fix it by flipping
                //           UVs. Textures loaded from files have the coordinates fixed already
                texture_flip: TextureFlip::Y,
                pivot: Vec2::new(),
                size,
            },
            color,
        )
    }
}

impl App<'_> {
    pub fn queue_draw_texture(
        &mut self,
        transform: &Transform,
        subtexture: Subtexture,
        size: Vec2,
        color: Color,
    ) {
        self.renderer.batch.queue_draw_texture(transform, subtexture, size, color);
    }
}
