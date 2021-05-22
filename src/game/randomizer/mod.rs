mod sequential;
mod fullrandom;
mod random7bag;

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
            RandomizerType::FullRandom => Randomizer::FullRandom(RandomizerFullRandom::new()),
            RandomizerType::Random7Bag => Randomizer::Random7Bag(Randomizer7Bag::new()),
            RandomizerType::Sequential => Randomizer::Sequential(RandomizerSequential::new()),
            _ => unimplemented!("Randomizer type not yet supported"),
        }
    }
}

pub enum Randomizer {
    Sequential(RandomizerSequential),
    FullRandom(RandomizerFullRandom),
    Random7Bag(Randomizer7Bag),
}

// @Maybe macro this?
impl Randomizer {
    //pub fn set_seed(&mut self);

    pub fn reset(&mut self) {
        match self {
            Randomizer::Sequential(x) => x.reset(),
            Randomizer::FullRandom(x) => x.reset(),
            Randomizer::Random7Bag(x) => x.reset(),
        }
    }

    pub fn next_piece(&mut self) -> PieceType {
        match self {
            Randomizer::Sequential(x) => x.next_piece(),
            Randomizer::FullRandom(x) => x.next_piece(),
            Randomizer::Random7Bag(x) => x.next_piece(),
        }
    }
}

impl_imdraw_todo!(RandomizerType);
impl_imdraw_todo!(Randomizer);

impl_imdraw_todo!(Randomizer7Bag);
