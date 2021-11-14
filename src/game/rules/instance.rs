use crate::BLOCK_SCALE;
use crate::app::*;
use crate::linalg::*;

use crate::game::{
    pieces::{get_piece_type_color, Piece, PieceType },
    playfield::{ Playfield, PLAYFIELD_VISIBLE_HEIGHT },
    randomizer::*,
    render::*,
    rules::{
        LockDelayRule,
        Rules,
        line_clear::*,
        lock::*,
        movement::*,
        rotation::*,
        scoring::*,
        topout::*,
    },
    scenes::PersistentData,
};

pub const NEXT_PIECES_COUNT: usize = 8;

#[derive(Clone, Debug, ImDraw)]
// @Rename
pub struct RulesInstance {
    // @Maybe add which topout rule was the cause
    has_topped_out: bool, // per game

    rules: Rules,           // per game
    playfield: Playfield,   // per game
    randomizer: Randomizer, // per game

    current_score: u32,       // per game
    total_lines_cleared: u32, // per game

    current_piece: Option<(Piece, Vec2i)>,
    next_piece_types: [PieceType; NEXT_PIECES_COUNT], // per game

    lock_piece_timestamp: u64,
    last_locked_piece: Option<LockedPiece>, // per piece
    soft_drop_steps: u8, // per piece
    hard_drop_steps: u8, // per piece

    hold_piece: Option<Piece>, // per game
    has_used_hold: bool, // per piece

    is_locking: bool, // per piece
    remaining_lock_delay: LockDelayRule, // per piece

    has_moved: bool,   // per frame
    has_rotated: bool, // per frame
    has_stepped: bool, // per frame
    last_piece_action: LastPieceAction, // per frame

    // Timestamps used for animations
    movement_last_timestamp_x: u64,
    movement_last_timestamp_y: u64,

    // Render info
    // @Refactor render positions should be in the scene
    preview_window_delta_pos: Vec2,
    hold_window_delta_pos: Vec2,

    // @Maybe split animation data into another struct. This will for sure be modified when styles
    //        are implemented
    // Animations
    has_movement_animation: bool,
    movement_animation_show_ghost: bool,
    movement_animation_duration: u64,
    movement_animation_delta_grid_x: f32,
    movement_animation_delta_grid_y: f32,
    movement_animation_current_delta_grid: Vec2,

    has_line_clear_animation: bool,
    line_clear_animation_type: LineClearAnimationType,

    has_locking_animation: bool,
    locking_animation_duration: u64,
    locking_animation_timestamp: u64,
}

impl RulesInstance {
    pub fn rules(&self) -> &Rules           { &self.rules }
    pub fn playfield(&self) -> &Playfield   { &self.playfield }
    pub fn randomizer(&self) -> &Randomizer { &self.randomizer }
}

