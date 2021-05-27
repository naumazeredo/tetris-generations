use crate::app::ImDraw;
use crate::linalg::Vec2i;

pub const PLAYFIELD_VISIBLE_HEIGHT : i32 = 20;

#[derive(Clone, Debug, ImDraw)]
pub struct Playfield {
    pub pos: Vec2i,
    pub grid_size: Vec2i,

    // @Refactor vec of bools are bad!
    pub blocks: Vec<bool>,
}

impl Playfield {
    pub fn new(pos: Vec2i, grid_size: Vec2i) -> Self {
        let mut blocks = Vec::new();
        blocks.resize((grid_size.x * grid_size.y) as usize, false);

        Self {
            pos,
            grid_size,
            blocks,
        }
    }

    pub fn block(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.grid_size.x { return true; }
        if y < 0 || y >= self.grid_size.y { return true; }

        let pos = y * self.grid_size.x + x;
        if pos as usize >= self.blocks.len() {
            return true;
        }

        self.blocks[pos as usize]
    }

    pub fn set_block(&mut self, x: i32, y: i32, set: bool) {
        assert!(x >= 0 && x < self.grid_size.x);
        assert!(y < self.grid_size.y);

        // The pieces spawn on negative y, we have try to place a block near the spawn,
        // so we just ignore and not assert
        if y < 0 { return; }

        let pos = y * self.grid_size.x + x;
        self.blocks[pos as usize] = set;
    }
}
