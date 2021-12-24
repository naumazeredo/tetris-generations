use crate::app::ImDraw;
use super::randomizer::RandomizerType;

pub mod instance;
pub mod line_clear;
pub mod lock;
pub mod movement;
pub mod rotation;
pub mod scoring;
pub mod topout;

use lock::{LockedPieceResult, LockedPiece};
use line_clear::{LineClearAnimationType, LineClearRule};
use scoring::ScoringRule;
use topout::TopOutRule;

// Guideline: https://tetris.fandom.com/wiki/Tetris_Guideline

#[derive(Clone, Debug, ImDraw)]
pub struct Rules {
    // @TODO use bitfields
    // gameplay rules

    // @TODO HardDropRule, SoftDropRule
    // https://tetris.fandom.com/wiki/Drop
    pub has_hard_drop: bool,
    pub has_hard_drop_lock: bool, // Firm drop = false
    pub has_soft_drop: bool,
    pub has_soft_drop_lock: bool, // Lock when soft dropping

    pub has_ghost_piece: bool,
    pub has_hold_piece: bool,
    pub hold_piece_reset_rotation: bool,   // usually hold resets rotation

    // @Maybe these are just related to spawning before entry delay
    pub has_initial_rotation_system: bool, // IRS
    pub has_initial_hold_system: bool,     // IHS
    pub spawn_drop: bool, // "Immediately drop one space if no existing Block is in its path"

    // https://tetris.fandom.com/wiki/Infinity
    //pub has_lock_delay_infinity: bool,

    pub spawn_row: u8,
    pub next_pieces_preview_count: u8,

    pub line_clear_rule: LineClearRule,

    pub top_out_rule: TopOutRule,

    // @TODO some games implement a progression for these intervals (faster levels = smaller ARE,
    //       line clear delays and faster DAS). So we may need to accept a closure that receives the
    //       level
    pub das_repeat_delay: u64, // DAS - Delayed Auto Shift (initial delay)
    pub das_repeat_interval: u64, // ARR - Auto-Repeat Rate (all other delays)
    pub soft_drop_interval: u64,
    pub line_clear_delay: u64,

    pub gravity_curve: GravityCurve,
    pub scoring_curve: ScoringRule,
    pub level_curve: LevelCurve, // @Maybe rename to difficulty curve
    pub start_level: u8,

    // THIS IS A CONSTANT (how can we do this better?)
    pub minimum_level: u8, // Some games start at 0, some start at 1

    // @TODO depend on last lock height.
    //       Tetris NES: ARE is 10~18 frames depending on the height at which the piece locked;
    //                   pieces that lock in the bottom two rows are followed by 10 frames of entry
    //                   delay, and each group of 4 rows above that has an entry delay 2 frames
    //                   longer than the last;
    pub entry_delay: u64, // aka ARE

    pub lock_delay: LockDelayRule,

    // piece positioning rules
    pub rotation_system: RotationSystem,
    //pub does_ceiling_prevents_rotation: bool, // Sega
    //pub double_rotation: bool, // DTET
    //pub has_wall_kicks: bool, // Disable/enable wall kicks

    // @Design this is part of the rotation system!
    //pub has_ccw_rotation: bool,

    // randomizer
    pub randomizer_type: RandomizerType,

    // Animation
    has_movement_animation: bool,
    movement_animation_show_ghost: bool, // @Remove this is debug only
    movement_animation_duration: u64,

    line_clear_animation_type: Option<LineClearAnimationType>,

    has_locking_animation: bool,
    locking_animation_duration: u64,
}

// @TODO RulesBuilder?

