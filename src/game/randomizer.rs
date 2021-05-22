use crate::app::ImDraw;
use super::piece::{
    PieceType,
    PIECES,
};

// @TODO abstract PCG
use crate::rand_core::RngCore;

#[derive(Copy, Clone, Debug)]
pub enum RandomizerType {
    FullRandom,

    // https://tetris.fandom.com/wiki/Random_Generator
    Random7Bag, // Random Generator, with a bag of all 7 pieces
    TGMACE,     // Random Generator + not dealing SZO initially

    // https://tetris.fandom.com/wiki/TGM_randomizer
    TGM1, // 4 history, 4 tries, 2 variants: history with ZZZZ or ZZSS
    TGM,  // 4 history, 6 tries
    //TGMCustom { history: u8, retries: u8, },

    // @TODO TGM3 35-piece bag?
}

impl_imdraw_todo!(RandomizerType);

pub trait Randomizer {
    // TODO receive seed
    fn new() -> Self;
    fn reset(&mut self);
    fn next_piece(&mut self) -> PieceType;
}

pub struct RandomizerFullRandom {
    rng: rand_pcg::Pcg32,
}

impl_imdraw_todo!(RandomizerFullRandom);

impl Randomizer for RandomizerFullRandom {
    fn new() -> Self {
        Self {
            rng: rand_pcg::Pcg32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7),
        }
    }

    fn reset(&mut self) {}

    fn next_piece(&mut self) -> PieceType {
        let piece_id = (self.rng.next_u32() % 7) as usize;
        PIECES[piece_id]
    }
}

#[derive(ImDraw)]
pub struct RandomizerRandomGenerator {
    rng: rand_pcg::Pcg32,
    sequence: [u8; 7],
    index: u8,
}

impl Randomizer for RandomizerRandomGenerator {
    fn new() -> Self {
        Self {
            rng: rand_pcg::Pcg32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7),
            sequence: [0, 1, 2, 3, 4, 5, 6],
            index: 7,
        }
    }

    fn reset(&mut self) {
        self.index = 7;
    }

    fn next_piece(&mut self) -> PieceType {
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

impl_imdraw_todo!(rand_pcg::Pcg32);

//
impl ImDraw for [u8; 7] {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        imgui::TreeNode::new(im_str2!(label)).build(ui, || {
            for i in 0..7 {
                self[i].imdraw(&format!("[{}]", i), ui);
            }
        });
    }
}
