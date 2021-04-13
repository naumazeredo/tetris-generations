use crate::linalg::{Vec2, Vec2i};

use crate::app::{
    imgui::ImDraw,
    transform::Transform,
};

use super::{
    Renderer,
    color::Color,
    draw_command::{Command, DrawCommand},
    texture::{Texture, TextureFlip},
};

// @Maybe we need to remove some of these fields from Sprite

#[derive(Copy, Clone, Debug, Default, ImDraw)]
pub struct Sprite {
    pub texture: Texture,
    pub texture_flip: TextureFlip,
    pub uvs: (Vec2i, Vec2i),
    pub pivot: Vec2,
    pub size: Vec2,
}

// @Refactor move to App
impl Renderer {
    pub fn queue_draw_sprite(
        &mut self,
        transform: &Transform,
        sprite: &Sprite,
        color: Color,
    ) {
        self.world_draw_cmds.push(DrawCommand {
            program: self.default_program,
            texture: sprite.texture,
            layer: transform.layer,
            color,
            pos: transform.pos,
            rot: transform.rot,
            cmd: Command::DrawSprite {
                texture_flip: sprite.texture_flip,
                uvs: sprite.uvs,
                pivot: sprite.pivot,
                size: sprite.size,
            },
        });
    }
}
