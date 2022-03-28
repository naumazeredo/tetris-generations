use bitflags::bitflags;
use std::path::Path;
use gl::types::*;
use sdl2::image::*;
use imgui::{im_str, TreeNode};

use crate::app::imgui_wrapper::ImDraw;

pub(in crate::app) type TextureObject = GLuint;

#[derive(Copy, Clone, Debug)]
enum TextureFormat {
    R,
    RG,
    RGB,
    RGBA,
    DepthStencil,
}

// @Refactor maybe own the object and free on Drop?
//           or maybe just manage what's in GPU properly (not so safe, right?)
#[derive(PartialEq, Copy, Clone, Debug, ImDraw, Default)]
pub struct Texture {
    pub(in crate::app) obj: TextureObject,
    pub w: u32,
    pub h: u32,
    pub white_pixel: Option<(u32, u32)>,
}

impl Texture {
    pub fn new(w: u32, h: u32, pixels: Option<&[u8]>) -> Self {
        let mut obj : TextureObject = 0;

        unsafe {
            gl::GenTextures(1, &mut obj);
            gl::BindTexture(gl::TEXTURE_2D, obj);

            gl::PixelStorei(gl::UNPACK_ROW_LENGTH, 0);

            let pixels = pixels.and_then(|x| Some(x.as_ptr())).unwrap_or(0 as *const u8);
            gl::TexImage2D(
                gl::TEXTURE_2D,    // GLenum target,
                0,                 // GLint level,
                gl::RGBA as _,     // GLint internalformat,
                w as _,            // GLsizei width,
                h as _,            // GLsizei height,
                0,                 // GLint border,
                gl::RGBA,          // GLenum format,
                gl::UNSIGNED_BYTE, // GLenum type,
                pixels as _        // const void * data
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Texture { obj, w, h, white_pixel: None }
    }

    /*
    // @XXX this would not be safe since we can copy the texture object
    pub fn destroy(&mut self) {
        unsafe {
            gl::DestroyTextures
        }
    }
    */

    pub(in crate::app) fn load_from_surface(surface: sdl2::surface::Surface) -> Self {
        //let format = surface.pixel_format_enum();
        let w = surface.width();
        let h = surface.height();

        let surface = surface.convert_format(sdl2::pixels::PixelFormatEnum::RGBA32).unwrap();
        surface.with_lock(|pixels| Self::new(w, h, Some(pixels)))
    }

    pub(in crate::app) fn load_from_file<P: AsRef<Path>>(path: P) -> Self {
        use sdl2::surface::Surface;
        let surface = Surface::from_file(path)
            .unwrap_or_else(|err| {
                panic!("Surface could not be loaded: {}", err)
            });

        Self::load_from_surface(surface)
    }

    pub(in crate::app) fn with_white_pixel(self, white_pixel: (u32, u32)) -> Self {
        Self {
            white_pixel: Some(white_pixel),
            ..self
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
