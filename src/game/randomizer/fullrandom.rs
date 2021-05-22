// @TODO abstract PCG
use crate::rand_core::RngCore;

use crate::game::piece::{
    PieceType,
    PIECES
};

pub struct RandomizerFullRandom {
    rng: rand_pcg::Pcg32,
}

impl RandomizerFullRandom {
    pub fn new() -> Self {
        Self {
            rng: rand_pcg::Pcg32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7),
        }
    }

    pub fn reset(&mut self) {}

    pub fn next_piece(&mut self) -> PieceType {
        let piece_id = (self.rng.next_u32() % 7) as usize;
        PIECES[piece_id]
    }
}
