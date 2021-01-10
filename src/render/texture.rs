//extern crate bitflags;
//extern crate gl;

use std::path::Path;
use gl::types::*;
use sdl2::image::*;

pub type TextureObject  = GLuint;

// @Refactor maybe own the object and free on Drop?
//           or maybe just manage what's in GPU properly (not so safe, right?)
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Texture {
    pub obj: TextureObject,
    pub w: u32,
    pub h: u32,
}

impl Texture {
    #[allow(dead_code)]
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

pub fn load_texture<P: AsRef<Path> + Copy + std::fmt::Display>(path: P) -> Texture {
    use sdl2::surface::Surface;
    let surface = Surface::from_file(path)
        .unwrap_or_else(|err| {
            panic!(format!("Surface could not be loaded: {}", err))
        });

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

    println!("Texture loaded: {}", path);

    Texture { obj, w, h }
}

