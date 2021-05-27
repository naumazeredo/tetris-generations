/* Usage

// construction

let font = app.bake_font("assets/fonts/Monocons.ttf").unwrap();

// render

app.queue_draw_text(
    "Hello world",
    &self.font,
    &Transform {
        pos: Vec2 { x: 200., y: 200. },
        rot: 0.,
        layer: 0,
    },
    32.,
    WHITE
);
*/

use std::collections::BTreeMap;
use std::path::Path;

use crate::{
    app::{
        App,
        imgui_wrapper::ImDraw,
    },
    linalg::{Vec2, Vec2i},
    transform::Transform,
};

use super::{
    color::Color,
    sprite::Sprite,
    texture::{
        Texture,
        TextureFlip,
        load_texture_from_surface,
    },
};

#[derive(Clone, Debug, ImDraw)]
pub struct Font {
    mapping: BTreeMap<char, CharData>,
    texture: Texture,
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

    fn get_char_data(&self, ch: char) -> Option<&CharData> {
        self.mapping.get(&ch)
    }
}

impl<S> App<'_, S>{
    pub fn bake_font<P: AsRef<Path>>(&self, path: P) -> Option<Font> {
        Font::bake(path, &self.sdl_context.ttf_context)
    }

    pub fn queue_draw_text(
        &mut self,
        //program: Program,
        text: &str,
        font: &Font,
        transform: &Transform,
        font_size: f32,
        color: Color,
    ) {
        let mut pos = Vec2::new();

        for ch in text.chars() {
            if let Some(char_data) = font.get_char_data(ch) {
                let uvs = char_data.get_uvs();

                let scale = font_size / 64.;
                let char_top_left = Vec2 {
                    x:  char_data.metrics.minx as f32 * scale,
                    y: -char_data.metrics.maxy as f32 * scale
                };
                let size = Vec2 {
                    x: char_data.metrics.w as f32 * scale,
                    y: char_data.metrics.h as f32 * scale
                };

                self.queue_draw_sprite(
                    transform,
                    &Sprite {
                        texture: font.texture,
                        texture_flip: TextureFlip::NO,
                        uvs,
                        pivot: - (pos + char_top_left),
                        size,
                    },
                    color,
                );

                let advance = char_data.metrics.advance as f32 * scale;
                pos.x += advance;
            }
        }
    }
}

// ------------
// Font packing
// ------------

fn pack_font<'a>(
    font: sdl2::ttf::Font,
    glyphs: Vec<char>,
    scale: u32,
    spacing: u32,
    reorder: bool
) -> (sdl2::surface::Surface<'a>, BTreeMap<char, CharData>) {
    // Filter to only valid glyphs
    let glyphs = glyphs.into_iter()
        .filter(|&glyph| {
            match font.find_glyph(glyph) {
                Some(_) => true,
                None => {
                    println!("glyph |{}| not found", glyph);
                    false
                }
            }
        })
        .collect::<Vec<_>>();

    let count = glyphs.len();

    // Get metrics and surfaces
    let mut metrics = Vec::new();
    let mut surfaces = Vec::new();
    metrics.reserve(count);
    surfaces.reserve(count);

    for &glyph in glyphs.iter() {
        // Get metrics info
        let glyph_metrics = font.find_glyph_metrics(glyph).unwrap();
        metrics.push(Metrics::from(glyph_metrics));

        // Render glyph
        // The character can be blank, so we need to get surfaces as optionals
        let render = font
            .render_char(glyph)
            .blended(sdl2::pixels::Color::RGBA(255, 255, 255, 255));

        match render {
            Ok(surface) => surfaces.push(Some(surface)),
            Err(_) => surfaces.push(None),
        }
    }

    // Sort by decreasing height and decreasing width
    let mut indexes : Vec<usize> = (0..count).collect();

    if reorder {
        indexes.sort_by(|&a, &b| {
            let a = &metrics[a];
            let b = &metrics[b];
            (b.h, b.w).cmp(&(a.h, a.w))
        });
    }

    let first = indexes[0];
    let indexes = indexes.iter();

    // Pixel format should match between surfaces. Maybe we need to convert all to the same format
    let pixel_format = surfaces[first].as_ref().unwrap().pixel_format_enum();

    // Exponentially scale size in case it doesn't fit
    let ascent = font.ascent();
    let mut size = 256u32;
    loop {
        let mut cur_y = spacing;
        let mut cur_x = spacing;
        let mut next_y;

        if reorder {
            next_y = cur_y + spacing + metrics[first].h as u32;
        } else {
            next_y = scale;
        }

        let pos : Vec<(u32, u32)> = indexes.clone()
            .map(|&index| {
                let metrics = &metrics[index];
                let width = metrics.w as u32;
                let height = metrics.h as u32;

                if cur_x + width > size {
                    cur_x = spacing;
                    cur_y = next_y;

                    if reorder {
                        next_y += height + spacing;
                    } else {
                        next_y += scale;
                    }
                }

                let ret = (cur_x, cur_y);
                cur_x += width + spacing;
                ret
            })
            .collect();

        if next_y > size {
            size *= 2;
            continue;
        }

        // Blit surfaces into the atlas
        let mut packed_mapping = BTreeMap::new();

        let mut packed_surface = sdl2::surface::Surface::new(size, size, pixel_format).unwrap();
        indexes.zip(pos).for_each(|(&index, (pos_x, pos_y))| {
            let metrics = metrics[index].clone();
            if let Some(surface) = surfaces[index].as_ref() {

                let src_rect = sdl2::rect::Rect::new(
                    std::cmp::max(metrics.minx, 0), ascent - metrics.maxy,
                    metrics.w as u32, metrics.h as u32
                );

                let dst_rect = sdl2::rect::Rect::new(
                    pos_x as i32, pos_y as i32,
                    metrics.w as u32, metrics.h as u32
                );

                surface.blit(Some(src_rect), &mut packed_surface, Some(dst_rect)).unwrap();
            }

            packed_mapping.insert(glyphs[index], CharData {
                pos: (pos_x, pos_y),
                metrics
            });
        });

        return (packed_surface, packed_mapping);
    }
}

// ----------
// Structures
// ----------

#[derive(Copy, Clone, Debug, ImDraw)]
struct Metrics {
    minx: i32,
    maxy: i32,
    w: i32,
    h: i32,
    advance: i32
}

impl From<sdl2::ttf::GlyphMetrics> for Metrics {
    fn from(metrics: sdl2::ttf::GlyphMetrics) -> Self {
        Metrics {
            minx: metrics.minx,
            maxy: metrics.maxy,
            w: metrics.maxx - metrics.minx,
            h: metrics.maxy - metrics.miny,
            advance: metrics.advance,
        }
    }
}

#[derive(Clone, Debug, ImDraw)]
struct CharData {
    pos: (u32, u32),
    metrics: Metrics,
}

impl CharData {
    fn get_uvs(&self) -> (Vec2i, Vec2i) {
        (
            Vec2i {
                x: self.pos.0 as i32,
                y: self.pos.1 as i32
            },
            Vec2i {
                x: self.pos.0 as i32 + self.metrics.w,
                y: self.pos.1 as i32 + self.metrics.h,
            }
        )
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
