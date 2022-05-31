use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use bitflags::bitflags;
use gl::types::*;
use sdl2::image::*;
use imgui::{im_str, TreeNode};

use crate::app::imgui_wrapper::ImDraw;

pub(in crate::app) type TextureID = GLuint;

#[derive(Copy, Clone, Debug)]
enum TextureFormat {
    R,
    RG,
    RGB,
    RGBA,
    DepthStencil,
}

#[derive(Debug, ImDraw, Default)]
pub struct Texture {
    pub(in crate::app) id: TextureID,
    pub w: u32,
    pub h: u32,
    pub white_pixel: Option<(u32, u32)>,
}

pub type TextureRef = Rc<RefCell<Texture>>;

impl Texture {
    pub fn new(w: u32, h: u32, pixels: Option<&[u8]>) -> TextureRef {
        let mut id: TextureID = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

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

            // @TODO create TextureSampler to abstract wrap+filter
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Rc::new(RefCell::new(Texture { id, w, h, white_pixel: None }))
    }

    pub(in crate::app) fn load_from_surface(surface: sdl2::surface::Surface) -> TextureRef {
        //let format = surface.pixel_format_enum();
        let w = surface.width();
        let h = surface.height();

        let surface = surface.convert_format(sdl2::pixels::PixelFormatEnum::RGBA32).unwrap();
        surface.with_lock(|pixels| Self::new(w, h, Some(pixels)))
    }

    pub(in crate::app) fn load_from_file<P: AsRef<Path>>(path: P) -> TextureRef {
        use sdl2::surface::Surface;
        let surface = Surface::from_file(path)
            .unwrap_or_else(|err| {
                panic!("Surface could not be loaded: {}", err)
            });

        Self::load_from_surface(surface)
    }

    // @Refactor this is not clean on use. Either we need to create a TextureBuilder or see a better
    //           way of doing this
    pub(in crate::app) fn set_white_pixel(&mut self, white_pixel: (u32, u32)) {
        self.white_pixel = Some(white_pixel);
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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
