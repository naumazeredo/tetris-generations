use crate::app::ImDraw;
use crate::game::pieces::PieceVariant;
use super::{Randomizer, RandomizerTrait};

#[derive(Clone, Debug, ImDraw)]
pub struct RandomizerDefinedSequence {
    current: u32,
    sequence: Vec<PieceVariant>,
}

impl RandomizerDefinedSequence {
    pub fn new(sequence: Vec<PieceVariant>) -> Randomizer {
        assert!(sequence.len() > 0);
        let current = sequence.len() as u32;
        Randomizer::RandomizerDefinedSequence(
            Self {
                sequence,
                current,
            }
        )
    }
}

impl RandomizerTrait for RandomizerDefinedSequence {
    fn reset(&mut self) {
        self.current = self.sequence.len() as u32;
    }

    fn next_piece(&mut self) -> PieceVariant {
        self.current += 1;
        if self.current >= self.sequence.len() as u32 { self.current = 0; }
        self.sequence[self.current as usize]
    }

    fn seed(&self) -> u64 { 0 }
}
