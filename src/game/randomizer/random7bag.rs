// @TODO abstract PCG
use crate::rand_core::RngCore;

use crate::game::piece::{
    PieceType,
    PIECES
};

pub struct Randomizer7Bag {
    rng: rand_pcg::Pcg32,
    sequence: [u8; 7],
    index: u8,
}

impl Randomizer7Bag {
    pub fn new() -> Self {
        Self {
            rng: rand_pcg::Pcg32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7),
            sequence: [0, 1, 2, 3, 4, 5, 6],
            index: 7,
        }
    }

    pub fn reset(&mut self) {
        self.index = 7;
    }

    pub fn next_piece(&mut self) -> PieceType {
        if self.index == 7 {
            for _ in 0..10 {
                let i = (self.rng.next_u32() % 7) as usize;
                let j = (self.rng.next_u32() % 7) as usize;
                self.sequence.swap(i, j);
            }
            self.index = 0;
        }

        let piece_id = self.sequence[self.index as usize] as usize;
        self.index += 1;
        PIECES[piece_id]
    }
}

//impl_imdraw_todo!(rand_pcg::Pcg32);

/*
impl ImDraw for [u8; 7] {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        imgui::TreeNode::new(im_str2!(label)).build(ui, || {
            for i in 0..7 {
                self[i].imdraw(&format!("[{}]", i), ui);
            }
        });
    }
}
*/
