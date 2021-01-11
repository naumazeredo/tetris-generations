//use std::ffi::CString;
use imgui::*;
pub use imdraw_derive::ImDraw;

pub trait ImDraw {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui);
}

#[macro_export]
macro_rules! im_str2 {
    ($e:tt) => ({
        &$crate::ImString::new($e)
    });
}

impl ImDraw for u32 {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        Drag::new(im_str2!(label)).build(ui, self);
    }
}

impl ImDraw for i32 {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        Drag::new(im_str2!(label)).build(ui, self);
    }
}

impl ImDraw for f32 {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        Drag::new(im_str2!(label)).speed(0.1).build(ui, self);
    }
}