impl From<RotationSystem> for Rules {
    fn from(rotation_system: RotationSystem) -> Self {
        match rotation_system {
            RotationSystem::NRSR => {
                Self {
                    has_hard_drop: false,
                    has_hard_drop_lock: false,
                    has_soft_drop: true,
                    has_soft_drop_lock: false,
                    has_hold_piece: false,
                    has_ghost_piece: false,
                    hold_piece_reset_rotation: true,
                    spawn_drop: false,

                    has_initial_rotation_system: false,
                    has_initial_hold_system: false,

                    spawn_row: 20u8,
                    next_pieces_preview_count: 1u8,

                    line_clear_rule: LineClearRule::Naive,

                    top_out_rule: TopOutRule::BLOCK_OUT,

                    das_repeat_delay: 266_228,   // 16 frames at 60.0988 Hz
                    das_repeat_interval: 99_835, // 6 frames at 60.0988 Hz
                    soft_drop_interval: 33_279,  // 1/2G at 60.0988 Hz
                    line_clear_delay: 332_785,   // 20 frames at 60.0988 Hz

                    gravity_curve: GravityCurve::Classic,
                    scoring_curve: ScoringRule::Classic,
                    level_curve: LevelCurve::Classic,
                    start_level: 0,
                    minimum_level: 0,

                    entry_delay: 0,
                    lock_delay: LockDelayRule::NoDelay,

                    rotation_system: RotationSystem::NRSR,

                    randomizer_type: RandomizerType::FullRandom,

                    // Animation
                    has_movement_animation: false,
                    movement_animation_show_ghost: false,
                    movement_animation_duration: 0,

                    line_clear_animation_type: Some(LineClearAnimationType::Classic),

                    has_locking_animation: false,
                    locking_animation_duration: 0,
                }
            }

            RotationSystem::SRS => {
                Self {
                    has_hard_drop: true,
                    has_hard_drop_lock: true,
                    has_soft_drop: true,
                    has_soft_drop_lock: false,
                    has_hold_piece: true,
                    has_ghost_piece: true,
                    hold_piece_reset_rotation: true,
                    spawn_drop: true,

                    has_initial_rotation_system: false,
                    has_initial_hold_system: false,

                    spawn_row: 22u8,
                    next_pieces_preview_count: 4u8,

                    line_clear_rule: LineClearRule::Naive,

                    top_out_rule: TopOutRule::BLOCK_OUT | TopOutRule::LOCK_OUT,

                    das_repeat_delay: 266_228,   // 16 frames at 60.0988 Hz
                    das_repeat_interval: 99_835, // 6 frames at 60.0988 Hz
                    soft_drop_interval: 33_279,  // 1/2G at 60.0988 Hz
                    line_clear_delay: 332_785,   // 20 frames at 60.0988 Hz

                    gravity_curve: GravityCurve::Classic,
                    scoring_curve: ScoringRule::Classic,
                    level_curve: LevelCurve::Classic,
                    start_level: 1,
                    minimum_level: 1,

                    entry_delay: 0,
                    lock_delay: LockDelayRule::MoveReset {
                        duration: 500_000,
                        rotations: 5,
                        movements: 5,
                    },

                    rotation_system: RotationSystem::SRS,

                    randomizer_type: RandomizerType::FullRandom,

                    // Animation
                    has_movement_animation: true,
                    movement_animation_show_ghost: false,
                    movement_animation_duration: 50_000,

                    line_clear_animation_type: Some(LineClearAnimationType::Classic),

                    has_locking_animation: true,
                    locking_animation_duration: 250_000,
                }
            }

            _ => {
                Self {
                    has_hard_drop: true,
                    has_hard_drop_lock: false,
                    has_soft_drop: true,
                    has_soft_drop_lock: false,

                    has_hold_piece: true,
                    has_ghost_piece: true,
                    hold_piece_reset_rotation: true,
                    spawn_drop: true,

                    has_initial_rotation_system: false,
                    has_initial_hold_system: false,

                    spawn_row: 22u8,
                    next_pieces_preview_count: 4u8,

                    line_clear_rule: LineClearRule::Naive,

                    top_out_rule: TopOutRule::BLOCK_OUT | TopOutRule::LOCK_OUT,

                    das_repeat_delay: 266_228,   // 16 frames at 60.0988 Hz
                    das_repeat_interval: 99_835, // 6 frames at 60.0988 Hz
                    soft_drop_interval: 33_279,  // 1/2G at 60.0988 Hz
                    line_clear_delay: 332_785,   // 20 frames at 60.0988 Hz

                    gravity_curve: GravityCurve::Classic,
                    scoring_curve: ScoringRule::Classic,
                    level_curve: LevelCurve::Classic,
                    start_level: 1,
                    minimum_level: 1,

                    entry_delay: 0,
                    lock_delay: LockDelayRule::MoveReset {
                        duration: 500_000,
                        rotations: 5,
                        movements: 5,
                    },

                    rotation_system: RotationSystem::SRS,

                    randomizer_type: RandomizerType::FullRandom,

                    // Animation
                    has_movement_animation: true,
                    movement_animation_show_ghost: true,
                    movement_animation_duration: 50_000,

                    line_clear_animation_type: Some(LineClearAnimationType::Classic),

                    has_locking_animation: true,
                    locking_animation_duration: 250_000,
                }
            }
        }
    }
}

