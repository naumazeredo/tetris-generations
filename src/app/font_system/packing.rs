// ------------
// Font packing
// ------------

use std::collections::BTreeMap;
use super::char_data::{CharData, Metrics};

pub(super) fn pack_font<'a>(
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
                    // @TODO count glyphs not found?
                    // @TODO logging
                    //println!("glyph |{}| not found", glyph);
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
        let mut cur_x = spacing + 1; // blit white pixel at (0, 0)
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

        // Blit white pixel
        packed_surface.fill_rect(
            sdl2::rect::Rect::new(0, 0, 0, 0),
            sdl2::pixels::Color::WHITE
        ).unwrap();

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
