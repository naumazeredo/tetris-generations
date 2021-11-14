use crate::app::ImDraw;
use crate::game::pieces::{
    PieceType,
    PIECES
};
use super::RandomizerTrait;

#[derive(Clone, Debug, ImDraw)]
pub struct RandomizerSequential {
    current: u8
}

impl RandomizerSequential {
    pub fn new() -> Self {
        Self {
            current: 7,
        }
    }
}

impl RandomizerTrait for RandomizerSequential {
    fn reset(&mut self) {
        self.current = 7;
    }

    fn next_piece(&mut self) -> PieceType {
        self.current += 1;
        if self.current >= 7 { self.current = 0; }
        PIECES[self.current as usize]
    }

    fn seed(&self) -> u64 { 0 }
}
