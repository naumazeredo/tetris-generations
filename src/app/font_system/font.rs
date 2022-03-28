use std::collections::BTreeMap;
use std::path::Path;

use crate::{
    impl_imdraw_todo,
    app::{
        App,
        imgui_wrapper::ImDraw,
        renderer::Texture,
    },
    utils::string_ref::StringRef,
};

use super::{
    FONT_SCALE,
    char_data::CharData,
    packing::pack_font,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FontId(StringRef);
impl FontId {
    fn new(s: String) -> Self {
        Self(StringRef::new(s))
    }
}

impl std::fmt::Display for FontId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FontId {}", self.0)
    }
}

impl_imdraw_todo!(FontId);

#[derive(Clone, Debug, ImDraw)]
pub(in crate::app) struct Font {
    mapping: BTreeMap<char, CharData>,
    pub(in crate::app) texture: Texture,
    // @TODO ascent, descent, etc
}

impl Font {
    pub(super) fn bake<P: AsRef<Path>>(
        path: P,
        ttf_context: &sdl2::ttf::Sdl2TtfContext
    ) -> Option<Self> {
        println!("[font bake] Packing {}", path.as_ref().display());

        let scale = FONT_SCALE;
        match ttf_context.load_font(path.as_ref(), scale) {
            Ok(font) => {
                let glyphs = build_ascii_and_latin1_string();
                let (packed_surface, mapping) = pack_font(font, glyphs, scale as u32, 2, true);

                // @TODO save packed font to file?
                //packed_surface.save_bmp("tmp/font.bmp").unwrap();
                println!("[font bake] Packing complete: {}", path.as_ref().display());

                let texture = Texture::load_from_surface(packed_surface)
                    .with_white_pixel((0, 0));

                Some(Font { mapping, texture })
            },

            Err(error) => {
                println!(
                    "[font back] failed to load font {} with error: {}",
                    path.as_ref().display(),
                    error
                );
                None
            }
        }
    }

    pub(in crate::app) fn get_char_data(&self, ch: char) -> Option<&CharData> {
        self.mapping.get(&ch)
    }
}

// @TODO return Result
pub(super) fn bake_font<P: AsRef<Path>>(
    path: P,
    ttf_context: &sdl2::ttf::Sdl2TtfContext
) -> Option<(FontId, Font)> {

    let font_id = FontId::new(path.as_ref().to_string_lossy().to_string());

    // @Check if it's already baked?

    match Font::bake(path, ttf_context) {
        Some(font) => Some((font_id, font)),
        None => None,
    }
}

impl App<'_>{
    // @TODO return Result
    pub fn bake_font<P: AsRef<Path>>(&mut self, path: P) -> Option<FontId> {
        if let Some((font_id, font)) = bake_font(path, &self.sdl_context.ttf_context) {
            self.font_system.fonts.insert(font_id, font);
            Some(font_id)
        } else {
            None
        }
    }
}

// ------------
// UTF-8 glyphs
// ------------

// @XXX This should be a compile time function, but Rust is not good enough...
fn build_ascii_string() -> Vec<char> {
    let mut s = [0u8; 128 - 32];
    for i in 32..128 { s[i-32] = i as u8; }

    std::str::from_utf8(&s)
        .unwrap()
        .to_string()
        .chars()
        .collect()
}

// @XXX This should be a compile time function, but Rust is not good enough...
fn build_latin1_string() -> Vec<char> {
    const COUNT: usize = 2 * ((0xc0 - 0xa0) + (0xc0 - 0x80));
    let mut s = [0u8; COUNT];

    let mut p = 0;

    for i in 0xa0..0xc0 {
        s[p] = 0xc2;
        s[p+1] = i;
        p += 2;
    }

    for i in 0x80..0xc0 {
        s[p] = 0xc3;
        s[p+1] = i;
        p += 2;
    }

    std::str::from_utf8(&s)
        .unwrap()
        .to_string()
        .chars()
        .collect()
}

// @XXX This should be a compile time function, but Rust is not good enough...
fn build_ascii_and_latin1_string() -> Vec<char> {
    let mut r = build_ascii_string();
    r.append(&mut build_latin1_string());
    r
}
