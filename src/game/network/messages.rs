use crate::app::*;
use crate::linalg::Vec2i;
use crate::game::{
    randomizer::{Randomizer, RandomizerType},
    rules::{
        RotationSystem,
        instance::NEXT_PIECES_COUNT,
        lock::{LastPieceAction, LockedPiece, LockedPieceResult},
    },
    pieces::{Piece, PieceType},
    playfield::BlockType,
};

pub enum MultiplayerMessages {
    Connect(Connect),
    Update(Update),
}

#[derive(Debug)]
pub struct Connect {
    pub timestamp: u64,
    pub instance: NetworkedRulesInstance,

    // Removed from RulesInstance that needed to be synched
    //rules: Rules, // @XXX using rotation system instead since we are just testing and we have no macro to automate Serialize/Deserialize yet
    pub rotation_system: RotationSystem,
    pub randomizer: Randomizer,
}

#[derive(Debug)]
pub struct Update {
    pub timestamp: u64,
    pub instance: NetworkedRulesInstance,
}

// @Refactor Serialize/Deserialize should be a macro

// Rules Instance

#[derive(Debug)]
pub struct NetworkedPlayfield {
    pub grid_size: Vec2i, // @Fix This doesn't need to be synced!
    pub blocks: Vec<BlockType>,
}

#[derive(Debug)]
pub struct NetworkedRulesInstance {
    pub has_topped_out: bool, // per game
    pub playfield: NetworkedPlayfield,   // per game

    pub current_score: u32,       // per game
    pub total_lines_cleared: u32, // per game

    pub current_piece: Option<(Piece, Vec2i)>,
    pub next_piece_types: [PieceType; NEXT_PIECES_COUNT], // per game

    pub lock_piece_timestamp: u64,
    pub last_locked_piece: Option<LockedPiece>, // per piece

    pub hold_piece: Option<Piece>, // per game

    pub movement_last_timestamp_x: u64,
    pub movement_last_timestamp_y: u64,

    /*
    // @XXX Sync animations later
    // Animations
    has_movement_animation: bool,
    movement_animation_show_ghost: bool,
    movement_animation_duration: u64,
    movement_animation_delta_grid_x: f32,
    movement_animation_delta_grid_y: f32,
    movement_animation_current_delta_grid: Vec2,

    has_line_clear_animation: bool,
    is_line_clear_animation_playing: bool,
    line_clear_animation_state: LineClearAnimationState,

    locking_animation_timestamp: u64,
    locking_animation_duration: u64,
    */
}

impl Serialize for MultiplayerMessages {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), SerializationError> {
        match self {
            MultiplayerMessages::Connect(c) => {
                0u8.serialize(serializer)?;
                c.timestamp.serialize(serializer)?;
                c.instance.serialize(serializer)?;
                //c.rules.serialize(serializer)?;
                c.rotation_system.serialize(serializer)?;
                c.randomizer.serialize(serializer)?;
            },

            MultiplayerMessages::Update(u) => {
                1u8.serialize(serializer)?;
                u.timestamp.serialize(serializer)?;
                u.instance.serialize(serializer)?;
            },
        }
        Ok(())
    }
}

impl Deserialize for MultiplayerMessages {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self, SerializationError> {
        let t = match u8::deserialize(deserializer)? {
            0 => {
                let timestamp = u64::deserialize(deserializer)?;
                let instance = NetworkedRulesInstance::deserialize(deserializer)?;
                //let rules = Rules::deserialize(deserializer)?;
                let rotation_system = RotationSystem::deserialize(deserializer)?;
                let randomizer = Randomizer::deserialize(deserializer)?;

                MultiplayerMessages::Connect(Connect {
                    timestamp,
                    instance,
                    //rules,
                    rotation_system,
                    randomizer,
                })
            },

            1 => {
                let timestamp = u64::deserialize(deserializer)?;
                let instance = NetworkedRulesInstance::deserialize(deserializer)?;

                MultiplayerMessages::Update(Update {
                    timestamp,
                    instance,
                })
            }

            _ => unreachable!(),
        };
        Ok(t)
    }
}

impl Serialize for NetworkedPlayfield {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), SerializationError> {
        self.grid_size.serialize(serializer)?;
        self.blocks.serialize(serializer)?;
        Ok(())
    }
}

