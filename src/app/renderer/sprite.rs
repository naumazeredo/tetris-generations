use crate::linalg::Vec2;
use crate::app::{
    App,
    imgui_wrapper::ImDraw,
    transform::Transform,
};
use super::*;

#[derive(Clone, Debug, Default, ImDraw)]
pub struct Sprite {
    pub subtexture:   Subtexture,
    pub texture_flip: TextureFlip,
    pub pivot:        Vec2,
    pub size:         Vec2,
}

// @TODO builder
impl Sprite {
    pub fn new(texture: TextureRef, x: u32, y: u32, w: u32, h: u32) -> Self {
        Self {
            subtexture:   Subtexture::new(texture, x, y, w, h),
            texture_flip: TextureFlip::NO,
            pivot:        Vec2::new(),
            size:         Vec2 { x: w as f32, y: h as f32 },
        }
    }
}

impl Batch {
    pub fn queue_draw_sprite(
        &mut self,
        transform: Transform,
        sprite:    Sprite,
        color:     Color,
    ) {
        self.cmds.push(
            DrawCommand::Draw(DrawCommandData {
                material: None,
                texture:  Some(sprite.subtexture.texture.clone()),
                size:     sprite.size,
                color,
                transform,
                variant: DrawVariant::Sprite {
                    texture_flip: sprite.texture_flip,
                    pivot:        sprite.pivot,
                    uvs:          sprite.subtexture.uvs,
                }
            })
        );
    }
}

impl App<'_> {
    pub fn queue_draw_sprite(
        &mut self,
        transform: Transform,
        sprite:    Sprite,
        color:     Color,
    ) {
        self.renderer.batch.queue_draw_sprite(transform, sprite, color);
    }
}
