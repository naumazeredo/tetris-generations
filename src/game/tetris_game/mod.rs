use crate::BLOCK_SCALE;
use crate::app::*;
use crate::linalg::*;

use crate::game::{
    input::*,
    pieces::{get_piece_variant_color, Piece, PieceVariant},
    playfield::{PLAYFIELD_VISIBLE_HEIGHT, Playfield},
    randomizer::*,
    render::*,
    rules::{
        LockDelayRule,
        Rules,
        lock::*,
        movement::*,
        rotation::*,
        scoring::*,
        topout::*,
    },
    scenes::PersistentData,
};

mod network;
mod layout;

pub use network::*;
pub use layout::*;

pub const NEXT_PIECES_COUNT: usize = 8;

#[derive(Clone, Debug, ImDraw)]
// @Rename again?
pub struct TetrisGame {
    timestamp: u64, // @TODO Time

    // @Maybe add which topout rule was the cause
    has_topped_out: bool, // per game

    rules: Rules,           // per game
    playfield: Playfield,   // per game
    randomizer: Randomizer, // per game

    current_score: u32,       // per game
    total_lines_cleared: u32, // per game

    current_piece: Option<(Piece, Vec2i)>,
    next_piece_types: [PieceVariant; NEXT_PIECES_COUNT], // per game

    lock_piece_timestamp: u64,
    last_locked_piece: Option<LockedPiece>, // per piece
    soft_drop_steps: u8, // per piece
    hard_drop_steps: u8, // per piece

    hold_piece: Option<Piece>, // per game
    has_used_hold: bool, // per piece

    is_locking: bool, // per piece
    remaining_lock_delay: LockDelayRule, // per piece // @Maybe we should make this count up to not copy on construction

    has_moved: bool,   // per frame
    has_rotated: bool, // per frame
    has_stepped: bool, // per frame
    last_piece_action: LastPieceAction, // per frame

    // Movement timestamps. Used for gravity and animations
    movement_last_timestamp_x: u64,
    movement_last_timestamp_y: u64,

    // @Maybe split animation data into another struct. This will for sure be modified when styles
    //        are implemented
    // Animations
    movement_animation_delta_grid_x: f32,
    movement_animation_delta_grid_y: f32,
    movement_animation_current_delta_grid: Vec2,

    locking_animation_timestamp: u64,

    // @Maybe this should be part of the game, but it depends on window size, pixel scale, etc
    //layout: TetrisLayout,
}

impl TetrisGame {
    pub fn timestamp(&self)  -> u64         { self.timestamp }
    pub fn rules(&self)      -> &Rules      { &self.rules }
    pub fn playfield(&self)  -> &Playfield  { &self.playfield }
    pub fn randomizer(&self) -> &Randomizer { &self.randomizer }
}

impl TetrisGame {
    pub fn new(
        rules: Rules,
        seed: u64,
    ) -> Self {
        let playfield_grid_size = Vec2i { x: 10, y: 40 };
        let playfield = Playfield::new(playfield_grid_size, PLAYFIELD_VISIBLE_HEIGHT);

        // rng
        let mut randomizer: Randomizer = rules.randomizer_type.build(seed);

        // next pieces preview window
        let mut next_piece_types = [PieceVariant::I; NEXT_PIECES_COUNT];
        for i in 0..NEXT_PIECES_COUNT {
            next_piece_types[i] = randomizer.next_piece();
        }

        // lock delay
        let remaining_lock_delay = rules.lock_delay;

        Self {
            timestamp: 0,

            has_topped_out: false,

            playfield,
            rules,
            randomizer,

            current_score: 0,
            total_lines_cleared: 0,

            current_piece: None,
            next_piece_types,

            lock_piece_timestamp: 0,
            last_locked_piece: None,
            soft_drop_steps: 0,
            hard_drop_steps: 0,

            hold_piece: None,
            has_used_hold: false,

            is_locking: false,
            remaining_lock_delay,

            has_moved: false,
            has_rotated: false,
            has_stepped: false,
            last_piece_action: LastPieceAction::Movement,

            movement_last_timestamp_x: 0,
            movement_last_timestamp_y: 0,

            movement_animation_delta_grid_x: 0.0,
            movement_animation_delta_grid_y: 0.0,
            movement_animation_current_delta_grid: Vec2::new(),

            locking_animation_timestamp: 0,
        }
    }

