use crate::linalg::{Vec2, Vec2i};
use crate::app::imgui::ImDraw;
use super::{
    texture::{Texture, TextureFlip},
};

// @Maybe we need to remove some of these fields from Sprite

#[derive(Copy, Clone, Debug, ImDraw)]
pub struct Sprite {
    pub texture: Texture,
    pub texture_flip: TextureFlip,
    pub uvs: (Vec2i, Vec2i),
    pub pivot: Vec2,
    pub size: Vec2,
}
