use crate::linalg::Vec2;
use crate::app::imgui_wrapper::ImDraw;

#[derive(Copy, Clone, Debug, ImDraw)]
pub struct Transform {
    pub pos:   Vec2,
    pub scale: Vec2,
    pub rot:   f32,
    // pub pivot: Vec2,
    pub layer: i32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            pos:   Vec2::new(),
            scale: Vec2::identity(),
            rot:   0.0,
            layer: 0,
        }
    }
}

pub struct TransformBuilder {
    transform: Transform,
}

impl TransformBuilder {
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
        }
    }

    pub fn pos(mut self, pos: Vec2) -> Self {
        self.transform.pos = pos;
        self
    }

    pub fn pos_xy(mut self, x: f32, y: f32) -> Self {
        self.transform.pos = Vec2 { x, y };
        self
    }

    pub fn scale(mut self, scale: Vec2) -> Self {
        self.transform.scale = scale;
        self
    }

    pub fn scale_xy(mut self, x: f32, y: f32) -> Self {
        self.transform.scale = Vec2 { x, y };
        self
    }

    pub fn rot(mut self, rot: f32) -> Self {
        self.transform.rot = rot;
        self
    }

    pub fn layer(mut self, layer: i32) -> Self {
        self.transform.layer = layer;
        self
    }

    pub fn build(self) -> Transform {
        self.transform
    }
}
