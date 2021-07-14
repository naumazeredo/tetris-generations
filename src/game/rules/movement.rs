use crate::linalg::Vec2i;
use crate::game::{
    pieces::Piece,
    playfield::Playfield,
};

pub fn try_move_piece(
    piece: &Piece,
    pos: &mut Vec2i,
    playfield: &Playfield,
    dx: i32,
    dy: i32,
) -> bool {
    for block_pos in piece.blocks() {
        let new_x = pos.x + block_pos.x + dx;
        let new_y = pos.y + block_pos.y + dy;
        if playfield.block(new_x, new_y).is_some() {
            return false;
        }
    }

    pos.x += dx;
    pos.y += dy;
    true
}

pub fn try_apply_gravity(
    piece: &Piece,
    pos: &mut Vec2i,
    playfield: &Playfield,
) -> bool {
    try_move_piece(piece, pos, playfield, 0, -1)
}

pub fn full_drop_piece(
    piece: &Piece,
    pos: &mut Vec2i,
    playfield: &Playfield,
) -> u8 {
    let mut step_count = 0;
    while try_apply_gravity(piece, pos, playfield) {
        step_count += 1;
    }
    step_count
}