impl Deserialize for NetworkedPlayfield {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self, SerializationError> {
        let grid_size = Vec2i::deserialize(deserializer)?;
        let blocks = Vec::<BlockType>::deserialize(deserializer)?;
        Ok(Self { grid_size, blocks })
    }
}

impl Serialize for NetworkedRulesInstance {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), SerializationError> {
        self.has_topped_out.serialize(serializer)?;
        self.playfield.serialize(serializer)?;

        self.current_score.serialize(serializer)?;
        self.total_lines_cleared.serialize(serializer)?;

        self.current_piece.serialize(serializer)?;
        self.next_piece_types.serialize(serializer)?;

        self.lock_piece_timestamp.serialize(serializer)?;
        self.last_locked_piece.serialize(serializer)?;

        self.hold_piece.serialize(serializer)?;

        self.movement_last_timestamp_x.serialize(serializer)?;
        self.movement_last_timestamp_y.serialize(serializer)?;

        Ok(())
    }
}

impl Deserialize for NetworkedRulesInstance {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self, SerializationError> {
        let has_topped_out = bool::deserialize(deserializer)?;
        let playfield = NetworkedPlayfield::deserialize(deserializer)?;

        let current_score = u32::deserialize(deserializer)?;
        let total_lines_cleared = u32::deserialize(deserializer)?;

        let current_piece = Option::<(Piece, Vec2i)>::deserialize(deserializer)?;
        let next_piece_types = <[PieceType; NEXT_PIECES_COUNT]>::deserialize(deserializer)?;

        let lock_piece_timestamp = u64::deserialize(deserializer)?;
        let last_locked_piece = Option::<LockedPiece>::deserialize(deserializer)?;

        let hold_piece = Option::<Piece>::deserialize(deserializer)?;

        let movement_last_timestamp_x = u64::deserialize(deserializer)?;
        let movement_last_timestamp_y = u64::deserialize(deserializer)?;

        Ok(Self {
            has_topped_out,
            playfield,

            current_score,
            total_lines_cleared,

            current_piece,
            next_piece_types,

            lock_piece_timestamp,
            last_locked_piece,

            hold_piece,

            movement_last_timestamp_x,
            movement_last_timestamp_y,
        })
    }
}

impl Serialize for Vec2i {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), SerializationError> {
        self.x.serialize(serializer)?;
        self.y.serialize(serializer)?;
        Ok(())
    }
}

impl Deserialize for Vec2i {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self, SerializationError> {
        let x = i32::deserialize(deserializer)?;
        let y = i32::deserialize(deserializer)?;
        Ok(Self { x, y })
    }
}

impl Serialize for RandomizerType {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), SerializationError> {
        match *self {
            RandomizerType::Sequential => serializer.serialize_packed_u8::<0, 5>(0)?,
            RandomizerType::FullRandom => serializer.serialize_packed_u8::<0, 5>(1)?,
            RandomizerType::Random7Bag => serializer.serialize_packed_u8::<0, 5>(2)?,
            RandomizerType::TGMACE     => serializer.serialize_packed_u8::<0, 5>(3)?,
            RandomizerType::TGM1       => serializer.serialize_packed_u8::<0, 5>(4)?,
            RandomizerType::TGM        => serializer.serialize_packed_u8::<0, 5>(5)?,
        }
        Ok(())
    }
}

impl Deserialize for RandomizerType {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self, SerializationError> {
        let t = match deserializer.deserialize_packed_u8::<0, 5>()? {
            0 => RandomizerType::Sequential,
            1 => RandomizerType::FullRandom,
            2 => RandomizerType::Random7Bag,
            3 => RandomizerType::TGMACE,
            4 => RandomizerType::TGM1,
            _ => RandomizerType::TGM,
        };
        Ok(t)
    }
}

impl Serialize for Randomizer {
    fn serialize(&self, _serializer: &mut Serializer) -> Result<(), SerializationError> {
        // @TODO
        Ok(())
    }
}

impl Deserialize for Randomizer {
    fn deserialize(_deserializer: &mut Deserializer) -> Result<Self, SerializationError> {
        // @TODO
        Ok(RandomizerType::Random7Bag.build(0))
    }
}

