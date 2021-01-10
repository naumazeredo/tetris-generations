use crate::linalg::vec2::Vec2;
use crate::linalg::vec2i::Vec2i;
use super::types::*;
use super::texture::{Texture, TextureFlip};
use super::Render;

#[derive(Copy, Clone, Debug)]
pub enum Command {
    DrawSprite {
        size: Vec2,
        texture: Texture,
        texture_flip: TextureFlip,
        uv: (Vec2i, Vec2i),
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

impl Render {
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
        uv: (Vec2i, Vec2i)
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
                uv,
            },
        });
    }
}