impl RulesInstance {
    pub fn new(
        rules: Rules,
        seed: u64,
        app: &mut App,
        persistent: &mut PersistentData
    ) -> Self {
        let pixel_scale = persistent.pixel_scale;

        let window_size = app.video_system.window.size();

        // playfield positioning
        let playfield_grid_size = Vec2i { x: 10, y: 40 };

        // @Refactor render positions should be in the scene
        let playfield_pixel_size = Vec2i {
            x: (pixel_scale as f32 * BLOCK_SCALE * playfield_grid_size.x as f32) as i32,
            y: (pixel_scale as f32 * BLOCK_SCALE * PLAYFIELD_VISIBLE_HEIGHT as f32) as i32,
        };

        // @Refactor render positions should be in the scene
        let playfield_pos = Vec2i {
            x: (window_size.0 as i32 - playfield_pixel_size.x) / 2,
            y: (window_size.1 as i32 - playfield_pixel_size.y) / 2,
        };

        let playfield = Playfield::new(playfield_pos, playfield_grid_size, true);

        // rng
        let mut randomizer: Randomizer = RandomizerType::Random7Bag.build(seed);

        // next pieces preview window
        let mut next_piece_types = [PieceType::I; NEXT_PIECES_COUNT];
        for i in 0..NEXT_PIECES_COUNT {
            next_piece_types[i] = randomizer.next_piece();
        }

        // @Refactor render positions should be in the scene
        let preview_window_delta_pos = Vec2 { x: 20.0, y: 0.0 };

        // hold window
        // @Refactor render positions should be in the scene
        let hold_window_delta_pos = Vec2 { x: -20.0, y: 0.0 };

        // lock delay
        let remaining_lock_delay = rules.lock_delay;

        //
        let has_movement_animation = rules.has_movement_animation;
        let movement_animation_show_ghost = rules.movement_animation_show_ghost;
        let movement_animation_duration = rules.movement_animation_duration;

        let has_line_clear_animation = rules.has_line_clear_animation;
        let line_clear_animation_type = rules.line_clear_animation_type;

        let has_locking_animation = rules.has_locking_animation;
        let locking_animation_duration = rules.locking_animation_duration;

        Self {
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

            preview_window_delta_pos,
            hold_window_delta_pos,

            movement_last_timestamp_x: app.game_timestamp(),
            movement_last_timestamp_y: app.game_timestamp(),

            has_movement_animation,
            movement_animation_show_ghost,
            movement_animation_duration,
            movement_animation_delta_grid_x: 0.0,
            movement_animation_delta_grid_y: 0.0,
            movement_animation_current_delta_grid: Vec2::new(),

            has_line_clear_animation,
            line_clear_animation_type,

            has_locking_animation,
            locking_animation_duration,
            locking_animation_timestamp: 0,
        }
    }

    pub fn update(
        &mut self,
        app: &mut App,
        input_mapping: &InputMapping,
    ) -> bool {
        if self.has_topped_out { return false; }

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
                        *duration = duration.saturating_sub(app.last_frame_timestamp());
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
                        *duration = duration.saturating_sub(app.last_frame_timestamp());
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
                            *duration = duration.saturating_sub(app.last_frame_timestamp());
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
                self.lock_piece(app);
                has_updated = true;
            }

            // animation
            if !was_locking && self.is_locking {
                self.locking_animation_timestamp = app.game_timestamp();
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

            let left_button = input_mapping.button("left".to_string());
            if left_button.pressed_repeat_with_delay(
                self.rules.das_repeat_delay,
                self.rules.das_repeat_interval,
                app
            ) {
                horizontal_movement -= 1;
            }

            let right_button = input_mapping.button("right".to_string());
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
                self.movement_last_timestamp_x = app.game_timestamp();
                self.movement_animation_delta_grid_x =
                    self.movement_animation_current_delta_grid.x - horizontal_movement as f32;

                self.has_moved = true;
                self.last_piece_action = LastPieceAction::Movement;

                has_updated = true;
            }

            // Soft drop
            let down_button = input_mapping.button("down".to_string());
            if down_button.pressed_repeat(self.rules.soft_drop_interval, app) {
                if self.try_soft_drop_piece(app) {
                    has_updated = true;
                }
            }

            // Rotate
            let mut rotation = 0;

            let ccw_button = input_mapping.button("rotate_ccw".to_string());
            if ccw_button.pressed() { rotation -= 1; }