    pub fn new_preview(
        rules: Rules,
        playfield: Playfield,
        mut randomizer: Randomizer,
    ) -> Self {
        // next pieces preview window
        let mut next_piece_types = [PieceVariant::I; NEXT_PIECES_COUNT];
        for i in 0..NEXT_PIECES_COUNT {
            next_piece_types[i] = randomizer.next_piece();
        }

        // lock delay
        let remaining_lock_delay = rules.lock_delay;

        //

        Self {
            timestamp: 0,

            has_topped_out: false,

            playfield,
            rules,
            randomizer,

            current_score: 0,
            total_lines_cleared: 0,

            current_piece: None,
            next_piece_types,

            lock_piece_timestamp: 0,
            last_locked_piece: None,
            soft_drop_steps: 0,
            hard_drop_steps: 0,

            hold_piece: None,
            has_used_hold: false,

            is_locking: false,
            remaining_lock_delay,

            has_moved: false,
            has_rotated: false,
            has_stepped: false,
            last_piece_action: LastPieceAction::Movement,

            movement_last_timestamp_x: 0,
            movement_last_timestamp_y: 0,

            movement_animation_delta_grid_x: 0.0,
            movement_animation_delta_grid_y: 0.0,
            movement_animation_current_delta_grid: Vec2::new(),

            locking_animation_timestamp: 0,
        }
    }

