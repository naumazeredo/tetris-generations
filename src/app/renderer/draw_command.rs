use crate::linalg::{Vec2, Vec2i};
use crate::app::Transform;
use super::{
    Renderer,
    sprite::Sprite,
    texture::{Texture, TextureFlip},
    types::*,
};

#[derive(Copy, Clone, Debug)]
pub enum Command {
    DrawSprite {
        texture: Texture,
        texture_flip: TextureFlip,
        uvs: (Vec2i, Vec2i),
        pivot: Vec2,
        size: Vec2,
    },
}

#[derive(Copy, Clone, Debug)]
pub struct DrawCommand {
    pub program: Program,
    pub color: Color,

    pub pos: Vec2,
    pub rot: f32,
    pub layer: i32,

    pub cmd: Command,
}

// @Refactor move to App
impl Renderer {
    pub fn queue_draw_sprite(
        &mut self,
        program: Program,
        color: Color,
        transform: &Transform,
        sprite: &Sprite,
    ) {
        self.world_draw_cmds.push(DrawCommand {
            program,
            layer: transform.layer,
            color,
            pos: transform.pos,
            rot: transform.rot,
            cmd: Command::DrawSprite {
                texture: sprite.texture,
                texture_flip: sprite.texture_flip,
                uvs: sprite.uvs,
                pivot: sprite.pivot,
                size: sprite.size,
            },
        });
    }
}
