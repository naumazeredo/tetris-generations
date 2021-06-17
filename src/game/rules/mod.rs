use crate::app::ImDraw;
use super::randomizer::RandomizerType;

pub mod movement;
pub mod line_clear;

use line_clear::LineClearRule;

// Guideline: https://tetris.fandom.com/wiki/Tetris_Guideline

#[derive(Copy, Clone, Debug, ImDraw)]
pub struct Rules {
    // @TODO use bitfields
    // gameplay rules

    // https://tetris.fandom.com/wiki/Drop
    pub has_hard_drop: bool,
    pub has_hard_drop_lock: bool, // Firm drop = false
    pub has_soft_drop: bool,
    pub has_soft_drop_lock: bool, // Lock when soft dropping

    pub has_hold_piece: bool,
    pub has_ghost_piece: bool,
    pub hold_piece_reset_rotation: bool,   // usually hold resets rotation
    pub spawn_immediate_drop: bool, // "Immediately drop one space if no existing Block is in its path"

    // @Maybe these are just related to spawning before entry delay
    pub has_initial_rotation_system: bool, // IRS
    pub has_initial_hold_system: bool,     // IHS

    // https://tetris.fandom.com/wiki/Infinity
    //pub has_lock_delay_infinity: bool,

    pub spawn_row: u8,
    pub next_pieces_preview_count: u8,

    pub wall_kick_rule: WallKickRule,
    //pub floor_kick_rule: FloorKickRule,
    pub line_clear_rule: LineClearRule,

    // @TODO some games implement a progression for these intervals (faster levels = smaller ARE,
    //       line clear delays and faster DAS). So we may need to accept a closure that receives the
    //       level
    pub das_repeat_delay: u64,
    pub das_repeat_interval: u64,
    pub soft_drop_interval: u64,
    pub line_clear_delay: u64,
    pub gravity_interval: u64,

    // @TODO depend on last lock height.
    //       Tetris NES: ARE is 10~18 frames depending on the height at which the piece locked;
    //                   pieces that lock in the bottom two rows are followed by 10 frames of entry
    //                   delay, and each group of 4 rows above that has an entry delay 2 frames
    //                   longer than the last;
    pub entry_delay: Option<u32>, // aka ARE
    pub lock_delay: Option<u32>,  // microsecs

    // piece positioning rules
    //pub rotation_system: RotationSystem, // It's quite annoying to make it completely general
    pub spawn_round_left: bool,
    pub has_extended_orientations: bool,
    pub is_right_handed: bool,
    pub is_spawn_flat_side_up: bool, // NRS: true, rest: false
    pub is_spawn_top_block_aligned: bool, // Original Rotation System: true, rest: false

    // randomizer
    pub randomizer_type: RandomizerType,
}

impl From<RotationSystem> for Rules {
    fn from(rotation_system: RotationSystem) -> Self {
        match rotation_system {
            RotationSystem::Original => {
                Self {
                    has_hard_drop: false,
                    has_hard_drop_lock: false,
                    has_soft_drop: true,
                    has_soft_drop_lock: false,
                    has_hold_piece: false,
                    has_ghost_piece: false,
                    hold_piece_reset_rotation: true,
                    spawn_immediate_drop: false,

                    has_initial_rotation_system: false,
                    has_initial_hold_system: false,

                    spawn_row: 20u8,
                    next_pieces_preview_count: 0u8,

                    wall_kick_rule: WallKickRule::Original,
                    line_clear_rule: LineClearRule::Naive,

                    das_repeat_delay: 266_228,   // 16 frames at 60.0988 Hz
                    das_repeat_interval: 99_835, // 6 frames at 60.0988 Hz
                    soft_drop_interval: 33_279,  // 1/2G at 60.0988 Hz
                    line_clear_delay: 332_785,   // 20 frames at 60.0988 Hz
                    gravity_interval: 1_000_000,

                    entry_delay: None,
                    lock_delay: None,

                    spawn_round_left: true,
                    has_extended_orientations: false,
                    is_right_handed: true,
                    is_spawn_flat_side_up: true,
                    is_spawn_top_block_aligned: true,

                    randomizer_type: RandomizerType::FullRandom,
                }
            },
            _ => {
                Self {
                    has_hard_drop: true,
                    has_hard_drop_lock: false,
                    has_soft_drop: true,
                    has_soft_drop_lock: false,
                    has_hold_piece: true,
                    has_ghost_piece: true,
                    hold_piece_reset_rotation: true,
                    spawn_immediate_drop: false,

                    has_initial_rotation_system: false,
                    has_initial_hold_system: false,

                    spawn_row: 20u8,
                    next_pieces_preview_count: 2u8,

                    wall_kick_rule: WallKickRule::Original,
                    line_clear_rule: LineClearRule::Naive,

                    das_repeat_delay: 266_228,   // 16 frames at 60.0988 Hz
                    das_repeat_interval: 99_835, // 6 frames at 60.0988 Hz
                    soft_drop_interval: 33_279,  // 1/2G at 60.0988 Hz
                    line_clear_delay: 332_785,   // 20 frames at 60.0988 Hz
                    gravity_interval: 250_000,

                    entry_delay: None,
                    lock_delay: None,

                    spawn_round_left: true,
                    has_extended_orientations: false,
                    is_right_handed: false,
                    is_spawn_flat_side_up: true,
                    is_spawn_top_block_aligned: true,

                    randomizer_type: RandomizerType::FullRandom,
                }
            }
        }
    }
}

/*
// https://tetris.fandom.com/wiki/Drop
#[derive(Copy, Clone, Debug)]
pub enum HardDropRule {
    No,
    HardDrop,
    FirmDrop,
}

#[derive(Copy, Clone, Debug)]
pub enum SoftDropRule {
    No,
    SoftDrop,
    SoftDropLock,
}
*/

// https://tetris.fandom.com/wiki/Category:Rotation_Systems
#[derive(Copy, Clone, Debug, ImDraw)]
pub enum RotationSystem {
    Original, // Original Rotation System
    NRSL,     // Nintendo Rotation System - Left Handed
    NRSR,     // Nintendo Rotation System - Right Handed
    Sega,     // row 20 or 22 in TGMACE
    ARS,      // Arika Rotation System
    SRS,      // Super Rotation System
    DTET,
    Test,
}

// https://tetris.fandom.com/wiki/Wall_kick
#[derive(Copy, Clone, Debug, ImDraw)]
pub enum WallKickRule {
    Original, // https://tetris.fandom.com/wiki/Original_Rotation_System
    TGM,      // https://tetris.fandom.com/wiki/TGM_Rotation
    TGM3,     // https://tetris.fandom.com/wiki/TGM_Rotation
    DX,       // https://tetris.fandom.com/wiki/Tetris_DX
    SRS,      // https://tetris.fandom.com/wiki/SRS
    DTET,     // https://tetris.fandom.com/wiki/DTET
}

impl Default for WallKickRule {
    fn default() -> Self {
        WallKickRule::SRS
    }
}

// https://tetris.fandom.com/wiki/Top_out
// @TODO bitflags
#[derive(Copy, Clone, Debug)]
pub enum TopOutRule {
    BlockOut,
    LockOut,
    PartialLockOut,
    GarbageOut,
}
