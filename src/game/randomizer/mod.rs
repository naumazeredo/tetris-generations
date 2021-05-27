mod sequential;
mod fullrandom;
mod random7bag;

use crate::enum_dispatch::*;
use crate::app::ImDraw;
use super::piece::PieceType;

pub use fullrandom::RandomizerFullRandom;
pub use random7bag::Randomizer7Bag;
pub use sequential::RandomizerSequential;

#[derive(Copy, Clone, Debug)]
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

impl From<RandomizerType> for Randomizer {
    fn from(ty: RandomizerType) -> Self {
        match ty {
            RandomizerType::FullRandom => Randomizer::RandomizerFullRandom(RandomizerFullRandom::new()),
            RandomizerType::Random7Bag => Randomizer::Randomizer7Bag(Randomizer7Bag::new()),
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
#[derive(Clone, Debug)]
pub enum Randomizer {
    RandomizerSequential,
    RandomizerFullRandom,
    Randomizer7Bag,
}

impl_imdraw_todo!(RandomizerType);
impl_imdraw_todo!(Randomizer);

impl_imdraw_todo!(Randomizer7Bag);
