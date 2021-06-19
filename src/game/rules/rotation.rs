use crate::linalg::Vec2i;
use crate::game::{
    pieces::Piece,
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

    match rules.rotation_system {
        | RotationSystem::Original
        | RotationSystem::NRSR
        | RotationSystem::NRSL
        | RotationSystem::Sega

        // @TODO these systems have wall kicks
        | RotationSystem::SRS
        | RotationSystem::ARS
        | RotationSystem::DTET

        => {
            for block_pos in piece.type_.blocks(piece.rot + delta_rot, rules.rotation_system) {
                let x = pos.x + block_pos.x;
                let y = pos.y + block_pos.y;
                if playfield.block(x, y).is_some() {
                    return false;
                }
            }

            piece.rot += delta_rot;
            true
        },
    }
}

// https://tetris.fandom.com/wiki/Wall_kick

// https://tetris.fandom.com/wiki/Original_Rotation_System
// https://tetris.fandom.com/wiki/TGM_Rotation
// https://tetris.fandom.com/wiki/TGM_Rotation
// https://tetris.fandom.com/wiki/Tetris_DX
// https://tetris.fandom.com/wiki/SRS
// https://tetris.fandom.com/wiki/DTET