impl Serialize for PieceType {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), SerializationError> {
        match *self {
            PieceType::S => serializer.serialize_packed_u8::<0, 6>(0)?,
            PieceType::Z => serializer.serialize_packed_u8::<0, 6>(1)?,
            PieceType::J => serializer.serialize_packed_u8::<0, 6>(2)?,
            PieceType::L => serializer.serialize_packed_u8::<0, 6>(3)?,
            PieceType::O => serializer.serialize_packed_u8::<0, 6>(4)?,
            PieceType::I => serializer.serialize_packed_u8::<0, 6>(5)?,
            PieceType::T => serializer.serialize_packed_u8::<0, 6>(6)?,
        }
        Ok(())
    }
}

impl Deserialize for PieceType {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self, SerializationError> {
        let t = match deserializer.deserialize_packed_u8::<0, 6>()? {
            0 => PieceType::S,
            1 => PieceType::Z,
            2 => PieceType::J,
            3 => PieceType::L,
            4 => PieceType::O,
            5 => PieceType::I,
            _ => PieceType::T,
        };
        Ok(t)
    }
}

impl Serialize for BlockType {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), SerializationError> {
        match *self {
            BlockType::Piece(piece_type) => piece_type.serialize(serializer)?,
            BlockType::Empty => serializer.serialize_packed_u8::<0, 7>(7)?,
        }
        Ok(())
    }
}

impl Deserialize for BlockType {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self, SerializationError> {
        let t = match deserializer.deserialize_packed_u8::<0, 7>()? {
            // @XXX: this is repeating the PieceType deserialization,
            //       any way to avoid this repetition?
            0 => BlockType::Piece(PieceType::S),
            1 => BlockType::Piece(PieceType::Z),
            2 => BlockType::Piece(PieceType::J),
            3 => BlockType::Piece(PieceType::L),
            4 => BlockType::Piece(PieceType::O),
            5 => BlockType::Piece(PieceType::I),
            6 => BlockType::Piece(PieceType::T),

            _ => BlockType::Empty,
        };
        Ok(t)
    }
}

impl Serialize for Piece {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), SerializationError> {
        self.type_.serialize(serializer)?;
        let rot = ((self.rot % 4) + 4) % 4;
        serializer.serialize_packed_i32::<0, 3>(rot)?;
        self.rotation_system.serialize(serializer)?;
        Ok(())
    }
}

impl Deserialize for Piece {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self, SerializationError> {
        let type_ = PieceType::deserialize(deserializer)?;
        let rot = deserializer.deserialize_packed_i32::<0, 3>()?;
        let rotation_system = RotationSystem::deserialize(deserializer)?;
        Ok(Self { type_, rot, rotation_system })
    }
}

impl Serialize for RotationSystem {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), SerializationError> {
        match *self {
            RotationSystem::Original => serializer.serialize_packed_u8::<0, 6>(0)?,
            RotationSystem::NRSL     => serializer.serialize_packed_u8::<0, 6>(1)?,
            RotationSystem::NRSR     => serializer.serialize_packed_u8::<0, 6>(2)?,
            RotationSystem::Sega     => serializer.serialize_packed_u8::<0, 6>(3)?,
            RotationSystem::ARS      => serializer.serialize_packed_u8::<0, 6>(4)?,
            RotationSystem::SRS      => serializer.serialize_packed_u8::<0, 6>(5)?,
            RotationSystem::DTET     => serializer.serialize_packed_u8::<0, 6>(6)?,
        }
        Ok(())
    }
}

impl Deserialize for RotationSystem {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self, SerializationError> {
        let t = match deserializer.deserialize_packed_u8::<0, 6>()? {
            0 => RotationSystem::Original,
            1 => RotationSystem::NRSL,
            2 => RotationSystem::NRSR,
            3 => RotationSystem::Sega,
            4 => RotationSystem::ARS,
            5 => RotationSystem::SRS,
            _ => RotationSystem::DTET,
        };
        Ok(t)
    }
}

