use crate::app::ImDraw;
use crate::linalg::Vec2i;

#[derive(ImDraw)]
pub struct Playfield {
    pub pos: Vec2i,
    pub size: Vec2i,

    // @Refactor vec of bools are bad!
    pub blocks: Vec<bool>,
}

impl Playfield {
    pub fn block(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.size.x { return true; }
        if y >= self.size.y { return true; }

        // The pieces spawn on negative y
        if y < 0 { return false; }

        let pos = y * self.size.x + x;
        if pos as usize >= self.blocks.len() {
            return true;
        }

        self.blocks[pos as usize]
    }

    pub fn set_block(&mut self, x: i32, y: i32, set: bool) {
        assert!(x >= 0 && x < self.size.x);
        assert!(y < self.size.y);

        // The pieces spawn on negative y, we have try to place a block near the spawn,
        // so we just ignore and not assert
        if y < 0 { return; }

        let pos = y * self.size.x + x;
        self.blocks[pos as usize] = set;
    }
}