            let cw_button = input_mapping.button("rotate_cw".to_string());
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
            let up_button = input_mapping.button("hard_drop".to_string());
            if up_button.pressed() {
                if self.try_hard_drop_piece(app) {
                    has_updated = true;
                }
            }
        }

        // Hold piece
        if self.rules.has_hold_piece && self.current_piece.is_some() {
            let hold_button = input_mapping.button("hold".to_string());
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
                            self.movement_last_timestamp_x = app.game_timestamp();
                            self.movement_last_timestamp_y = app.game_timestamp();
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
            let gravity_interval = self.rules.get_gravity_interval(self.level());
            if app.game_timestamp() >= self.movement_last_timestamp_y + gravity_interval {
                let (piece, piece_pos) = self.current_piece.as_mut().unwrap();
                if try_apply_gravity(
                    piece,
                    piece_pos,
                    &self.playfield,
                ) {
                    // Gravity move successful

                    self.has_stepped = true;

                    self.movement_last_timestamp_y = app.game_timestamp();
                    self.movement_animation_delta_grid_y = self.movement_animation_current_delta_grid.y + 1.0;

                    has_updated = true;
                } else {
                    // Piece blocked: lock piece

                    // Only lock on gravity movement if there's no lock delay
                    if self.rules.lock_delay == LockDelayRule::NoDelay {
                        self.lock_piece(app);
                        has_updated = true;
                    }
                }
            }
        }

        // Line clear
        let can_spawn_new_piece;
        if app.game_timestamp() >= self.lock_piece_timestamp + self.rules.line_clear_delay {
            if self.rules.try_clear_lines(&mut self.playfield) {
                self.update_score_and_line_cleared();
                has_updated = true;
            }
            can_spawn_new_piece = true;
        } else {
            can_spawn_new_piece = false;
        }

        // New piece
        if self.current_piece.is_none() &&
            can_spawn_new_piece &&
            app.game_timestamp() >= self.lock_piece_timestamp + self.rules.entry_delay
        {
            self.new_piece();

            self.movement_last_timestamp_x = app.game_timestamp();
            self.movement_last_timestamp_y = app.game_timestamp();

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
                ) > 0 {
                    let (piece, piece_pos) = self.current_piece.as_mut().unwrap();
                    if try_apply_gravity(
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

    fn update_animations(
        &mut self,
        app: &mut App
    ) {
        // Movement animation
        if self.has_movement_animation {
            if app.game_timestamp() <= self.movement_last_timestamp_x + self.movement_animation_duration {
                let t = norm_u64(
                    app.game_timestamp(),
                    self.movement_last_timestamp_x,
                    self.movement_last_timestamp_x  + self.movement_animation_duration
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

            if app.game_timestamp() <= self.movement_last_timestamp_y + self.movement_animation_duration {
                let t = norm_u64(
                    app.game_timestamp(),
                    self.movement_last_timestamp_y,
                    self.movement_last_timestamp_y  + self.movement_animation_duration
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

    // @TODO split rendering parts
    pub fn render_playfield(
        &mut self,
        //pos: Vec2i,
        app: &mut App,
        persistent: &mut PersistentData
    ) {
        let pixel_scale = persistent.pixel_scale;

        // @Refactor render positions should be in the scene
        let window_size = app.video_system.window.size();
        let playfield_pixel_size = Vec2i {
            x: (pixel_scale as f32 * BLOCK_SCALE * self.playfield.grid_size.x as f32) as i32,
            y: (pixel_scale as f32 * BLOCK_SCALE * PLAYFIELD_VISIBLE_HEIGHT as f32) as i32,
        };
        self.playfield.pos = Vec2i {
            x: (window_size.0 as i32 - playfield_pixel_size.x) / 2,
            y: (window_size.1 as i32 - playfield_pixel_size.y) / 2,
        };

        // playfield
        let playfield_size = get_draw_playfield_size(&self.playfield, persistent.pixel_scale);

        // @Temporary recalculate playfield position since has_grid can change
        let window_size = app.video_system.window.size();
        self.playfield.pos = Vec2i {
            x: (window_size.0 as f32 - playfield_size.x) as i32 / 2,
            y: (window_size.1 as f32 - playfield_size.y) as i32 / 2,
        };

        // Update animations
        self.update_animations(app);

        if self.has_line_clear_animation {
            // Get line clear info
            let lines_to_clear = match self.last_locked_piece {
                None => &[],
                Some(LockedPiece { ref lock_piece_result, .. })
                    => lock_piece_result.get_lines_to_clear_slice(),
            };

            draw_playfield(
                &self.playfield,
                Some((
                    self.lock_piece_timestamp,
                    self.rules.line_clear_delay,
                    self.line_clear_animation_type,
                    lines_to_clear,
                )),
                self.rules.rotation_system,
                app,
                persistent
            );
        } else {
            draw_playfield(
                &self.playfield,
                None,
                self.rules.rotation_system,
                app,
                persistent
            );
        }

        // ghost piece
        if let Some(piece) = self.current_piece {
            let (piece, piece_pos) = piece;
            if self.movement_animation_show_ghost {
                draw_piece_in_playfield(
                    piece,
                    piece_pos,
                    Vec2::new(),
                    Color { r: 1., g: 1., b: 1., a: 0.1 },
                    &self.playfield,
                    app,
                    persistent
                );
            }

            // render ghost piece
            if self.rules.has_ghost_piece {
                // @TODO cache the ghost piece and only recalculate the position when piece moves
                let mut ghost_piece = piece.clone();
                let mut ghost_piece_pos = piece_pos;

                // Only render if ghost is not on top of the current piece
                // @Maybe improve this and just draw the different pixels. Since there's piece
                //        movement animation, not rendering the ghost in the middle of the piece
                //        movement may be visually weird
                if full_drop_piece(
                    &mut ghost_piece,
                    &mut ghost_piece_pos,
                    &self.playfield,
                ) > 0 {
                    draw_piece_in_playfield(
                        ghost_piece,
                        ghost_piece_pos,
                        Vec2::new(),
                        Color { r: 1., g: 1., b: 1., a: 0.1 },
                        &self.playfield,
                        app,
                        persistent
                    );
                }
            }

            // render piece
            let movement_animation_delta_grid;
            if self.has_movement_animation {
                movement_animation_delta_grid = self.movement_animation_current_delta_grid;
            } else {
                movement_animation_delta_grid = Vec2::new();
            }

            let color;
            if self.is_locking && self.has_locking_animation {
                let delta_time = app.game_timestamp() - self.locking_animation_timestamp;
                let alpha = 2.0 * std::f32::consts::PI * delta_time as f32;
                let alpha = alpha / (self.locking_animation_duration as f32);
                let alpha = ((1.0 + alpha.sin()) / 2.0) / 2.0 + 0.5;

                color = piece.color().alpha(alpha);
            } else {
                color = piece.color();
            }

            draw_piece_in_playfield(
                piece,
                piece_pos,
                movement_animation_delta_grid,
                color,
                &self.playfield,
                app,
                persistent
            );
        }
    }

    pub fn render_next_pieces_preview(
        &mut self,
        app: &mut App,
        persistent: &mut PersistentData
    ) {
        // render preview
        if self.rules.next_pieces_preview_count > 0 {
            let size;
            if self.playfield.has_grid {
                size = persistent.pixel_scale as f32 * ((1.0 + BLOCK_SCALE) * 4.0 + 1.0);
            } else {
                size = persistent.pixel_scale as f32 * BLOCK_SCALE * 4.0;
            }
            let window_size = Vec2 {
                x: size,
                y: size * self.rules.next_pieces_preview_count as f32,
            };

            let playfield_size = get_draw_playfield_size(&self.playfield, persistent.pixel_scale);

            let window_pos =
                Vec2::from(self.playfield.pos) + self.preview_window_delta_pos +
                Vec2 { x: playfield_size.x, y: 0.0 };

            draw_rect_window(
                window_pos,
                window_size,
                persistent.pixel_scale,
                app,
                persistent
            );

            assert!(self.rules.next_pieces_preview_count <= 8);
            for i in 0..self.rules.next_pieces_preview_count {
                draw_piece_centered(
                    Piece {
                        type_: self.next_piece_types[i as usize],
                        rot: 0,
                        rotation_system: self.rules.rotation_system,
                    },
                    Vec2 {
                        x: window_pos.x,
                        y: window_pos.y + i as f32 * size,
                    },
                    get_piece_type_color(self.next_piece_types[i as usize], self.rules.rotation_system),
                    self.playfield.has_grid,
                    app,
                    persistent
                );
            }
        }
    }

    pub fn render_hold_piece(
        &mut self,
        app: &mut App,
        persistent: &mut PersistentData
    ) {
        // hold piece
        if self.rules.has_hold_piece {
            let window_size;
            if self.playfield.has_grid {
                let size = persistent.pixel_scale as f32 * ((1.0 + BLOCK_SCALE) * 4.0 + 1.0);
                window_size = Vec2 { x: size as f32, y: size as f32 };
            } else {
                let size = persistent.pixel_scale as f32 * BLOCK_SCALE * 4.0;
                window_size = Vec2 { x: size as f32, y: size as f32 };
            }

            let window_pos =
                Vec2::from(self.playfield.pos) + self.hold_window_delta_pos +
                Vec2 { x: -window_size.x, y: 0.0 };

            draw_rect_window(
                window_pos,
                window_size,
                persistent.pixel_scale,
                app,
                persistent
            );

            if let Some(hold_piece) = self.hold_piece {
                draw_piece_centered(
                    hold_piece,
                    window_pos,
                    hold_piece.color(),
                    self.playfield.has_grid,
                    app,
                    persistent
                );
            }
        }
    }

    pub fn render(
        &mut self,
        app: &mut App,
        persistent: &mut PersistentData
    ) {
        self.render_playfield(app, persistent);
        self.render_next_pieces_preview(app, persistent);
        self.render_hold_piece(app, persistent);
    }

    fn new_piece(&mut self) {
        // update piece position and type

        let new_piece = Piece {
            type_: self.next_piece_types[0],
            rot: 0,
            rotation_system: self.rules.rotation_system,
        };

        let new_piece_pos = Vec2i {
            x: self.playfield.grid_size.x / 2 - 2,
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

    fn lock_piece(&mut self, app: &App) {
        let (piece, piece_pos) = self.current_piece.as_ref().unwrap();
        lock_piece(
            piece,
            *piece_pos,
            &mut self.playfield,
        );

        // @Refactor this is repeated and any lock piece should check for this.
        if locked_out(piece, *piece_pos, &self.rules) {
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

        self.lock_piece_timestamp = app.game_timestamp();
    }

    fn try_hard_drop_piece(&mut self, app: &App) -> bool {
        if !self.rules.has_hard_drop { return false; }

        let (piece, piece_pos) = self.current_piece.as_mut().unwrap();
        self.hard_drop_steps = full_drop_piece(piece, piece_pos, &mut self.playfield);
        self.last_piece_action = LastPieceAction::Movement;

        self.lock_piece(app);
        true
    }

    fn try_soft_drop_piece(&mut self, app: &App) -> bool {
        if !self.rules.has_soft_drop { return false; }

        let (piece, piece_pos) = self.current_piece.as_mut().unwrap();
        if try_move_piece(piece, piece_pos, &self.playfield, 0, -1) {
            self.movement_last_timestamp_y = app.game_timestamp();
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

// Network
use crate::game::network;

impl RulesInstance {
    pub fn from_network(
        net_instance: network::NetworkedRulesInstance,
        rules: Rules,
        randomizer: Randomizer,
        net_timestamp: u64,
        app: &mut App,
        persistent: &mut PersistentData
    ) -> Self {
        let pixel_scale = persistent.pixel_scale;

        let window_size = app.video_system.window.size();

        // playfield positioning
        let playfield_grid_size = net_instance.playfield.grid_size;

        // @Refactor render positions should be in the scene
        let playfield_pixel_size = Vec2i {
            x: (pixel_scale as f32 * BLOCK_SCALE * playfield_grid_size.x as f32) as i32,
            y: (pixel_scale as f32 * BLOCK_SCALE * PLAYFIELD_VISIBLE_HEIGHT as f32) as i32,
        };

        // @Refactor render positions should be in the scene
        let playfield_pos = Vec2i {
            x: (window_size.0 as i32 - playfield_pixel_size.x) / 2,
            y: (window_size.1 as i32 - playfield_pixel_size.y) / 2,
        };

        let playfield = Playfield::from_network(net_instance.playfield, playfield_pos, true);

        // @Refactor render positions should be in the scene
        let preview_window_delta_pos = Vec2 { x: 20.0, y: 0.0 };

        // hold window
        // @Refactor render positions should be in the scene
        let hold_window_delta_pos = Vec2 { x: -20.0, y: 0.0 };

        // lock delay
        let remaining_lock_delay = rules.lock_delay;

        // Fix timestamps
        app.set_game_timestamp(net_timestamp);

        Self {
            has_topped_out: net_instance.has_topped_out,

            playfield,
            rules,
            randomizer,

            current_score: net_instance.current_score,
            total_lines_cleared: net_instance.total_lines_cleared,

            current_piece: net_instance.current_piece,
            next_piece_types: net_instance.next_piece_types,

            lock_piece_timestamp: net_instance.lock_piece_timestamp,
            last_locked_piece: net_instance.last_locked_piece,
            soft_drop_steps: 0,
            hard_drop_steps: 0,

            hold_piece: net_instance.hold_piece,
            has_used_hold: false,

            is_locking: false,
            remaining_lock_delay,

            has_moved: false,
            has_rotated: false,
            has_stepped: false,
            last_piece_action: LastPieceAction::Movement,

            preview_window_delta_pos,
            hold_window_delta_pos,

            movement_last_timestamp_x: net_instance.movement_last_timestamp_x,
            movement_last_timestamp_y: net_instance.movement_last_timestamp_y,

            has_movement_animation: true,
            movement_animation_show_ghost: false,
            movement_animation_duration: 50_000,
            movement_animation_delta_grid_x: 0.0,
            movement_animation_delta_grid_y: 0.0,
            movement_animation_current_delta_grid: Vec2::new(),

            has_line_clear_animation: true,
            line_clear_animation_type: LineClearAnimationType::Classic,

            has_locking_animation: true,
            locking_animation_timestamp: 0,
            locking_animation_duration: 250_000,
        }
    }

    pub fn to_network(&self) -> network::NetworkedRulesInstance {
        network::NetworkedRulesInstance {
            has_topped_out: self.has_topped_out,
            playfield: Playfield::to_network(&self.playfield),

            current_score: self.current_score,
            total_lines_cleared: self.total_lines_cleared,

            current_piece: self.current_piece,
            next_piece_types: self.next_piece_types.clone(),

            lock_piece_timestamp: self.lock_piece_timestamp,
            last_locked_piece: self.last_locked_piece,

            hold_piece: self.hold_piece,

            movement_last_timestamp_x: self.movement_last_timestamp_x,
            movement_last_timestamp_y: self.movement_last_timestamp_y,
        }
    }

    pub fn update_from_network(
        &mut self,
        net_instance: network::NetworkedRulesInstance,
        net_timestamp: u64,
        app: &mut App,
    ) {
        // Fix timestamps
        app.set_game_timestamp(net_timestamp);

        self.has_topped_out = net_instance.has_topped_out;
        self.playfield.update_from_network(net_instance.playfield);

        self.current_score = net_instance.current_score;
        self.total_lines_cleared = net_instance.total_lines_cleared;

        self.current_piece = net_instance.current_piece;
        self.next_piece_types = net_instance.next_piece_types;

        self.lock_piece_timestamp = net_instance.lock_piece_timestamp;
        self.last_locked_piece = net_instance.last_locked_piece;

        self.hold_piece = net_instance.hold_piece;

        self.movement_last_timestamp_x = net_instance.movement_last_timestamp_x;
        self.movement_last_timestamp_y = net_instance.movement_last_timestamp_y;
    }
}
