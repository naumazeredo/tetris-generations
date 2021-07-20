use std::collections::BTreeMap;
use std::path::Path;

use crate::{
    impl_imdraw_todo,
    app::{
        App,
        imgui_wrapper::ImDraw,
        renderer::{
            Texture,
            load_texture_from_surface,
        },
    },
    linalg::Vec2,
    utils::string_ref::StringRef,
};

use super::{
    CharData,
    packing::pack_font,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FontRef(StringRef);
impl FontRef {
    fn new(s: String) -> Self {
        Self(StringRef::new(s))
    }
}

impl std::fmt::Display for FontRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FontRef {}", self.0)
    }
}

impl_imdraw_todo!(FontRef);

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

        let scale = 64;
        match ttf_context.load_font(path.as_ref(), scale) {
            Ok(font) => {
                let glyphs = build_ascii_and_latin1_string();
                let (packed_surface, mapping) = pack_font(font, glyphs, scale as u32, 1, true);

                // @TODO save packed font to file?
                //packed_surface.save_bmp("tmp/font.bmp").unwrap();
                println!("[font bake] Packing complete: {}", path.as_ref().display());

                let texture = load_texture_from_surface(packed_surface);

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

impl<S> App<'_, S>{
    // @TODO return Result
    pub fn bake_font<P: AsRef<Path>>(&mut self, path: P) -> Option<FontRef> {
        let font_ref = FontRef::new(path.as_ref().to_string_lossy().to_string());

        // @Check if it's already baked?

        match Font::bake(path, &self.sdl_context.ttf_context) {
            Some(font) => {
                self.font_system.fonts.insert(font_ref, font);
                Some(font_ref)
            }

            None => None,
        }
    }

    // @TODO return offset. Some glyphs can have negative minx or miny
    pub fn calculate_draw_text_size(
        &mut self,
        text: &str,
        font_ref: FontRef,
        font_size: f32,
    ) -> Vec2 {
        let font = self.font_system.fonts.get(&font_ref).unwrap();

        let mut pos = Vec2::new();
        let mut max_size = Vec2::new();

        for ch in text.chars() {
            if let Some(char_data) = font.get_char_data(ch) {
                let scale = font_size / 64.;
                let char_top_left = Vec2 {
                    x:  char_data.metrics.minx as f32 * scale,
                    y: -char_data.metrics.maxy as f32 * scale
                };
                let size = Vec2 {
                    x: char_data.metrics.w as f32 * scale,
                    y: char_data.metrics.h as f32 * scale
                };

                let advance = char_data.metrics.advance as f32 * scale;
                pos.x += advance;

                max_size.x = max_size.x.max(pos.x);
            }
        }

        max_size
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
