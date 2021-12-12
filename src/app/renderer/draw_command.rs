use super::*;
use crate::linalg::{Vec2, Vec2i};

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
    pub(in crate::app) program: Option<ShaderProgram>,
    pub(in crate::app) texture: Option<Texture>,
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
