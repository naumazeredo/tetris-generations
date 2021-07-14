use crate::linalg::Vec2i;
use crate::game::{
    pieces::{ Piece, PieceType },
    playfield::Playfield,
};

use super::*;

// Move this to rules
pub fn try_rotate_piece(
    piece: &mut Piece,
    pos: &mut Vec2i,
    is_clockwise: bool,
    playfield: &Playfield,
    rules: &Rules
) -> bool {
    let delta_rot = if is_clockwise { 1 } else { -1 };

    match rules.rotation_system {
        | RotationSystem::Original
        | RotationSystem::NRSR
        | RotationSystem::NRSL
        | RotationSystem::Sega

        // @TODO these systems have wall kicks
        | RotationSystem::ARS
        | RotationSystem::DTET

        => {
            for block_pos in piece.blocks_with_rot(piece.rot + delta_rot) {
                let x = pos.x + block_pos.x;
                let y = pos.y + block_pos.y;
                if playfield.block(x, y).is_some() {
                    return false;
                }
            }

            piece.rot += delta_rot;
            true
        },

        | RotationSystem::SRS
        => {
            for delta_pos in get_srs_rotation_tests(piece, is_clockwise) {
                let can_rotate = piece
                    .blocks_with_rot(piece.rot + delta_rot)
                    .iter()
                    .all(|block_pos| {
                        let x = pos.x + block_pos.x + delta_pos.x;
                        let y = pos.y + block_pos.y + delta_pos.y;

                        playfield.block(x, y).is_none()
                    });

                if can_rotate {
                    piece.rot += delta_rot;
                    *pos += *delta_pos;
                    return true;
                }
            }

            false
        },
    }
}

// https://tetris.fandom.com/wiki/Wall_kick

// https://tetris.fandom.com/wiki/SRS
const NO_TESTS: [Vec2i; 1] = [
    Vec2i { x:  0, y:  0 },
];

const JLSTZ_TESTS: [[[Vec2i; 5]; 2]; 4] = [
    // 0
    [
        // 0 -> 1
        [
            Vec2i { x:  0, y:  0 },
            Vec2i { x: -1, y:  0 },
            Vec2i { x: -1, y:  1 },
            Vec2i { x:  0, y: -2 },
            Vec2i { x: -1, y: -2 },
        ],

        // 0 -> 3
        [
            Vec2i { x: 0, y:  0 },
            Vec2i { x: 1, y:  0 },
            Vec2i { x: 1, y:  1 },
            Vec2i { x: 0, y: -2 },
            Vec2i { x: 1, y: -2 },
        ],
    ],

    // 1
    [
        // 1 -> 2
        [
            Vec2i { x: 0, y:  0 },
            Vec2i { x: 1, y:  0 },
            Vec2i { x: 1, y: -1 },
            Vec2i { x: 0, y:  2 },
            Vec2i { x: 1, y:  2 },
        ],

        // 1 -> 0
        [
            Vec2i { x: 0, y:  0 },
            Vec2i { x: 1, y:  0 },
            Vec2i { x: 1, y: -1 },
            Vec2i { x: 0, y:  2 },
            Vec2i { x: 1, y:  2 },
        ],
    ],

    // 2
    [
        // 2 -> 3
        [
            Vec2i { x: 0, y:  0 },
            Vec2i { x: 1, y:  0 },
            Vec2i { x: 1, y:  1 },
            Vec2i { x: 0, y: -2 },
            Vec2i { x: 1, y: -2 },
        ],

        // 2 -> 1
        [
            Vec2i { x:  0, y:  0 },
            Vec2i { x: -1, y:  0 },
            Vec2i { x: -1, y:  1 },
            Vec2i { x:  0, y: -2 },
            Vec2i { x: -1, y: -2 },
        ],
    ],

    // 3
    [
        // 3 -> 0
        [
            Vec2i { x:  0, y:  0 },
            Vec2i { x: -1, y:  0 },
            Vec2i { x: -1, y: -1 },
            Vec2i { x:  0, y:  2 },
            Vec2i { x: -1, y:  2 },
        ],

        // 3 -> 2
        [
            Vec2i { x:  0, y:  0 },
            Vec2i { x: -1, y:  0 },
            Vec2i { x: -1, y: -1 },
            Vec2i { x:  0, y:  2 },
            Vec2i { x: -1, y:  2 },
        ],
    ],
];

const I_TESTS: [[[Vec2i; 5]; 2]; 4] = [
    // 0
    [
        // 0 -> 1
        [
            Vec2i { x:  0, y:  0 },
            Vec2i { x: -2, y:  0 },
            Vec2i { x:  1, y:  1 },
            Vec2i { x: -2, y: -1 },
            Vec2i { x:  1, y:  2 },
        ],

        // 0 -> 3
        [
            Vec2i { x:  0, y:  0 },
            Vec2i { x: -1, y:  0 },
            Vec2i { x:  2, y:  0 },
            Vec2i { x: -1, y:  2 },
            Vec2i { x:  2, y: -1 },
        ],
    ],

    // 1
    [
        // 1 -> 2
        [
            Vec2i { x:  0, y:  0 },
            Vec2i { x: -1, y:  0 },
            Vec2i { x:  2, y:  0 },
            Vec2i { x: -1, y:  2 },
            Vec2i { x:  2, y: -1 },
        ],

        // 1 -> 0
        [
            Vec2i { x:  0, y:  0 },
            Vec2i { x:  2, y:  0 },
            Vec2i { x: -1, y:  0 },
            Vec2i { x:  2, y:  1 },
            Vec2i { x: -1, y: -2 },
        ],
    ],

    // 2
    [
        // 2 -> 3
        [
            Vec2i { x:  0, y:  0 },
            Vec2i { x:  2, y:  0 },
            Vec2i { x: -1, y:  0 },
            Vec2i { x:  2, y:  1 },
            Vec2i { x: -1, y: -2 },
        ],

        // 2 -> 1
        [
            Vec2i { x:  0, y:  0 },
            Vec2i { x:  1, y:  0 },
            Vec2i { x: -2, y:  0 },
            Vec2i { x:  1, y: -2 },
            Vec2i { x: -2, y:  1 },
        ],
    ],

    // 3
    [
        // 3 -> 0
        [
            Vec2i { x:  0, y:  0 },
            Vec2i { x:  1, y:  0 },
            Vec2i { x: -2, y:  0 },
            Vec2i { x:  1, y: -2 },
            Vec2i { x: -2, y:  1 },
        ],

        // 3 -> 2
        [
            Vec2i { x:  0, y:  0 },
            Vec2i { x: -2, y:  0 },
            Vec2i { x:  1, y:  0 },
            Vec2i { x: -2, y: -1 },
            Vec2i { x:  1, y:  2 },
        ],
    ],
];

fn get_srs_rotation_tests(piece: &Piece, is_clockwise: bool) -> &'static [Vec2i] {
    let rot = (((piece.rot % 4) + 4) % 4) as usize;
    let dir = (!is_clockwise) as usize;
    match piece.type_ {
        | PieceType::J
        | PieceType::L
        | PieceType::S
        | PieceType::T
        | PieceType::Z
        => {
            &JLSTZ_TESTS[rot][dir]
        }

        PieceType::I => { &I_TESTS[rot][dir] }
        PieceType::O => { &NO_TESTS }
    }
}

// https://tetris.fandom.com/wiki/Original_Rotation_System
// https://tetris.fandom.com/wiki/TGM_Rotation
// https://tetris.fandom.com/wiki/TGM_Rotation
// https://tetris.fandom.com/wiki/Tetris_DX
// https://tetris.fandom.com/wiki/DTET
