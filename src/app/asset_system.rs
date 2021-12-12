use std::path::Path;
use std::collections::BTreeMap;

use super::{
    App,
    ImDraw,
    renderer::{
        texture::Texture,
        framebuffer::Framebuffer,
    },
    utils::string_ref::StringRef,
};

// @Refactor AssetSystem should just handle what should be loaded and what should be unloaded, not
//           hold all data from all systems (the systems will hold their data)
#[derive(ImDraw)]
pub(super) struct AssetSystem {
    textures: BTreeMap<StringRef, Texture>,
    framebuffers: BTreeMap<StringRef, Framebuffer>,
}

impl AssetSystem {
    pub(super) fn new() -> Self {
        Self {
            textures: BTreeMap::new(),
            framebuffers: BTreeMap::new(),
        }
    }

    fn get_texture_or_load<P: AsRef<Path>>(&mut self, path: P) -> Texture {
        *self.textures
            .entry(StringRef::new(path.as_ref().display().to_string()))
            .or_insert_with(|| Texture::load_from_file(path))
    }

    fn get_texture_or_create<P: AsRef<Path>>(
        &mut self,
        path: P,
        width: u32,
        height: u32,
        pixels: Option<&[u8]>,
    ) -> Texture {
        *self.textures
            .entry(StringRef::new(path.as_ref().display().to_string()))
            .or_insert_with(|| Texture::new(width, height, pixels))
    }

    fn get_framebuffer_or_create<P: AsRef<Path>>(
        &mut self,
        path: P,
        texture: Texture,
    ) -> Framebuffer {
        *self.framebuffers
            .entry(StringRef::new(path.as_ref().display().to_string()))
            .or_insert_with(|| Framebuffer::new(texture.w, texture.h, texture))
    }
}

impl App<'_> {
    // @Refactor this is bad since we always have to take the whole path string.
    //           We should get a StringRef instead and not use Path if not necessary
    pub fn get_texture_or_load<P: AsRef<Path>>(&mut self, path: P) -> Texture {
        self.asset_system.get_texture_or_load(path)
    }

    pub fn get_texture_or_create<P: AsRef<Path>>(
        &mut self,
        path: P,
        width: u32,
        height: u32,
        pixels: Option<&[u8]>,
    ) -> Texture {
        self.asset_system.get_texture_or_create(path, width, height, pixels)
    }

    pub fn get_framebuffer_or_create<P: AsRef<Path>>(
        &mut self,
        path: P,
        texture: Texture
    ) -> Framebuffer {
        self.asset_system.get_framebuffer_or_create(path, texture)
    }
}
