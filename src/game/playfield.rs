use crate::app::ImDraw;
use crate::linalg::Vec2i;

pub const PLAYFIELD_VISIBLE_HEIGHT : i32 = 20;

#[derive(Clone, Debug)]
pub struct Playfield {
    pub pos: Vec2i,
    pub grid_size: Vec2i, // @Refactor use Vec2<u8>

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

    pub fn has_clear_lines(&self) -> Option<Vec<u8>> {
        // @Maybe refactor to use Vec::chunks
        // self.blocks.chunks(self.grid_size.x).iter()

        let lines = (0..self.grid_size.y)
            .filter_map(|i| {
                let line_start = (i * self.grid_size.x) as usize;
                let line_end   = ((i+1) * self.grid_size.x) as usize;
                let cnt = self.blocks[line_start..line_end]
                    .iter()
                    .fold(0, |acc, &x| if x { acc + 1 } else { acc });
                if cnt == self.grid_size.x {
                    return Some(i as u8);
                } else {
                    return None;
                }
            })
            .collect::<Vec<u8>>();

        if lines.len() == 0 {
            None
        } else {
            Some(lines)
        }
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
