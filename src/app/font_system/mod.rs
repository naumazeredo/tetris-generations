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

mod font;
mod packing;

pub use font::*;

use std::collections::BTreeMap;

use crate::{
    app::imgui_wrapper::ImDraw,
    linalg::Vec2i,
};

#[derive(ImDraw, Default)]
pub(in crate::app) struct FontSystem {
    pub(super) fonts: BTreeMap<FontRef, Font>,
}

// ----------
// Structures
// ----------

#[derive(Copy, Clone, Debug, ImDraw)]
pub(in crate::app) struct Metrics {
    pub minx: i32,
    pub maxy: i32,
    pub w: i32,
    pub h: i32,
    pub advance: i32
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

#[derive(Copy, Clone, Debug, ImDraw)]
pub(in crate::app) struct CharData {
    pub pos: (u32, u32),
    pub metrics: Metrics,
}

impl CharData {
    pub(in crate::app) fn get_uvs(&self) -> (Vec2i, Vec2i) {
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