    pub fn update<M: InputMapping>(
        &mut self,
        dt: u64, // @TODO Duration
        input_mapping: &M,
        app: &mut App, // @Remove only used to check input repeated over time, which should be improved to not need the whole app
    ) -> bool {
        if self.has_topped_out { return false; }

        self.timestamp += dt;

        let mut has_updated = false;

        // locking
        // This is done in the start of the frame to be just and consider the time the piece is
        // locking is the last frame duration. If the piece locks, all input will be ignored this
        // frame, even if there's no entry delay
        if self.current_piece.is_some() && self.rules.lock_delay != LockDelayRule::NoDelay {
            let was_locking = self.is_locking;
            self.is_locking = is_piece_locking(
                &self.current_piece.as_ref().unwrap().0,
                self.current_piece.as_ref().unwrap().1,
                &self.playfield,
            );

            let has_locked = match self.remaining_lock_delay {
                // Locking duration resets when a new piece enters
                LockDelayRule::EntryReset(ref mut duration) => {
                    if was_locking && self.is_locking {
                        *duration = duration.saturating_sub(dt);
                    }

                    *duration == 0
                },

                // Every step (gravity movement) resets the locking duration
                LockDelayRule::StepReset(ref mut duration) => {
                    if self.has_stepped {
                        match self.rules.lock_delay {
                            LockDelayRule::StepReset(lock_duration) => {
                                *duration = lock_duration;
                            }
                            _ => unreachable!(),
                        }
                    }

                    if self.is_locking {
                        *duration = duration.saturating_sub(dt);
                        *duration == 0
                    } else {
                        false
                    }
                },

                // Every rotation and movement (not gravity movement), when piece is in locking
                // position, resets the locking duration.
                // There's a limit to movements and rotations.
                LockDelayRule::MoveReset {
                    ref mut duration,
                    ref mut rotations,
                    ref mut movements,
                } => {
                    // Only reset duration if has movements/rotations left
                    if (self.has_moved && *movements > 0) || (self.has_rotated && *rotations > 0) {
                        match self.rules.lock_delay {
                            LockDelayRule::MoveReset { duration: lock_duration, .. } => {
                                *duration = lock_duration;
                            }
                            _ => unreachable!(),
                        }
                    };

                    if self.is_locking {
                        if !self.has_moved && !self.has_rotated {
                            *duration = duration.saturating_sub(dt);
                        }

                        if self.has_moved   { *movements = movements.saturating_sub(1); }
                        if self.has_rotated { *rotations = rotations.saturating_sub(1); }

                        *duration == 0
                    } else {
                        false
                    }
                },

                _ => { false }
            };

            if has_locked {
                self.lock_piece();
                has_updated = true;
            }

            // animation
            if !was_locking && self.is_locking {
                self.locking_animation_timestamp = self.timestamp;
                has_updated = true;
            }
        } else {
            self.is_locking = false;
        }

        // reset frame values
        self.has_moved = false;
        self.has_rotated = false;
        self.has_stepped = false;

        // movement input
        if self.current_piece.is_some() {
            // Horizontal movement logic
            let mut horizontal_movement = 0;

            let left_button = input_mapping.button(KEY_LEFT.to_string());
            if left_button.pressed_repeat_with_delay(
                self.rules.das_repeat_delay,
                self.rules.das_repeat_interval,
                app
            ) {
                horizontal_movement -= 1;
            }

            let right_button = input_mapping.button(KEY_RIGHT.to_string());
            if right_button.pressed_repeat_with_delay(
                self.rules.das_repeat_delay,
                self.rules.das_repeat_interval,
                app
            ) {
                horizontal_movement += 1;
            }

            let (piece, piece_pos) = self.current_piece.as_mut().unwrap();
            if horizontal_movement != 0 && try_move_piece(
                piece,
                piece_pos,
                &self.playfield,
                horizontal_movement,
                0,
            ) {
                self.movement_last_timestamp_x = self.timestamp;
                self.movement_animation_delta_grid_x =
                    self.movement_animation_current_delta_grid.x - horizontal_movement as f32;

                self.has_moved = true;
                self.last_piece_action = LastPieceAction::Movement;

                has_updated = true;
            }

            // Soft drop
            let down_button = input_mapping.button(KEY_SOFT_DROP.to_string());
            if down_button.pressed_repeat(self.rules.soft_drop_interval, app) {
                if self.try_soft_drop_piece() {
                    has_updated = true;
                }
            }

            // Rotate
            let mut rotation = 0;

            let ccw_button = input_mapping.button(KEY_ROTATE_CCW.to_string());
            if ccw_button.pressed() { rotation -= 1; }

            let cw_button = input_mapping.button(KEY_ROTATE_CW.to_string());
            if cw_button.pressed() { rotation += 1; }

            if rotation != 0 {
                if let Some(ref mut piece) = self.current_piece {
                    if try_rotate_piece(
                        &mut piece.0,
                        &mut piece.1,
                        rotation > 0,
                        &self.playfield,
                        &self.rules
                    ) {
                        self.has_rotated = true;
                        self.last_piece_action = LastPieceAction::Rotation;
                        // @TODO soft drop scoring

                        has_updated = true;
                    }
                }
            }
        }

        //
        // The next three mechanics can remove the current piece, so we have to isolate them and
        // verify again if the current piece is available or not
        //

        // Hard drop
        if self.current_piece.is_some() {
            let hard_drop_button = input_mapping.button(KEY_HARD_DROP.to_string());
            if hard_drop_button.pressed() { // @TODO DAS
                if self.try_hard_drop_piece() {
                    has_updated = true;
                }
            }
        }

        // Hold piece
        if self.rules.has_hold_piece && self.current_piece.is_some() {
            let hold_button = input_mapping.button(KEY_HOLD.to_string());
            if hold_button.pressed() {
                if !self.has_used_hold {
                    match self.hold_piece.take() {
                        Some(hold_piece) => {
                            let (piece, piece_pos) = &mut self.current_piece.as_mut().unwrap();

                            if self.rules.hold_piece_reset_rotation {
                                piece.rot = 0;
                            }

                            self.hold_piece = Some(*piece);

                            *piece = hold_piece;
                            *piece_pos = Vec2i {
                                x: self.playfield.grid_size.x / 2 - 2,
                                y: self.rules.spawn_row as i32 - 3,
                            };

                            self.has_used_hold = true;

                            // update movement timestamps
                            self.movement_last_timestamp_x = self.timestamp;
                            self.movement_last_timestamp_y = self.timestamp;
                        }

                        None => {
                            let mut piece = self.current_piece.as_mut().unwrap().0;
                            if self.rules.hold_piece_reset_rotation {
                                piece.rot = 0;
                            }
                            self.hold_piece = Some(piece);

                            // since this is not locking a piece (won't trigger animation or ARE),
                            // so we don't update the lock timestamp.
                            self.current_piece = None;
                        }
                    }

                    has_updated = true;
                }
            }
        }

        // Gravity
        if self.current_piece.is_some() {
            // @TODO move this to Rules (or something)
            if let Some(gravity_interval) = self.rules.get_gravity_interval(self.level()) {
                if self.timestamp >= self.movement_last_timestamp_y + gravity_interval {
                    let (piece, piece_pos) = self.current_piece.as_mut().unwrap();
                    if try_apply_gravity(
                        piece,
                        piece_pos,
                        &self.playfield,
                    ) {
                        // Gravity move successful

                        self.has_stepped = true;

                        self.movement_last_timestamp_y = self.timestamp;
                        self.movement_animation_delta_grid_y = self.movement_animation_current_delta_grid.y + 1.0;

                        has_updated = true;
                    } else {
                        // Piece blocked: lock piece

                        // Only lock on gravity movement if there's no lock delay
                        if self.rules.lock_delay == LockDelayRule::NoDelay {
                            self.lock_piece();
                            has_updated = true;
                        }
                    }
                }
            }
        }

        // Line clear
        let can_spawn_new_piece;
        if let Some(LockedPiece { lock_piece_result, .. }) = self.last_locked_piece {
            if let LockedPieceResult::Nothing = lock_piece_result {
                // No lines to clear, so don't wait the line_clear_delay
                can_spawn_new_piece = true;
            } else {
                // check if line_clear_delay time has passed
                if self.timestamp >= self.lock_piece_timestamp + self.rules.line_clear_delay {
                    self.update_score_and_line_cleared();
                    if self.rules.try_clear_lines(&mut self.playfield) { has_updated = true; }
                    can_spawn_new_piece = true;
                } else {
                    can_spawn_new_piece = false;
                }
            }
        } else {
            can_spawn_new_piece = true;
        }

        // New piece
        if self.current_piece.is_none() &&
            can_spawn_new_piece &&
            // @Fix spawn_delay should be added after line_clear_delay
            self.timestamp >= self.lock_piece_timestamp + self.rules.spawn_delay
        {
            self.new_piece();

            has_updated = true;

            // check for block out
            let (piece, piece_pos) = &self.current_piece.as_ref().unwrap();
            let has_block_out = blocked_out(
                piece,
                *piece_pos,
                &self.playfield,
                &self.rules
            );

            if has_block_out {
                self.has_topped_out = true;
                println!("game over: block out");
                return true;
            }

            // spawn drop
            if self.rules.spawn_drop {
                while blocks_out_of_playfield(
                    &self.current_piece.as_ref().unwrap().0,
                    self.current_piece.as_ref().unwrap().1,
                    self.playfield.visible_height,
                ) > 0 {
                    let (piece, piece_pos) = self.current_piece.as_mut().unwrap();
                    if !try_apply_gravity(
                        piece,
                        piece_pos,
                        &self.playfield,
                    ) {
                        break;
                    }
                }
            }
        }

        has_updated
    }

