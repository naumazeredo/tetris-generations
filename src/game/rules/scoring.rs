use std::cmp::min;
use super::*;

#[derive(Copy, Clone, Debug, ImDraw)]
pub enum ScoringRule {
    //Original,
    Classic,
    Guideline,
}

// @TODO T-spin, combo, back-to-back
pub fn lock_piece_score(
    level: u32,
    locked_piece: LockedPiece,
    rules: &Rules
) -> u32 {
    match rules.scoring_curve {
        ScoringRule::Classic => {
            // Drops
            let soft_drop_score = min(20, locked_piece.soft_drop_steps) as u32;
            let hard_drop_score = min(40, 2 * locked_piece.hard_drop_steps) as u32;

            // Line clear score
            let clear_score = match locked_piece.lock_piece_result {
                // @TODO add T-Spin by line clear amount
                LockedPieceResult::Single(_) => 40,
                LockedPieceResult::Double(_) => 100,
                LockedPieceResult::Triple(_) => 300,
                LockedPieceResult::Tetris(_) => 1200,
                _ => 0,
            };

            let clear_score = clear_score * (level + 1);

            clear_score + soft_drop_score + hard_drop_score
        },

        _ => { 0 }
    }
}
