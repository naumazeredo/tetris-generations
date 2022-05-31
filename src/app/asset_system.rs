use std::path::Path;
use std::collections::BTreeMap;

use super::{
    App,
    ImDraw,
    renderer::{
        texture::{ Texture, TextureRef },
        framebuffer::{ Framebuffer, FramebufferRef },
        shader::{ Shader, ShaderRef },
    },
    utils::string_ref::StringRef,
};

// @Refactor AssetSystem should just handle what should be loaded and what should be unloaded, not
//           hold all data from all systems (the systems will hold their data)
#[derive(ImDraw)]
pub(super) struct AssetSystem {
    textures     : BTreeMap<StringRef, TextureRef>,
    framebuffers : BTreeMap<StringRef, FramebufferRef>,
    shaders      : BTreeMap<StringRef, ShaderRef>,
}

impl AssetSystem {
    pub(super) fn new() -> Self {
        Self {
            textures     : BTreeMap::new(),
            framebuffers : BTreeMap::new(),
            shaders      : BTreeMap::new(),
        }
    }

    fn get_texture_or_load<P: AsRef<Path>>(&mut self, path: P) -> TextureRef {
        self.textures
            .entry(StringRef::new(path.as_ref().to_str().unwrap()))
            .or_insert_with(|| Texture::load_from_file(path))
            .clone()
    }

    fn get_texture_or_create<P: AsRef<Path>>(
        &mut self,
        path: P,
        width: u32,
        height: u32,
        pixels: Option<&[u8]>,
    ) -> TextureRef {
        self.textures
            .entry(StringRef::new(path.as_ref().to_str().unwrap()))
            .or_insert_with(|| Texture::new(width, height, pixels))
            .clone()
    }

    fn get_framebuffer_or_create<P: AsRef<Path>>(
        &mut self,
        path: P,
        texture: TextureRef,
    ) -> FramebufferRef {
        let w = texture.borrow().w;
        let h = texture.borrow().h;

        self.framebuffers
            .entry(StringRef::new(path.as_ref().to_str().unwrap()))
            .or_insert_with(|| Framebuffer::new(w, h, texture))
            .clone()
    }

    fn get_shader_or_create<P: AsRef<Path>>(
        &mut self,
        vs: P,
        fs: P,
    ) -> ShaderRef {
        self.shaders
            .entry(
                StringRef::new(
                    &(vs.as_ref().to_str().unwrap().to_owned() + fs.as_ref().to_str().unwrap())
                )
            )
            .or_insert_with(|| Shader::new(vs, fs))
            .clone()

    }

}

impl App<'_> {
    // @Refactor this is bad since we always have to take the whole path string.
    //           We should get a StringRef instead and not use Path if not necessary
    pub fn get_texture_or_load<P: AsRef<Path>>(&mut self, path: P) -> TextureRef {
        self.asset_system.get_texture_or_load(path)
    }

    pub fn get_texture_or_create<P: AsRef<Path>>(
        &mut self,
        path: P,
        width: u32,
        height: u32,
        pixels: Option<&[u8]>,
    ) -> TextureRef {
        self.asset_system.get_texture_or_create(path, width, height, pixels)
    }

    pub fn get_framebuffer_or_create<P: AsRef<Path>>(
        &mut self,
        path: P,
        texture: TextureRef,
    ) -> FramebufferRef {
        self.asset_system.get_framebuffer_or_create(path, texture)
    }
}
