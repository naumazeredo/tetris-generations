use crate::linalg::{Vec2, Vec2i};
use crate::app::{App, ImDraw, Transform};
use super::{
    Renderer,
    ShaderProgram,
    color::Color,
    texture::{Texture, TextureFlip},
};

#[derive(Clone, Debug, ImDraw)]
pub(in crate::app) enum DrawVariant {
    Solid {
        size: Vec2,
    },
    Sprite {
        texture_flip: TextureFlip,
        uvs: (Vec2i, Vec2i),
        pivot: Vec2,
        size: Vec2,
    },
}

#[derive(Clone, Debug, ImDraw)]
pub(in crate::app) struct DrawCommand {
    pub(in crate::app) program: ShaderProgram,
    pub(in crate::app) texture: Texture,
    pub(in crate::app) color: Color,

    pub(in crate::app) pos: Vec2,
    pub(in crate::app) scale: Vec2,
    pub(in crate::app) rot: f32,
    pub(in crate::app) layer: i32,

    pub(in crate::app) variant: DrawVariant,
}

#[derive(Clone, Debug, ImDraw)]
pub(in crate::app) enum Command {
    Draw(DrawCommand),

    PushClip {
        min: Vec2i,
        max: Vec2i,
        intersect: bool,
    },

    PopClip,
}

pub(in crate::app) fn queue_draw_solid(
    renderer: &mut Renderer,
    transform: &Transform,
    size: Vec2,
    color: Color,
) {
    renderer.world_cmds.push(
        Command::Draw(DrawCommand {
            program: renderer.default_program,
            texture: renderer.default_texture,
            layer: transform.layer,
            color,
            pos: transform.pos,
            scale: transform.scale,
            rot: transform.rot,
            variant: DrawVariant::Solid { size },
        })
    );
}

// @Incomplete
pub(in crate::app) fn queue_draw_quad(
    renderer: &mut Renderer,
    transform: &Transform,
    size: Vec2,
    color: Color,
) {
    renderer.world_cmds.push(
        Command::Draw(DrawCommand {
            program: renderer.default_program,
            texture: renderer.default_texture,
            layer: transform.layer,
            color,
            pos: transform.pos,
            scale: transform.scale,
            rot: transform.rot,
            variant: DrawVariant::Solid { size },
        })
    );
}

// @Refactor this seems strange as a global function. Either turn it into a Renderer method or
//           create a batch rendering struct that holds this data temporarily and apply it to the
//           renderer (or render to a framebuffer)

pub(in crate::app) fn push_clip(
    renderer: &mut Renderer,
    pos: Vec2i,
    size: Vec2i,
) {
    assert!(size.x >= 0);
    assert!(size.y >= 0);

    renderer.world_cmds.push(
        Command::PushClip {
            min: pos,
            max: pos + size,
            intersect: false,
        }
    );
}

pub(in crate::app) fn pop_clip(renderer: &mut Renderer) {
    renderer.world_cmds.push(Command::PopClip);
}

impl App<'_> {
    // @TODO move this somewhere else? To color module maybe?
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

    pub fn push_clip(
        &mut self,
        pos: Vec2i,
        size: Vec2i,
    ) {
        push_clip(&mut self.renderer, pos, size);
    }

    pub fn pop_clip(&mut self) {
        pop_clip(&mut self.renderer);
    }
}
