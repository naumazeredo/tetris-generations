use super::*;
use crate::linalg::{Vec2, Vec2i};
use crate::app::transform::Transform;

#[derive(Clone, Debug, ImDraw)]
pub(super) enum DrawVariant {
    Solid,
    Sprite {
        texture_flip: TextureFlip,
        pivot: Vec2,
        uvs: (Vec2i, Vec2i),
    },
}

#[derive(Clone, Debug, ImDraw)]
pub(super) struct DrawCommandData {
    pub(super) material:  Option<MaterialRef>,
    pub(super) texture:   Option<TextureRef>,
    pub(super) size:      Vec2,
    pub(super) color:     Color,
    pub(super) transform: Transform,
    pub(super) variant:   DrawVariant,
}

#[derive(Clone, Debug, ImDraw)]
pub(super) enum DrawCommand {
    Draw(DrawCommandData),

    PushClip {
        min: Vec2i,
        max: Vec2i,
        intersect: bool,
    },

    PopClip,
}
