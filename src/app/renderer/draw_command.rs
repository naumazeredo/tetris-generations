use crate::linalg::{Vec2, Vec2i};
use super::types::*;
use super::texture::{Texture, TextureFlip};
use super::Renderer;

#[derive(Copy, Clone, Debug)]
pub enum Command {
    DrawSprite {
        size: Vec2,
        texture: Texture,
        texture_flip: TextureFlip,
        uvs: (Vec2i, Vec2i),
    },
}

#[derive(Copy, Clone, Debug)]
pub struct DrawCommand {
    pub program: Program,
    pub layer: i32,
    pub color: Color,

    pub pos: Vec2,
    pub pivot: Vec2,
    pub rot: f32,

    pub cmd: Command,
}

// @Refactor move to App
impl Renderer {
    pub fn queue_draw_sprite(
        &mut self,
        program: Program,
        layer: i32,
        color: Color,
        pos: Vec2,
        size: Vec2,
        rot: f32,
        pivot: Vec2,
        texture: Texture,
        texture_flip: TextureFlip,
        uvs: (Vec2i, Vec2i)
    ) {
        self.world_draw_cmds.push(DrawCommand {
            program,
            layer,
            color,
            pos,
            pivot,
            rot,
            cmd: Command::DrawSprite {
                size,
                texture,
                texture_flip,
                uvs,
            },
        });
    }
}
