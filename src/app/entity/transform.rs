use crate::linalg::Vec2;
use crate::app::imgui::ImDraw;

#[derive(Copy, Clone, Debug, ImDraw)]
pub struct Transform {
    pub pos: Vec2,
    //pub scale: Vec2,
    pub rot: f32,
    pub layer: i32,
}
