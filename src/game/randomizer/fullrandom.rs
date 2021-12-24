// @TODO abstract PCG
use rand_core::RngCore;

use crate::app::ImDraw;
use crate::game::pieces::{PIECES, PieceVariant};
use super::RandomizerTrait;

#[derive(Clone, Debug, ImDraw)]
pub struct RandomizerFullRandom {
    rng: rand_pcg::Pcg32,
    seed: u64,
}

impl RandomizerFullRandom {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: rand_pcg::Pcg32::new(seed, 0xa02bdbf7bb3c0a7),
            seed,
        }
    }
}

impl RandomizerTrait for RandomizerFullRandom {
    fn reset(&mut self) {}

    fn next_piece(&mut self) -> PieceVariant {
        let piece_id = (self.rng.next_u32() % 7) as usize;
        PIECES[piece_id]
    }

    fn seed(&self) -> u64 { self.seed }
}
