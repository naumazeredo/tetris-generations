use bitflags::bitflags;
use crate::app::ImDraw;
use crate::linalg::Vec2i;
use crate::game::{
    pieces::Piece,
    playfield::{ Playfield, PLAYFIELD_VISIBLE_HEIGHT },
};

use super::{ Rules, movement::try_move_piece };

// https://tetris.fandom.com/wiki/Top_out
// @TODO bitflags

bitflags! {
    pub struct TopOutRule: u8 {
        const BLOCK_OUT        = 0b0001;
        const LOCK_OUT         = 0b0010;
        const PARTIAL_LOCK_OUT = 0b0100;
        const GARBAGE_OUT      = 0b1000;
    }
}

impl_imdraw_todo!(TopOutRule);

// @TODO Rules method?
pub fn blocked_out(
    piece: &Piece,
    pos: Vec2i,
    playfield: &Playfield,
    rules: &Rules
) -> bool {
    if !rules.top_out_rule.contains(TopOutRule::BLOCK_OUT) { return false; }

    let mut pos = pos;
    !try_move_piece(piece, &mut pos, playfield, 0, 0)
}

pub fn locked_out(
    piece: &Piece,
    pos: Vec2i,
    rules: &Rules
) -> bool {
    if rules.top_out_rule.intersects(TopOutRule::LOCK_OUT | TopOutRule::PARTIAL_LOCK_OUT) {
        let blocks_locked_out = blocks_out_of_playfield(piece, pos);
        if rules.top_out_rule.contains(TopOutRule::LOCK_OUT) {
            return blocks_locked_out == 4;
        } else {
            return blocks_locked_out > 0;
        }
    }

    false
}

pub fn blocks_out_of_playfield(
    piece: &Piece,
    pos: Vec2i,
) -> u8 {
    piece.blocks().iter()
        .fold(0, |acc, block_pos| {
            acc + (pos.y + block_pos.y >= PLAYFIELD_VISIBLE_HEIGHT) as u8
        })
}