    // @TODO private
    // @TODO timestamp instead of app
    pub fn update_animations(&mut self) {
        // Movement animation
        if self.rules.has_movement_animation {
            if self.timestamp <= self.movement_last_timestamp_x + self.rules.movement_animation_duration {
                let t = norm_u64(
                    self.timestamp,
                    self.movement_last_timestamp_x,
                    self.movement_last_timestamp_x  + self.rules.movement_animation_duration
                );

                self.movement_animation_current_delta_grid.x = lerp_f32(
                    self.movement_animation_delta_grid_x,
                    0.0,
                    t
                );
            } else {
                self.movement_animation_delta_grid_x = 0.0;
                self.movement_animation_current_delta_grid.x = 0.0;
            }

            if self.timestamp <= self.movement_last_timestamp_y + self.rules.movement_animation_duration {
                let t = norm_u64(
                    self.timestamp,
                    self.movement_last_timestamp_y,
                    self.movement_last_timestamp_y  + self.rules.movement_animation_duration
                );

                self.movement_animation_current_delta_grid.y = lerp_f32(
                    self.movement_animation_delta_grid_y,
                    0.0,
                    t
                );
            } else {
                self.movement_animation_delta_grid_y = 0.0;
                self.movement_animation_current_delta_grid.y = 0.0;
            }
        } else {
            /*
            // @Cleanup this shouldn't be necessary. It's necessary since we can disable the
            //          movement animation in the middle of the game, and we are using these
            //          variables to render
            self.movement_animation_delta_grid_x = 0.0;
            self.movement_animation_delta_grid_y = 0.0;
            self.movement_animation_current_delta_grid.x = 0.0;
            self.movement_animation_current_delta_grid.y = 0.0;
            */
        }
    }

