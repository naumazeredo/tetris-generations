use crate::imdraw::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ImDraw for Color {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        let mut c: [f32; 4] = (*self).into();
        imgui::ColorEdit::new(im_str2!(label), &mut c).build(&ui);
        *self = Color::from(c);
    }
}

#[allow(dead_code)]
pub static WHITE: Color  = Color { r: 1., g: 1., b: 1., a: 1. };
#[allow(dead_code)]
pub static BLAC: Color   = Color { r: 0., g: 0., b: 0., a: 1. };
#[allow(dead_code)]
pub static RE: Color     = Color { r: 1., g: 0., b: 0., a: 1. };
#[allow(dead_code)]
pub static GREE: Color   = Color { r: 0., g: 1., b: 0., a: 1. };
#[allow(dead_code)]
pub static BLU: Color    = Color { r: 0., g: 0., b: 1., a: 1. };
#[allow(dead_code)]
pub static MAGENT: Color = Color { r: 1., g: 0., b: 1., a: 1. };

impl From<[f32; 4]> for Color {
    #[inline]
    fn from(array: [f32; 4]) -> Self {
        Self { r: array[0], g: array[1], b: array[2], a: array[3] }
    }
}

impl Into<[f32; 4]> for Color {
    #[inline]
    fn into(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}
