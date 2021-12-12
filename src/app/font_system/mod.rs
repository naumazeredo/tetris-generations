/* Usage

// construction

let font_id = app.bake_font("assets/fonts/Monocons.ttf").unwrap();

// render

app.queue_draw_text(
    "Hello world",
    font_id,
    &Transform {
        pos: Vec2 { x: 200., y: 200. },
        rot: 0.,
        layer: 0,
    },
    32.,
    WHITE
);
*/

mod char_data;
mod font;
mod packing;

pub use font::*;

use std::collections::BTreeMap;

use crate::app::imgui_wrapper::ImDraw;

pub(in crate::app) const FONT_SCALE: u16 = 72;

#[derive(ImDraw)]
pub(in crate::app) struct FontSystem {
    pub(super) default_font_id: FontId,
    // @TODO use a method to abstract fonts
    pub(super) fonts: BTreeMap<FontId, Font>,
}

impl FontSystem {
    pub(in crate::app) fn new(ttf_context: &sdl2::ttf::Sdl2TtfContext) -> Self {
        let (default_font_id, default_font) = bake_font("assets/fonts/Fami-Sans-Bold.ttf", ttf_context).unwrap();

        let mut fonts = BTreeMap::new();
        fonts.insert(default_font_id, default_font);

        Self {
            default_font_id,
            fonts,
        }
    }
}