impl Serialize for LockedPiece {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), SerializationError> {
        self.piece.serialize(serializer)?;
        self.pos.serialize(serializer)?;
        self.soft_drop_steps.serialize(serializer)?;
        self.hard_drop_steps.serialize(serializer)?;
        self.last_piece_action.serialize(serializer)?;
        self.lock_piece_result.serialize(serializer)?;
        Ok(())
    }
}

impl Deserialize for LockedPiece {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self, SerializationError> {
        let piece = Piece::deserialize(deserializer)?;
        let pos = Vec2i::deserialize(deserializer)?;
        let soft_drop_steps = u8::deserialize(deserializer)?;
        let hard_drop_steps = u8::deserialize(deserializer)?;
        let last_piece_action = LastPieceAction::deserialize(deserializer)?;
        let lock_piece_result = LockedPieceResult::deserialize(deserializer)?;

        Ok(Self {
            piece,
            pos,
            soft_drop_steps,
            hard_drop_steps,
            last_piece_action,
            lock_piece_result,
        })
    }
}

impl Serialize for LastPieceAction {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), SerializationError> {
        match *self {
            LastPieceAction::Movement => false.serialize(serializer)?,
            LastPieceAction::Rotation => true.serialize(serializer)?,
        }
        Ok(())
    }
}

impl Deserialize for LastPieceAction {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self, SerializationError> {
        let t = match bool::deserialize(deserializer)? {
            false => LastPieceAction::Movement,
            true  => LastPieceAction::Rotation,
        };
        Ok(t)
    }
}

impl Serialize for LockedPieceResult {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), SerializationError> {
        match self {
            LockedPieceResult::Nothing => serializer.serialize_packed_u8::<0, 5>(0)?,

            LockedPieceResult::Single(v) => { serializer.serialize_packed_u8::<0, 5>(1)?; v.serialize(serializer)?; }
            LockedPieceResult::Double(v) => { serializer.serialize_packed_u8::<0, 5>(2)?; v.serialize(serializer)?; }
            LockedPieceResult::Triple(v) => { serializer.serialize_packed_u8::<0, 5>(3)?; v.serialize(serializer)?; }
            LockedPieceResult::Tetris(v) => { serializer.serialize_packed_u8::<0, 5>(4)?; v.serialize(serializer)?; }

            _ =>  serializer.serialize_packed_u8::<0, 5>(5)?,

            /*
            LockedPieceResult::MiniTSpin       => serializer.serialize_packed_u8::<0, 11>(0)?,
            LockedPieceResult::MiniTSpinSingle => serializer.serialize_packed_u8::<0, 11>(0)?,
            LockedPieceResult::MiniTSpinDouble => serializer.serialize_packed_u8::<0, 11>(0)?,
            LockedPieceResult::TSpin           => serializer.serialize_packed_u8::<0, 11>(0)?,
            LockedPieceResult::TSpinSingle     => serializer.serialize_packed_u8::<0, 11>(0)?,
            LockedPieceResult::TSpinDouble     => serializer.serialize_packed_u8::<0, 11>(0)?,
            LockedPieceResult::TSpinTriple     => serializer.serialize_packed_u8::<0, 11>(0)?,
            */
        }
        Ok(())
    }
}

impl Deserialize for LockedPieceResult {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self, SerializationError> {
        let t = match deserializer.deserialize_packed_u8::<0, 5>()? {
            0 => LockedPieceResult::Nothing,

            1 => LockedPieceResult::Single(<[u8; 1]>::deserialize(deserializer)?),
            2 => LockedPieceResult::Double(<[u8; 2]>::deserialize(deserializer)?),
            3 => LockedPieceResult::Triple(<[u8; 3]>::deserialize(deserializer)?),
            4 => LockedPieceResult::Tetris(<[u8; 4]>::deserialize(deserializer)?),

            _ => LockedPieceResult::MiniTSpin,
        };
        Ok(t)
    }
}

/*
/*
// Rules
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
    pub spawn_drop: bool, // "Immediately drop one space if no existing Block is in its path"

    // @Maybe these are just related to spawning before entry delay
    pub has_initial_rotation_system: bool, // IRS
    pub has_initial_hold_system: bool,     // IHS

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
}
*/

impl Serialize for Rules {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), SerializationError> {
    }
}

impl Deserialize for Rules {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self, SerializationError> {
    }
}
*/
