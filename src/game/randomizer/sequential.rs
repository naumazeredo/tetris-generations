use crate::game::piece::{
    PieceType,
    PIECES
};

pub struct RandomizerSequential {
    current: u8
}

impl RandomizerSequential {
    pub fn new() -> Self {
        Self {
            current: 7,
        }
    }

    pub fn reset(&mut self) {
        self.current = 7;
    }

    pub fn next_piece(&mut self) -> PieceType {
        self.current += 1;
        if self.current >= 7 { self.current = 0; }
        PIECES[self.current as usize]
    }
}
