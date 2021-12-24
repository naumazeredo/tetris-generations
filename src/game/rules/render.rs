use super::*;
use crate::app::*;
use crate::linalg::*;

pub struct InstanceStyle {
    playfield_pos:   Vec2i,
    hold_window_pos: Vec2i,
    next_window_pos: [Vec2i; NEXT_PIECES_COUNT],

    // visible_rows_count: u8,
    has_grid: bool,
}
