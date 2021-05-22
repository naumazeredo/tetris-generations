use std::path::Path;
use gl::types::*;
use sdl2::image::*;
use imgui::{im_str, TreeNode};

use crate::app::imgui::ImDraw;

pub type TextureObject  = GLuint;

// @Maybe we should accept default in texture

// @Refactor maybe own the object and free on Drop?
//           or maybe just manage what's in GPU properly (not so safe, right?)
#[derive(PartialEq, Copy, Clone, Debug, ImDraw, Default)]
pub struct Texture {
    pub obj: TextureObject,
    pub w: u32,
    pub h: u32,
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
    #[derive(Default)]
    pub struct TextureFlip: u8 {
        const NO = 0b00;
        const X  = 0b01;
        const Y  = 0b10;
        const XY = 0b11;
    }
}

pub(in crate::app) fn load_texture_from_surface(surface: sdl2::surface::Surface) -> Texture {
    //let format = surface.pixel_format_enum();
    let w = surface.width();
    let h = surface.height();

    let mut obj : TextureObject = 0;

    //let surface = surface.convert_format(sdl2::pixels::PixelFormatEnum::RGBA32).unwrap();
    surface.with_lock(|pixels| unsafe {
        gl::GenTextures(1, &mut obj);
        gl::BindTexture(gl::TEXTURE_2D, obj);

        gl::PixelStorei(gl::UNPACK_ROW_LENGTH, 0);

        // @TODO verify image format
        gl::TexImage2D(
            gl::TEXTURE_2D, // GLenum target,
            0, // GLint level,
            gl::RGBA as _, // GLint internalformat,
            w as _, // GLsizei width,
            h as _, // GLsizei height,
            0, // GLint border,
            gl::RGBA, // GLenum format,
            gl::UNSIGNED_BYTE, // GLenum type,
            pixels.as_ptr() as _ // const void * data);
        );

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);
    });

    Texture { obj, w, h }
}

pub(in crate::app) fn load_texture<P: AsRef<Path>>(path: P) -> Texture {
    use sdl2::surface::Surface;
    let surface = Surface::from_file(path)
        .unwrap_or_else(|err| {
            panic!("Surface could not be loaded: {}", err)
        });

    load_texture_from_surface(surface)
}

// ------
// ImDraw
// ------
impl ImDraw for TextureFlip {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        TreeNode::new(im_str2!(label)).build(ui, || {
            let mut flip_x = self.contains(TextureFlip::X);
            ui.checkbox(im_str!("flip x"), &mut flip_x);

            let mut flip_y = self.contains(TextureFlip::Y);
            ui.checkbox(im_str!("flip y"), &mut flip_y);

            let mut texture_flip = TextureFlip::NO;
            if flip_x { texture_flip |= TextureFlip::X; }
            if flip_y { texture_flip |= TextureFlip::Y; }
            *self = texture_flip;
        });
    }
}
