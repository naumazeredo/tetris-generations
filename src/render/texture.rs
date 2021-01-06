//extern crate bitflags;
//extern crate gl;

use gl::types::*;
//use super::types::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Texture {
    obj: GLuint,
    w: u32,
    h: u32,
}

impl Texture {
    pub fn new() -> Self {
        Self {
            obj: 0 as GLuint,
            w: 1,
            h: 1,
        }
    }
}

bitflags! {
    pub struct TextureFlip: u8 {
        const NO = 0b00;
        const X  = 0b01;
        const Y  = 0b10;
        const XY = 0b11;
    }
}
