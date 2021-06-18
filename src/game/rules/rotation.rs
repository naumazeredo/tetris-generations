use crate::linalg::Vec2i;
use crate::game::{
    piece::Piece,
    playfield::Playfield,
};

use super::*;

// Move this to rules
pub fn try_rotate_piece(
    piece: &mut Piece,
    pos: &mut Vec2i,
    clockwise: bool,
    playfield: &Playfield,
    rules: &Rules
) -> bool {
    let delta_rot = if clockwise { 1 } else { -1 };

    match rules.wall_kick_rule {
        WallKickRule::Original => {
            for block_pos in piece.type_.blocks(piece.rot + delta_rot) {
                let x = pos.x + block_pos.x;
                let y = pos.y + block_pos.y;
                if playfield.block(x, y).is_some() {
                    return false;
                }
            }

            piece.rot += delta_rot;
            true
        },

        _ => { unimplemented!("rotation system not implemented"); }
    }
}
