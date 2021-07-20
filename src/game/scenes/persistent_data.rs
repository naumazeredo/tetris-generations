use crate::app::*;
use crate::linalg::*;
use crate::State;
use crate::game::input::get_default_input_mapping;

// Persistent Data
#[derive(ImDraw)]
pub struct Sprites {
    pub blank: Sprite,
    pub block: Sprite,
}

#[derive(ImDraw)]
pub struct PersistentData {
    pub input_mapping: InputMapping,
    pub font: FontRef,
    pub sprites: Sprites,
    pub pixel_scale: u8,
}

impl PersistentData {
    pub fn new(app: &mut App<'_, State>) -> Self {
        // Fonts
        let font = app.bake_font("assets/fonts/Monocons.ttf").unwrap();

        // Sprites
        let build_sprite = |tex, x, y, w, h| {
            Sprite {
                texture: tex,
                texture_flip: TextureFlip::NO,
                uvs: (Vec2i { x, y }, Vec2i { x: w + x, y: h + y }),
                pivot: Vec2 { x: 0.0, y: 0.0 },
                size: Vec2 { x:  w as f32, y: h as f32 },
            }
        };

        let blank_texture = app.get_texture("assets/gfx/blank.png");
        let blank = build_sprite(blank_texture, 0, 0, 1, 1);

        let block_texture = app.get_texture("assets/gfx/block-soft.png");
        let block = build_sprite(block_texture, 0, 0, 8, 8);

        // input
        let input_mapping = get_default_input_mapping();

        // pixel scaling
        let pixel_scale = 5;

        Self {
            input_mapping,
            font,
            sprites: Sprites {
                blank,
                block,
            },
            pixel_scale,
        }
    }
}
