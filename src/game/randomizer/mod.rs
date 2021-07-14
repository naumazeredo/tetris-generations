mod sequential;
mod fullrandom;
mod random7bag;

use crate::enum_dispatch::*;
use crate::app::ImDraw;
use super::pieces::PieceType;

pub use fullrandom::RandomizerFullRandom;
pub use random7bag::Randomizer7Bag;
pub use sequential::RandomizerSequential;

#[derive(Copy, Clone, Debug, ImDraw)]
pub enum RandomizerType {
    Sequential,
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

impl RandomizerType {
    pub fn build(self, seed: u64) -> Randomizer {
        match self {
            RandomizerType::FullRandom => Randomizer::RandomizerFullRandom(RandomizerFullRandom::new(seed)),
            RandomizerType::Random7Bag => Randomizer::Randomizer7Bag(Randomizer7Bag::new(seed)),
            RandomizerType::Sequential => Randomizer::RandomizerSequential(RandomizerSequential::new()),
            _ => unimplemented!("Randomizer type not yet supported"),
        }
    }
}

#[enum_dispatch]
pub trait RandomizerTrait {
    fn reset(&mut self);
    fn next_piece(&mut self) -> PieceType;
}

#[enum_dispatch(RandomizerTrait)]
#[derive(Clone, Debug, ImDraw)]
pub enum Randomizer {
    RandomizerSequential,
    RandomizerFullRandom,
    Randomizer7Bag,
}

impl_imdraw_todo!(rand_pcg::Pcg32);
