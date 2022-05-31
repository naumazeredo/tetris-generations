use std::rc::Rc;
use std::cell::RefCell;

use bitflags::bitflags;
use super::*;

pub(in crate::app) type FramebufferID = GLuint;


#[derive(Clone, Debug, ImDraw)]
pub struct Framebuffer {
    pub(super) id: FramebufferID,
    pub(super) width:  u32,
    pub(super) height: u32,

    // @TODO add a list of attachments. We will need to pass and store the texture format (in
    //       Texture) to be able to either create an R/RG/RGB or DepthStencil texture
    pub(super) color_texture: TextureRef,
}

pub type FramebufferRef = Rc<RefCell<Framebuffer>>;

impl Framebuffer {
    pub fn new(width: u32, height: u32, color_texture: TextureRef) -> FramebufferRef {
        let mut fbo = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                color_texture.borrow().id,
                0
            );
        }

        Rc::new(RefCell::new(Self {
            id: fbo,
            width,
            height,
            color_texture: color_texture,
        }))
    }

    pub fn clear(&mut self, buffer_clear: BufferClear) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
            gl::Disable(gl::SCISSOR_TEST);
            gl::ColorMask(true as _, true as _, true as _, true as _);
            gl::ClearColor(buffer_clear.color.r, buffer_clear.color.g, buffer_clear.color.b, buffer_clear.color.a);
            gl::ClearDepth(buffer_clear.depth as _);
            gl::ClearStencil(buffer_clear.stencil as _);
            gl::Clear(buffer_clear.clear_mask.to_gl());
        }
    }

    pub(in crate::app) fn bind(&mut self) { //, _renderer: &mut Renderer) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
            gl::Disable(gl::DEPTH_TEST); // @Hack it should depend on framebuffer configuration
        }

        //renderer.change_viewport((self.width, self.height)); // Is this necessary?
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.id);
        }
    }
}

// @TODO move clear stuff to another file

bitflags! {
    pub struct ClearMask : u8 {
        const COLOR   = 0b001;
        const DEPTH   = 0b010;
        const STENCIL = 0b100;
    }
}

const CLEAR_MASK_TO_GL_TABLE: [GLbitfield; 8] = [
    0                    | 0                    | 0,
    gl::COLOR_BUFFER_BIT | 0                    | 0,
    0                    | gl::DEPTH_BUFFER_BIT | 0,
    gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | 0,
    0                    | 0                    | gl::STENCIL_BUFFER_BIT,
    gl::COLOR_BUFFER_BIT | 0                    | gl::STENCIL_BUFFER_BIT,
    0                    | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT,
    gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT,
];

impl ClearMask {
    #[inline(always)] fn to_gl(self) -> GLbitfield {
        CLEAR_MASK_TO_GL_TABLE[self.bits() as usize]
    }
}


pub struct BufferClear {
    color: Color,
    depth: f32,
    stencil: u8,
    clear_mask: ClearMask,
}

impl BufferClear {
    pub fn new() -> Self {
        Self {
            color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
            depth: 1.0,
            stencil: 0,
            clear_mask: ClearMask::empty(),
        }
    }

    pub fn color(self, color: Color) -> Self {
        Self {
            color,
            clear_mask: self.clear_mask | ClearMask::COLOR,
            ..self
        }
    }

    pub fn depth(self, depth: f32) -> Self {
        Self {
            depth,
            clear_mask: self.clear_mask | ClearMask::DEPTH,
            ..self
        }
    }

    pub fn stencil(self, stencil: u8) -> Self {
        Self {
            stencil,
            clear_mask: self.clear_mask | ClearMask::STENCIL,
            ..self
        }
    }
}
