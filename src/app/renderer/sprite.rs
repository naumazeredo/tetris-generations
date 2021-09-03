use crate::linalg::{Vec2, Vec2i};

use crate::app::{
    App,
    imgui_wrapper::ImDraw,
    transform::Transform,
};

use super::{
    Renderer,
    color::Color,
    draw_command::{Command, DrawCommand, DrawVariant},
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

pub(super) fn queue_draw_sprite(
    renderer: &mut Renderer,
    transform: &Transform,
    sprite: &Sprite,
    color: Color,
) {
    renderer.world_cmds.push(
        Command::Draw(DrawCommand {
            program: renderer.default_program,
            texture: sprite.texture,
            layer: transform.layer,
            color,
            pos: transform.pos,
            scale: transform.scale,
            rot: transform.rot,
            variant: DrawVariant::Sprite {
                texture_flip: sprite.texture_flip,
                uvs: sprite.uvs,
                pivot: sprite.pivot,
                size: sprite.size,
            },
        })
    );
}

impl App<'_> {
    pub fn queue_draw_sprite(
        &mut self,
        transform: &Transform,
        sprite: &Sprite,
        color: Color,
    ) {
        queue_draw_sprite(&mut self.renderer, transform, sprite, color);
    }
}
