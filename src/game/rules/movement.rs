use crate::linalg::Vec2i;
use crate::game::{
    pieces::Piece,
    playfield::Playfield,
};

use super::*;

pub enum PieceState {
    Falling,
    Locking,
}

// @TODO Rules method?
pub fn try_move_piece(
    piece: &Piece,
    pos: &mut Vec2i,
    playfield: &Playfield,
    dx: i32,
    dy: i32,
    rotation_system: RotationSystem
) -> bool {
    for block_pos in piece.blocks(rotation_system) {
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

pub fn is_piece_locking(
    piece: &Piece,
    pos: Vec2i,
    playfield: &Playfield,
    rotation_system: RotationSystem
) -> bool {
    for block_pos in piece.blocks(rotation_system) {
        let down_x = pos.x + block_pos.x;
        let down_y = pos.y + block_pos.y - 1;
        if playfield.block(down_x, down_y).is_some() {
            return true;
        }
    }

    false
}

// @TODO Rules method?
pub fn try_apply_gravity(
    piece: &Piece,
    pos: &mut Vec2i,
    playfield: &Playfield,
    rotation_system: RotationSystem
) -> bool {
    try_move_piece(piece, pos, playfield, 0, -1, rotation_system)
}

// @TODO Rules method?
pub fn lock_piece(
    piece: &Piece,
    pos: Vec2i,
    playfield: &mut Playfield,
    rotation_system: RotationSystem
) {
    for block_pos in piece.blocks(rotation_system) {
        playfield.set_block(
            pos.x + block_pos.x,
            pos.y + block_pos.y,
            piece.type_,
        );
    }
}

// @TODO Rules method?
pub fn try_soft_drop_piece(
    piece: &Piece,
    pos: &mut Vec2i,
    playfield: &Playfield,
    rules: &Rules
) -> bool {
    if !rules.has_soft_drop { return false; }
    try_move_piece(piece, pos, playfield, 0, -1, rules.rotation_system)
}

// @TODO Rules method?
pub fn try_hard_drop_piece(
    piece: &Piece,
    pos: &mut Vec2i,
    playfield: &mut Playfield,
    rules: &Rules
) -> bool {
    if !rules.has_hard_drop { return false; }

    full_drop_piece(piece, pos, playfield, rules.rotation_system);
    lock_piece(piece, *pos, playfield, rules.rotation_system);
    true
}

// @TODO Rules method?
pub fn full_drop_piece(
    piece: &Piece,
    pos: &mut Vec2i,
    playfield: &Playfield,
    rotation_system: RotationSystem
) -> bool {
    let has_moved = try_apply_gravity(piece, pos, playfield, rotation_system);
    while try_apply_gravity(piece, pos, playfield, rotation_system) {}
    has_moved
}

#[cfg(test)]
mod tests {
    use crate::linalg::*;
    use crate::game::piece::PieceType;
    use super::*;

    #[test]
    fn test_try_move_piece() {
        let playfield_grid_size = Vec2i { x: 10, y: 40 };
        let playfield = Playfield::new(Vec2i::new(), playfield_grid_size);

        let mut piece = Piece {
            type_: PieceType::S,
            pos: Vec2i { x: playfield_grid_size.x / 2 - 2, y: 20 },
            rot: 0,
        };

        let old_blocks = piece.blocks().clone();
        try_move_piece(&mut piece, &playfield, 0, -1);

        for (new, old) in piece.blocks().iter().zip(old_blocks.iter()) {
            assert_eq!(new.x, old.x);
            assert_eq!(new.y, old.y - 1);
        }
    }
}
