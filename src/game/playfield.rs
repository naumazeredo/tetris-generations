use crate::app::ImDraw;
use crate::linalg::Vec2i;
use super::pieces::PieceVariant;

pub const PLAYFIELD_VISIBLE_HEIGHT : u8 = 20;

#[derive(Copy, Clone, Debug)]
pub enum BlockType {
    Empty,
    Piece(PieceVariant),
}

#[derive(Clone, Debug)]
pub struct Playfield {
    pub grid_size: Vec2i, // @Refactor use Vec2<u8>
    pub visible_height: u8,
    pub blocks: Vec<BlockType>, // @Refactor don't use Vec
}

impl Playfield {
    pub fn new(grid_size: Vec2i, visible_height: u8) -> Self {
        assert!(grid_size.x > 0);
        assert!(grid_size.y > 0);

        let mut blocks = Vec::new();
        blocks.resize((grid_size.x * grid_size.y) as usize, BlockType::Empty);

        Self {
            grid_size,
            visible_height,
            blocks,
        }
    }

    // @TODO use row, col, instead of x, y
    pub fn block(&self, x: i32, y: i32) -> Option<PieceVariant> {
        if x < 0 || x >= self.grid_size.x { return Some(PieceVariant::S); }
        if y < 0 || y >= self.grid_size.y { return Some(PieceVariant::S); }

        let pos = y * self.grid_size.x + x;
        let pos = pos as usize;

        if let BlockType::Piece(piece_type) = self.blocks[pos] {
            Some(piece_type)
        } else {
            None
        }
    }

    pub fn set_block(&mut self, x: i32, y: i32, piece_type: PieceVariant) {
        assert!(x >= 0 && x < self.grid_size.x);
        assert!(y >= 0 && y < self.grid_size.y);

        let pos = y * self.grid_size.x + x;
        let pos = pos as usize;

        self.blocks[pos] = BlockType::Piece(piece_type);
    }

    pub fn reset_block(&mut self, x: i32, y: i32) {
        assert!(x >= 0 && x < self.grid_size.x);
        assert!(y < self.grid_size.y);

        let pos = y * self.grid_size.x + x;
        let pos = pos as usize;

        self.blocks[pos] = BlockType::Empty;
    }

    pub fn get_lines_to_clear(&self) -> (u8, [u8; 4]) {
        // @Maybe refactor to use Vec::chunks
        // self.blocks.chunks(self.grid_size.x).iter()

        let mut total_lines_to_clear = 0u8;
        let mut lines_to_clear = [0u8; 4];

        (0..self.grid_size.y)
            .for_each(|i| {
                let line_start = (i * self.grid_size.x) as usize;
                let line_end   = ((i+1) * self.grid_size.x) as usize;
                let cnt = self.blocks[line_start..line_end]
                    .iter()
                    .fold(0, |acc, &x| if let BlockType::Piece(_) = x { acc + 1 } else { acc });

                if cnt == self.grid_size.x {
                    lines_to_clear[total_lines_to_clear as usize] = i as u8;
                    total_lines_to_clear += 1;
                }
            });

        (total_lines_to_clear, lines_to_clear)
    }

    pub fn try_clear_lines_naive(&mut self) -> bool {
        let mut last_free_line = 0;
        (0..self.grid_size.y).for_each(|current_line| {
            let line_start = (current_line * self.grid_size.x) as usize;
            let line_end   = ((current_line + 1) * self.grid_size.x) as usize;
            let cnt = self.blocks[line_start..line_end]
                .iter()
                .fold(0, |acc, &x| if let BlockType::Piece(_) = x { acc + 1 } else { acc });

            if cnt != self.grid_size.x {
                if last_free_line != current_line {
                    let last_free_line_start = (last_free_line * self.grid_size.x) as usize;
                    let last_free_line_end   = ((last_free_line + 1) * self.grid_size.x) as usize;

                    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.swap_with_slice
                    let (split_left, split_right) = self.blocks.split_at_mut(last_free_line_end);

                    let right_current_line_start = line_start - last_free_line_end;
                    let right_current_line_end   = line_end - last_free_line_end;

                    split_left[last_free_line_start..last_free_line_end].swap_with_slice(
                        &mut split_right[right_current_line_start..right_current_line_end]
                    );
                }
                last_free_line += 1;
            }
        });

        if last_free_line != self.grid_size.y {
            let empty_block_start = (last_free_line * self.grid_size.x) as usize;
            let empty_block_end   = (self.grid_size.y * self.grid_size.x) as usize;
            self.blocks[empty_block_start..empty_block_end].fill(BlockType::Empty);
        }

        last_free_line != self.grid_size.y
    }
}

impl ImDraw for Playfield {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        imgui::TreeNode::new(label).build(ui, || {
            let id = ui.push_id(label);
            self.grid_size.imdraw("grid_size", ui);

            /*
            imgui::TreeNode::new("blocks").build(ui, || {
                for i in (0..self.grid_size.y).rev() {
                    ui.text(format!("{:>2}", i));
                    ui.same_line(0.0);

                    for j in 0..self.grid_size.x-1 {
                        let index = (i * self.grid_size.x + j) as usize;
                        ui.checkbox("", &mut self.blocks[index]);
                        ui.same_line(0.0);
                    }

                    let index = ((i + 1) * self.grid_size.x - 1) as usize;
                    ui.checkbox("", &mut self.blocks[index]);
                }
            });
            */

            id.pop();
        });
    }
}