/*
// @TODO HardDropRule, SoftDropRule
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

// @TODO macro this
pub const ROTATION_SYSTEM_NAMES: &[&str] = &["ORIGINAL", "NRSL", "NRSR", "SEGA", "ARS", "SRS", "DTET"];

// https://tetris.fandom.com/wiki/Category:Rotation_Systems
#[derive(Copy, Clone, Debug, ImDraw)]
pub enum RotationSystem {
    Original, // Original Rotation System
    NRSL,     // Nintendo Rotation System - Left Handed
    NRSR,     // Nintendo Rotation System - Right Handed
    Sega,     // row 20 or 22 in TGMACE
    ARS,      // Arika Rotation System - TGM Rotation
    SRS,      // Super Rotation System
    DTET,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ImDraw)]
pub enum LockDelayRule {
    NoDelay,
    EntryReset(u64),
    StepReset(u64),
    MoveReset { duration: u64, rotations: u8, movements: u8 },
    //MoveResetInfinity(u64),
}

#[derive(Copy, Clone, Debug, ImDraw)]
pub enum GravityCurve {
    //Original,
    Classic,
    Guideline,
    //Tetris99,
}

#[derive(Copy, Clone, Debug, ImDraw)]
pub enum LevelCurve {
    //Original,
    Classic,
    Guideline,
}

impl Rules {
    // @TODO move to gravity.rs
    pub fn get_gravity_interval(&self, level: u32) -> u64 {
        match self.gravity_curve {
            GravityCurve::Classic => {
                let gravity = match level {
                    0       => 48,
                    1       => 43,
                    2       => 38,
                    3       => 33,
                    4       => 28,
                    5       => 23,
                    6       => 18,
                    7       => 13,
                    8       => 8,
                    9       => 6,
                    10..=12 => 5,
                    13..=15 => 4,
                    16..=18 => 3,
                    19..=28 => 2,
                    _ => 1,
                };

                let frame_duration = (1_000_000.0 / 60.0988) as u64;
                gravity * frame_duration
            },

            _ => { 1_000_000 }
        }
    }

    // @TODO move somewhere else (level.rs? progress.rs? difficulty?)
    // @XXX this seems a very bad design. The player can't just start at any level, for example
    pub fn get_level(&self, _score: u32, total_lines_cleared: u32) -> u32 {
        match self.level_curve {
            LevelCurve::Classic => {
                // @TODO implement the real classic bugged logic
                //https://meatfighter.com/nintendotetrisai/#Lines_and_Statistics

                let lines_to_next_level_from_start_level = [
                    0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 90, 90, 90, 90, 90, 90,
                    100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 190, 190
                ];

                // @Remove
                if self.start_level as usize >= lines_to_next_level_from_start_level.len() {
                    return 200;
                }

                let line_diff_from_start =
                    total_lines_cleared.saturating_sub(
                        lines_to_next_level_from_start_level[self.start_level as usize]
                    );

                self.start_level as u32 + (line_diff_from_start / 10)
            }

            _ => { 0 }
        }
    }
    /*
    enum LevelProgressData {
        Classic { total_lines_cleared: u32 },
        Guideline { score: u32 },
    }
    */
}

