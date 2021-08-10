use crate::linalg::{Vec2, Vec2i};
use crate::app::{App, ImDraw, Transform};
use super::{
    Renderer,
    ShaderProgram,
    color::Color,
    texture::{Texture, TextureFlip},
};

#[derive(Copy, Clone, Debug, ImDraw)]
pub enum Command {
    DrawSolid {
        size: Vec2,
    },
    DrawSprite {
        texture_flip: TextureFlip,
        uvs: (Vec2i, Vec2i),
        pivot: Vec2,
        size: Vec2,
    },
}

#[derive(Copy, Clone, Debug, ImDraw)]
pub struct DrawCommand {
    pub program: ShaderProgram,
    pub texture: Texture,
    pub color: Color,

    pub pos: Vec2,
    pub scale: Vec2,
    pub rot: f32,
    pub layer: i32,

    pub cmd: Command,
}

pub(in crate::app) fn queue_draw_solid(
    renderer: &mut Renderer,
    transform: &Transform,
    size: Vec2,
    color: Color,
) {
    renderer.world_draw_cmds.push(DrawCommand {
        program: renderer.default_program,
        texture: renderer.default_texture,
        layer: transform.layer,
        color,
        pos: transform.pos,
        scale: transform.scale,
        rot: transform.rot,
        cmd: Command::DrawSolid { size },
    });
}

// @Incomplete
pub(in crate::app) fn queue_draw_quad(
    renderer: &mut Renderer,
    transform: &Transform,
    size: Vec2,
    color: Color,
) {
    renderer.world_draw_cmds.push(DrawCommand {
        program: renderer.default_program,
        texture: renderer.default_texture,
        layer: transform.layer,
        color,
        pos: transform.pos,
        scale: transform.scale,
        rot: transform.rot,
        cmd: Command::DrawSolid { size },
    });
}

impl<S> App<'_, S> {
    pub fn queue_draw_solid(
        &mut self,
        transform: &Transform,
        size: Vec2,
        color: Color,
    ) {
        queue_draw_solid(&mut self.renderer, transform, size, color);
    }

    pub fn queue_draw_quad(
        &mut self,
        transform: &Transform,
        size: Vec2,
        color: Color,
    ) {
        queue_draw_quad(&mut self.renderer, transform, size, color);
    }
}