    // Can be used externally to discard the current piece and spawn a new one
    pub fn new_piece(&mut self) {
        // update piece position and type

        let new_piece = Piece {
            variant: self.next_piece_types[0],
            rot: 0,
            rotation_system: self.rules.rotation_system,
        };

        let new_piece_pos = Vec2i {
            x: (self.playfield.grid_size.x + 1) / 2 - 2,
            y: self.rules.spawn_row as i32 - 3,
        };

        self.current_piece = Some((new_piece, new_piece_pos));

        // cycle next pieces
        for i in 0..NEXT_PIECES_COUNT-1 { self.next_piece_types[i] = self.next_piece_types[i+1]; }
        self.next_piece_types[7] = self.randomizer.next_piece();

        // reset per piece data
        self.has_used_hold = false;
        self.remaining_lock_delay = self.rules.lock_delay;
        self.last_locked_piece = None;
        self.soft_drop_steps = 0;
        self.hard_drop_steps = 0;

        // reset movement timestamps
        self.movement_last_timestamp_x = self.timestamp;
        self.movement_last_timestamp_y = self.timestamp;

        // Animations
        // reset movement animation
        self.movement_animation_delta_grid_x = 0.0;
        self.movement_animation_delta_grid_y = 0.0;
        self.movement_animation_current_delta_grid = Vec2::new();
    }

    // This is used as a deferred score update if there's a line clear animation
    fn update_score_and_line_cleared(&mut self) {
        if let Some(locked_piece) = self.last_locked_piece {
            self.current_score += lock_piece_score(
                self.level(),
                locked_piece,
                &self.rules
            );

            self.total_lines_cleared +=
                locked_piece.lock_piece_result.get_lines_to_clear_slice().len() as u32;
        }
    }

    fn get_lock_piece_result(&self) -> LockedPieceResult {
        // @TODO check T-Spins

        let (total_lines_to_clear, lines_to_clear) = self.playfield.get_lines_to_clear();

        match total_lines_to_clear {
            0 => LockedPieceResult::Nothing,
            1 => LockedPieceResult::Single([lines_to_clear[0]]),
            2 => LockedPieceResult::Double([lines_to_clear[0], lines_to_clear[1]]),
            3 => LockedPieceResult::Triple([lines_to_clear[0], lines_to_clear[1], lines_to_clear[2]]),
            4 => LockedPieceResult::Tetris(lines_to_clear),
            _ => unreachable!(),
        }
    }

    fn lock_piece(&mut self) {
        let (piece, piece_pos) = self.current_piece.as_ref().unwrap();
        lock_piece(
            piece,
            *piece_pos,
            &mut self.playfield,
        );

        // @Refactor this is repeated and any lock piece should check for this.
        if locked_out(piece, *piece_pos, self.playfield.visible_height, &self.rules) {
            self.has_topped_out = true;
            println!("game over: locked out");
            return;
        }

        let (piece, piece_pos) = self.current_piece.take().unwrap();
        self.last_locked_piece = Some(LockedPiece {
            piece,
            pos: piece_pos,
            soft_drop_steps: self.soft_drop_steps,
            hard_drop_steps: self.hard_drop_steps,
            last_piece_action: self.last_piece_action,
            lock_piece_result: self.get_lock_piece_result(),
        });

        self.lock_piece_timestamp = self.timestamp;
    }

    fn try_hard_drop_piece(&mut self) -> bool {
        if !self.rules.has_hard_drop { return false; }

        let (piece, piece_pos) = self.current_piece.as_mut().unwrap();
        self.hard_drop_steps = full_drop_piece(piece, piece_pos, &mut self.playfield);
        self.last_piece_action = LastPieceAction::Movement;

        self.lock_piece();
        true
    }

    fn try_soft_drop_piece(&mut self) -> bool {
        if !self.rules.has_soft_drop { return false; }

        let (piece, piece_pos) = self.current_piece.as_mut().unwrap();
        if try_move_piece(piece, piece_pos, &self.playfield, 0, -1) {
            self.movement_last_timestamp_y = self.timestamp;
            self.movement_animation_delta_grid_y =
                self.movement_animation_current_delta_grid.y + 1.0;

            self.has_moved = true;
            self.has_stepped = true;
            self.last_piece_action = LastPieceAction::Movement;

            self.soft_drop_steps += 1;

            true
        } else {
            false
        }
    }


    pub fn playfield_grid_size(&self) -> Vec2i { // -> (u8, u8)
        self.playfield.grid_size
    }

    pub fn level(&self) -> u32 {
        self.rules.get_level(self.current_score, self.total_lines_cleared)
    }

    pub fn score(&self) -> u32 {
        self.current_score
    }

    pub fn total_lines_cleared(&self) -> u32 {
        self.total_lines_cleared
    }
}

/*
// @Maybe use a state machine?
enum TetrisState {
    Falling,
    LineClearAnimation,
}
*/
