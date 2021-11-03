use crate::app::ImDraw;
use crate::linalg::Vec2i;
use super::pieces::PieceType;

pub const PLAYFIELD_VISIBLE_HEIGHT : i32 = 20;

#[derive(Clone, Debug)]
pub struct Playfield {
    pub grid_size: Vec2i, // @Refactor use Vec2<u8>
    pub blocks: Vec<bool>, // @Refactor don't use Vec (and Vec<bool> is worse)
    pub block_types: Vec<PieceType>, // @Refactor don't use Vec

    // Render variables
    // @Refactor rendering information struct
    pub pos: Vec2i,
    pub has_grid: bool, // @XXX this is probably better be where pixel_scale is
}

impl Playfield {
    // @TODO remove pos
    pub fn new(pos: Vec2i, grid_size: Vec2i, has_grid: bool) -> Self {
        let mut blocks = Vec::new();
        blocks.resize((grid_size.x * grid_size.y) as usize, false);

        let mut block_types = Vec::new();
        block_types.resize((grid_size.x * grid_size.y) as usize, PieceType::S);

        Self {
            pos,
            grid_size,
            has_grid,

            blocks,
            block_types,
        }
    }

    // @TODO use row, col, instead of x, y
    pub fn block(&self, x: i32, y: i32) -> Option<PieceType> {
        if x < 0 || x >= self.grid_size.x { return Some(PieceType::S); }
        if y < 0 || y >= self.grid_size.y { return Some(PieceType::S); }

        let pos = y * self.grid_size.x + x;
        let pos = pos as usize;

        if self.blocks[pos] {
            Some(self.block_types[pos])
        } else {
            None
        }
    }

    pub fn set_block(&mut self, x: i32, y: i32, piece_type: PieceType) {
        assert!(x >= 0 && x < self.grid_size.x);
        assert!(y >= 0 && y < self.grid_size.y);

        let pos = y * self.grid_size.x + x;
        let pos = pos as usize;

        self.blocks[pos] = true;
        self.block_types[pos] = piece_type;
    }

    pub fn reset_block(&mut self, x: i32, y: i32) {
        assert!(x >= 0 && x < self.grid_size.x);
        assert!(y < self.grid_size.y);

        let pos = y * self.grid_size.x + x;
        let pos = pos as usize;

        self.blocks[pos] = false;
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
                    .fold(0, |acc, &x| if x { acc + 1 } else { acc });

                if cnt == self.grid_size.x {
                    lines_to_clear[total_lines_to_clear as usize] = i as u8;
                    total_lines_to_clear += 1;
                }
            });

        (total_lines_to_clear, lines_to_clear)
    }

    pub fn clear_lines_naive(&mut self) -> u8 {
        let mut last_free_line = 0;
        (0..self.grid_size.y).for_each(|current_line| {
            let line_start = (current_line * self.grid_size.x) as usize;
            let line_end   = ((current_line + 1) * self.grid_size.x) as usize;
            let cnt = self.blocks[line_start..line_end]
                .iter()
                .fold(0, |acc, &x| if x { acc + 1 } else { acc });

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

                    // 
                    let (split_left, split_right) = self.block_types.split_at_mut(last_free_line_end);

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
            self.blocks[empty_block_start..empty_block_end].fill(false);
        }

        (self.grid_size.y - last_free_line) as u8
    }
}

impl ImDraw for Playfield {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        imgui::TreeNode::new(im_str2!(label)).build(ui, || {
            let id = ui.push_id(label);

            self.pos.imdraw("pos", ui);
            self.grid_size.imdraw("grid_size", ui);
            self.has_grid.imdraw("has_grid", ui);

            imgui::TreeNode::new(im_str2!("blocks")).build(ui, || {
                for i in (0..self.grid_size.y).rev() {
                    ui.text(format!("{:>2}", i));
                    ui.same_line(0.0);

                    for j in 0..self.grid_size.x-1 {
                        let index = (i * self.grid_size.x + j) as usize;
                        ui.checkbox(im_str2!(""), &mut self.blocks[index]);
                        ui.same_line(0.0);
                    }

                    let index = ((i + 1) * self.grid_size.x - 1) as usize;
                    ui.checkbox(im_str2!(""), &mut self.blocks[index]);
                }
            });

            id.pop(ui);
        });
    }
}

// Network
use crate::game::network;

impl Playfield {
    pub fn from_network(
        net_instance: network::NetworkedPlayfield,
        pos: Vec2i,
        has_grid: bool
    ) -> Self {
        let network::NetworkedPlayfield { grid_size, blocks, block_types } = net_instance;

        Self {
            pos,
            grid_size,
            has_grid,

            blocks,
            block_types,
        }
    }

    pub fn to_network(&self) -> network::NetworkedPlayfield {
        network::NetworkedPlayfield {
            grid_size: self.grid_size,
            blocks: self.blocks.clone(),
            block_types: self.block_types.clone(),
        }
    }

    pub fn update_from_network(
        &mut self,
        net_instance: network::NetworkedPlayfield
    ) {
        let network::NetworkedPlayfield { grid_size, blocks, block_types } = net_instance;
        self.grid_size = grid_size;
        self.blocks = blocks;
        self.block_types = block_types;
    }
}
