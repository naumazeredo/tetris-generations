use crate::app::ImDraw;
use super::randomizer::RandomizerType;

// Guideline: https://tetris.fandom.com/wiki/Tetris_Guideline

#[derive(Copy, Clone, Debug, ImDraw)]
pub struct Rules {
    // @TODO use bitfields
    // gameplay rules
    pub has_hard_drop: bool,
    pub has_firm_drop: bool,
    pub has_soft_drop: bool,
    pub has_soft_drop_lock: bool, // Lock when soft dropping
    pub has_hold_piece: bool,
    pub has_ghost_piece: bool,
    pub hold_piece_reset_rotation: bool,   // usually hold resets rotation
    pub spawn_immediate_drop: bool, // "Immediately drop one space if no existing Block is in its path"

    // @Maybe these are just related to spawning before entry delay
    pub has_initial_rotation_system: bool, // IRS
    pub has_initial_hold_system: bool,     // IHS

    pub spawn_row: u8,
    pub next_pieces_preview_count: u8,

    pub wall_kick_rule: WallKickRule,

    pub entry_delay: Option<u32>, // aka ARE
    pub lock_delay: Option<u32>,  // microsecs

    // piece positioning rules
    pub spawn_round_left: bool,
    pub has_extended_orientations: bool,
    pub is_right_handed: bool,
    pub is_spawn_flat_side_up: bool, // NRS: true, rest: false
    pub is_top_aligned: bool, // Original Rotation System: true, rest: false

    // randomizer
    pub randomizer_type: RandomizerType,
}

/*
// https://tetris.fandom.com/wiki/Drop
#[derive(Clone, Debug)]
pub enum HardDropRule {
    No,
    HardDrop,
    FirmDrop,
}
*/

impl From<RotationSystem> for Rules {
    fn from(rotation_system: RotationSystem) -> Self {
        match rotation_system {
            RotationSystem::Original => {
                Self {
                    has_hard_drop: false,
                    has_firm_drop: false,
                    has_soft_drop: true,
                    has_soft_drop_lock: false,
                    has_hold_piece: false,
                    has_ghost_piece: false,
                    hold_piece_reset_rotation: false,
                    spawn_immediate_drop: false,

                    has_initial_rotation_system: false,
                    has_initial_hold_system: false,

                    spawn_row: 20u8,
                    next_pieces_preview_count: 0u8,

                    wall_kick_rule: WallKickRule::Original,

                    entry_delay: None,
                    lock_delay: None,

                    spawn_round_left: true,
                    has_extended_orientations: false,
                    is_right_handed: true,
                    is_spawn_flat_side_up: true,
                    is_top_aligned: true,

                    randomizer_type: RandomizerType::FullRandom,
                }
            },
            _ => {
                Self {
                    has_hard_drop: false,
                    has_firm_drop: false,
                    has_soft_drop: false,
                    has_soft_drop_lock: false,
                    has_hold_piece: false,
                    has_ghost_piece: false,
                    hold_piece_reset_rotation: false,
                    spawn_immediate_drop: false,

                    has_initial_rotation_system: false,
                    has_initial_hold_system: false,

                    spawn_row: 22u8,
                    next_pieces_preview_count: 1u8,

                    wall_kick_rule: WallKickRule::Original,

                    entry_delay: None,
                    lock_delay: None,

                    spawn_round_left: true,
                    has_extended_orientations: false,
                    is_right_handed: false,
                    is_spawn_flat_side_up: true,
                    is_top_aligned: true,

                    randomizer_type: RandomizerType::FullRandom,
                }
            }
        }
    }
}

// https://tetris.fandom.com/wiki/Category:Rotation_Systems
#[derive(Copy, Clone, Debug)]
pub enum RotationSystem {
    Original, // Original Rotation System
    NRSL,     // Nintendo Rotation System - Left Handed
    NRSR,     // Nintendo Rotation System - Right Handed
    Sega,     // row 20 or 22 in TGMACE
    ARS,      // Arika Rotation System
    SRS,      // Super Rotation System
    DTET,
}

impl_imdraw_todo!(RotationSystem);

// https://tetris.fandom.com/wiki/Wall_kick
#[derive(Copy, Clone, Debug)]
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

impl_imdraw_todo!(WallKickRule);

// https://tetris.fandom.com/wiki/Top_out
// @TODO bitflags
#[derive(Copy, Clone, Debug)]
pub enum TopOutRule {
    BlockOut,
    LockOut,
    PartialLockOut,
    GarbageOut,
}
