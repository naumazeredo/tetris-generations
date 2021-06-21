use crate::BLOCK_SCALE;
use crate::app::*;
use crate::linalg::*;
use crate::State;

use super::*;

use crate::game::{
    randomizer::*,
    rules::{
        Rules,
        RotationSystem,
        LockDelayRule,
        movement::*,
        rotation::*,
        topout::*,
    },
    pieces::{ Piece, PieceType },
    playfield::{ Playfield, PLAYFIELD_VISIBLE_HEIGHT },
    render::*,
};

const NEXT_PIECES_COUNT: usize = 8;

// @Refactor
// This struct is best described as an instantiation of the Rules, where it basically handles all
// everything the rules describes. It's better to just refactor this whole Rules logic into a struct
// with a good naming and clear objective. SinglePlayerScene should just hold the instantiation of
// the Rules and the drawing instead
#[derive(Clone, Debug, ImDraw)]
pub struct SinglePlayerScene {
    debug_pieces_scene_opened: bool,

    // @Maybe add which topout rule was the cause
    topped_out: bool,

    rules: Rules,
    playfield: Playfield,
    randomizer: Randomizer,

    current_piece: Option<Piece>,
    // @Cleanup this seems bad. We use too often this paired with the current_piece, so maybe just
    //          readd it to Piece and make the hold piece the 2 variables (or be a tuple)
    current_piece_pos: Vec2i,
    next_piece_types: [PieceType; NEXT_PIECES_COUNT],
    lock_piece_timestamp: u64,

    hold_piece: Option<Piece>,
    has_used_hold: bool,

    is_locking: bool,
    has_moved: bool,
    has_rotated: bool,
    has_stepped: bool,
    remaining_lock_delay: LockDelayRule,

    preview_window_delta_pos: Vec2,
    hold_window_delta_pos: Vec2,

    movement_last_timestamp_x: u64,
    movement_last_timestamp_y: u64,

    has_movement_animation: bool,
    movement_animation_show_ghost: bool,
    movement_animation_duration: u64,
    movement_animation_delta_grid_x: f32,
    movement_animation_delta_grid_y: f32,
    movement_animation_current_delta_grid: Vec2,

    has_line_clear_animation: bool,
    is_line_clear_animation_playing: bool,
    line_clear_animation_timestamp: u64,
    line_clear_animation_state: LineClearAnimationState,

    locking_animation_timestamp: u64,
    locking_animation_duration: u64,
}

impl SceneTrait for SinglePlayerScene {
    fn update(
        &mut self,
        app: &mut App<'_, State>,
        persistent: &mut PersistentData
    ) {
        // pause
        let options_button = persistent.input_mapping.button("options".to_string());
        if options_button.pressed() {
            if app.is_paused() { app.resume(); }
            else { app.pause(); }
        }

        if app.is_paused() { return; }

        if self.topped_out { return; }

        // locking
        // This is done in the start of the frame to be just and consider the time the piece is
        // locking is the last frame duration. If the piece locks, all input will be ignored this
        // frame, even if there's no entry delay
        if self.current_piece.is_some() && self.rules.lock_delay != LockDelayRule::NoDelay {
            let was_locking = self.is_locking;
            self.is_locking = is_piece_locking(
                self.current_piece.as_ref().unwrap(),
                self.current_piece_pos,
                &self.playfield,
                self.rules.rotation_system
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
                let piece = self.current_piece.as_ref().unwrap();
                lock_piece(
                    piece,
                    self.current_piece_pos,
                    &mut self.playfield,
                    self.rules.rotation_system
                );

                // @Refactor this is repeated and any lock piece should check for this.
                if locked_out(piece, self.current_piece_pos, &self.rules) {
                    self.topped_out = true;
                    println!("game over: locked out");
                    return;
                }

                self.current_piece = None;
                self.lock_piece_timestamp = app.game_timestamp();
            }

            // animation
            if !was_locking && self.is_locking {
                self.locking_animation_timestamp = app.game_timestamp();
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
            // horizontal movement logic
            let mut horizontal_movement = 0;

            let left_button = persistent.input_mapping.button("left".to_string());
            if left_button.pressed_repeat_with_delay(
                self.rules.das_repeat_delay,
                self.rules.das_repeat_interval,
                app
            ) {
                horizontal_movement -= 1;
            }

            let right_button = persistent.input_mapping.button("right".to_string());
            if right_button.pressed_repeat_with_delay(
                self.rules.das_repeat_delay,
                self.rules.das_repeat_interval,
                app
            ) {
                horizontal_movement += 1;
            }

            if horizontal_movement != 0 && try_move_piece(
                self.current_piece.as_ref().unwrap(),
                &mut self.current_piece_pos,
                &self.playfield,
                horizontal_movement,
                0,
                self.rules.rotation_system
            ) {
                self.movement_last_timestamp_x = app.game_timestamp();
                self.movement_animation_delta_grid_x =
                    self.movement_animation_current_delta_grid.x - horizontal_movement as f32;

                self.has_moved = true;
            }

            // soft drop
            let down_button = persistent.input_mapping.button("down".to_string());
            if down_button.pressed_repeat(self.rules.soft_drop_interval, app) {
                if try_soft_drop_piece(
                    self.current_piece.as_ref().unwrap(),
                    &mut self.current_piece_pos,
                    &self.playfield,
                    &self.rules
                ) {
                    self.movement_last_timestamp_y = app.game_timestamp();
                    self.movement_animation_delta_grid_y =
                        self.movement_animation_current_delta_grid.y + 1.0;

                    self.has_moved = true;
                    self.has_stepped = true;
                }
            }

            // Rotate
            let mut rotation = 0;

            let ccw_button = persistent.input_mapping.button("rotate_ccw".to_string());
            if ccw_button.pressed() { rotation -= 1; }

            let cw_button = persistent.input_mapping.button("rotate_cw".to_string());
            if cw_button.pressed() { rotation += 1; }

            if rotation != 0 {
                if let Some(ref mut piece) = self.current_piece {
                    if try_rotate_piece(
                        piece,
                        &mut self.current_piece_pos,
                        rotation > 0,
                        &self.playfield,
                        &self.rules
                    ) {
                        self.has_rotated = true;
                    }
                }
            }
        }

        //
        // The next three mechanics can remove the current piece, so we have to isolate them and
        // verify again if the current piece is available or not

        if self.current_piece.is_some() {
            // hard drop
            let up_button = persistent.input_mapping.button("hard_drop".to_string());
            if up_button.pressed() {
                if try_hard_drop_piece(
                    self.current_piece.as_ref().unwrap(),
                    &mut self.current_piece_pos,
                    &mut self.playfield,
                    &self.rules
                ) {
                    // @Refactor this is repeated and any lock piece should check for this.
                    let piece = self.current_piece.as_ref().unwrap();
                    if locked_out(piece, self.current_piece_pos, &self.rules) {
                        self.topped_out = true;
                        println!("game over: locked out");
                        return;
                    }

                    self.current_piece = None;
                    self.lock_piece_timestamp = app.game_timestamp();
                }
            }
        }

        if self.current_piece.is_some() {
            // hold piece
            let hold_button = persistent.input_mapping.button("hold".to_string());
            if hold_button.pressed() {
                if !self.has_used_hold {
                    match self.hold_piece.take() {
                        Some(hold_piece) => {
                            let piece = self.current_piece.as_mut().unwrap();
                            if self.rules.hold_piece_reset_rotation {
                                piece.rot = 0;
                            }

                            self.hold_piece = Some(*piece);

                            *piece = hold_piece;
                            self.current_piece_pos = Vec2i {
                                x: self.playfield.grid_size.x / 2 - 2,
                                y: self.rules.spawn_row as i32 - 3,
                            };

                            self.has_used_hold = true;

                            // update movement timestamps
                            self.movement_last_timestamp_x = app.game_timestamp();
                            self.movement_last_timestamp_y = app.game_timestamp();
                        }

                        None => {
                            let piece = self.current_piece.as_mut().unwrap();
                            if self.rules.hold_piece_reset_rotation {
                                piece.rot = 0;
                            }
                            self.hold_piece = Some(*piece);

                            // since this is not locking a piece (won't trigger animation or ARE),
                            // so we don't update the lock timestamp.
                            self.current_piece = None;
                        }
                    }
                }
            }
        }

        if self.current_piece.is_some() {
            // Gravity
            // @TODO move this to Rules (or something)
            if app.game_timestamp() >= self.movement_last_timestamp_y + self.rules.gravity_interval {
                if try_apply_gravity(
                    self.current_piece.as_ref().unwrap(),
                    &mut self.current_piece_pos,
                    &self.playfield,
                    self.rules.rotation_system
                ) {
                    self.has_stepped = true;

                    self.movement_last_timestamp_y = app.game_timestamp();
                    self.movement_animation_delta_grid_y = self.movement_animation_current_delta_grid.y + 1.0;
                } else {
                    // Only lock on gravity movement if there's no lock delay
                    if self.rules.lock_delay == LockDelayRule::NoDelay {
                        let piece = self.current_piece.as_ref().unwrap();
                        lock_piece(
                            piece,
                            self.current_piece_pos,
                            &mut self.playfield,
                            self.rules.rotation_system
                        );

                        // @Refactor this is repeated and any lock piece should check for this.
                        if locked_out(piece, self.current_piece_pos, &self.rules) {
                            self.topped_out = true;
                            println!("game over: locked out");
                            return;
                        }

                        self.current_piece = None;
                        self.lock_piece_timestamp = app.game_timestamp();
                    }
                }
            }
        }

        // @Maybe move this to render?
        // piece movement animation
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

        // line clear
        let can_spawn_new_piece;

        if self.has_line_clear_animation {
            if !self.is_line_clear_animation_playing {
                if let Some(lines) = self.playfield.get_clear_lines() {
                    self.line_clear_animation_state.lines_to_clear = lines;
                    self.is_line_clear_animation_playing = true;
                    self.line_clear_animation_timestamp = app.game_timestamp();
                }
            }

            if self.is_line_clear_animation_playing {
                let is_animation_over;

                match self.line_clear_animation_state.type_ {
                    LineClearAnimationType::Classic => {
                        // Tetris Classic clear line animation has 5 steps
                        // 1st step: bbbb__bbbb
                        // 2st step: bbb____bbb
                        // 3nd step: bb______bb
                        // 4rd step: b________b
                        // 5th step: __________
                        let animation_duration = app.game_timestamp() - self.line_clear_animation_timestamp;
                        let animation_step = 5 * animation_duration / self.rules.line_clear_delay;
                        is_animation_over = animation_step >= 5;

                        self.line_clear_animation_state.step = animation_step as u8;
                    },
                }

                if is_animation_over {
                    self.is_line_clear_animation_playing = false;
                    self.rules.try_clear_lines(&mut self.playfield);
                } else {
                    self.is_line_clear_animation_playing = true;
                }
            }

            can_spawn_new_piece = !self.is_line_clear_animation_playing;
        } else {
            self.rules.try_clear_lines(&mut self.playfield);
            can_spawn_new_piece = true;
        }

        // new piece
        if self.current_piece.is_none() && can_spawn_new_piece &&
            app.game_timestamp() >= self.lock_piece_timestamp + self.rules.entry_delay
        {
            self.new_piece();

            self.movement_last_timestamp_x = app.game_timestamp();
            self.movement_last_timestamp_y = app.game_timestamp();

            // check for block out
            let block_out = blocked_out(
                self.current_piece.as_ref().unwrap(),
                self.current_piece_pos,
                &self.playfield,
                &self.rules
            );

            if block_out {
                self.topped_out = true;
                println!("game over: block out");
                return;
            }

            // spawn drop
            if self.rules.spawn_drop {
                while blocks_out_of_playfield(
                    self.current_piece.as_ref().unwrap(),
                    self.current_piece_pos,
                    &self.rules
                ) > 0 {
                    if !try_apply_gravity(
                        self.current_piece.as_ref().unwrap(),
                        &mut self.current_piece_pos,
                        &self.playfield,
                        self.rules.rotation_system
                    ) {
                        break;
                    }
                }
            }
        }
    }

    fn render(
        &mut self,
        app: &mut App<'_, State>,
        persistent: &mut PersistentData
    ) {
        app.queue_draw_text(
            &format!("time: {:.2}", app.game_time()),
            &persistent.font,
            &TransformBuilder::new().pos_xy(10.0, 42.0).layer(1000).build(),
            32.,
            WHITE
        );

        if let Some(piece) = self.current_piece {
            let rot = ((piece.rot % 4) + 4) % 4;
            app.queue_draw_text(
                &format!("rot: {}", rot),
                &persistent.font,
                &TransformBuilder::new().pos_xy(10.0, 84.0).layer(1000).build(),
                32.,
                WHITE
            );
        }

        // playfield
        let playfield_size = get_draw_playfield_size(&self.playfield, persistent.pixel_scale);

        // @Temporary recalculate playfield position since has_grid can change
        let window_size = app.video_system.window.size();
        self.playfield.pos = Vec2i {
            x: (window_size.0 as f32 - playfield_size.x) as i32 / 2,
            y: (window_size.1 as f32 - playfield_size.y) as i32 / 2,
        };

        if self.has_line_clear_animation {
            let line_clear_animation_state;
            if self.is_line_clear_animation_playing {
                line_clear_animation_state = Some(&self.line_clear_animation_state);
            } else {
                line_clear_animation_state = None;
            }

            draw_playfield(
                &self.playfield,
                line_clear_animation_state,
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
            if self.movement_animation_show_ghost {
                draw_piece_in_playfield(
                    piece,
                    self.current_piece_pos,
                    Vec2::new(),
                    Color { r: 1., g: 1., b: 1., a: 0.1 },
                    &self.playfield,
                    self.rules.rotation_system,
                    app,
                    persistent
                );
            }

            // render ghost piece
            if self.rules.has_ghost_piece {
                // @TODO cache the ghost piece and only recalculate the position when piece moves
                let mut ghost_piece = piece.clone();
                let mut ghost_piece_pos = self.current_piece_pos;

                // Only render if ghost is not on top of the current piece
                // @Maybe improve this and just draw the different pixels. Since there's piece
                //        movement animation, not rendering the ghost in the middle of the piece
                //        movement may be visually weird
                if full_drop_piece(
                    &mut ghost_piece,
                    &mut ghost_piece_pos,
                    &self.playfield,
                    self.rules.rotation_system
                ) {
                    draw_piece_in_playfield(
                        ghost_piece,
                        ghost_piece_pos,
                        Vec2::new(),
                        Color { r: 1., g: 1., b: 1., a: 0.1 },
                        &self.playfield,
                        self.rules.rotation_system,
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
            if self.is_locking {
                let delta_time = app.game_timestamp() - self.locking_animation_timestamp;
                let alpha = 2.0 * std::f32::consts::PI * delta_time as f32;
                let alpha = alpha / (self.locking_animation_duration as f32);
                let alpha = ((1.0 + alpha.sin()) / 2.0) / 2.0 + 0.5;

                color = piece.color(self.rules.rotation_system).alpha(alpha);
            } else {
                color = piece.color(self.rules.rotation_system);
            }

            draw_piece_in_playfield(
                piece,
                self.current_piece_pos,
                movement_animation_delta_grid,
                color,
                &self.playfield,
                self.rules.rotation_system,
                app,
                persistent
            );
        }

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
                    Piece { type_: self.next_piece_types[i as usize], rot: 0 },
                    Vec2 {
                        x: window_pos.x,
                        y: window_pos.y + i as f32 * size,
                    },
                    self.next_piece_types[i as usize].color(self.rules.rotation_system),
                    self.playfield.has_grid,
                    self.rules.rotation_system,
                    app,
                    persistent
                );
            }
        }

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
                    hold_piece.color(self.rules.rotation_system),
                    self.playfield.has_grid,
                    self.rules.rotation_system,
                    app,
                    persistent
                );
            }
        }

        // queue rendering
        app.render_queued();
    }

    fn handle_input(
        &mut self,
        app: &mut App<'_, State>,
        _persistent: &mut PersistentData,
        event: &sdl2::event::Event
    ) -> bool {
        use sdl2::event::Event;
        use sdl2::keyboard::Scancode;

        match event {
            Event::KeyDown { scancode: Some(Scancode::F3), .. } => {
                app.set_time_scale(0.1);
            }

            Event::KeyDown { scancode: Some(Scancode::F4), .. } => {
                app.set_time_scale(1.0);
            }

            Event::KeyDown { scancode: Some(Scancode::F10), .. } => {
                self.debug_pieces_scene_opened = true;
                app.pause();
            }

            Event::KeyDown { scancode: Some(Scancode::W), .. } => {
                self.playfield.has_grid = !self.playfield.has_grid;
            }

            _ => {}
        }

        false
    }

    fn transition(&mut self) -> Option<SceneTransition> {
        if self.debug_pieces_scene_opened {
            self.debug_pieces_scene_opened = false;
            Some(SceneTransition::Push(Scene::DebugPiecesScene(DebugPiecesScene::new())))
        } else {
            None
        }
    }
}

impl SinglePlayerScene {
    pub fn new(app: &mut App<'_, State>, persistent: &mut PersistentData) -> Self {
        let pixel_scale = persistent.pixel_scale;

        // playfield positioning
        let playfield_grid_size = Vec2i { x: 10, y: 40 };

        let playfield_pixel_size = Vec2i {
            x: (pixel_scale as f32 * BLOCK_SCALE * playfield_grid_size.x as f32) as i32,
            y: (pixel_scale as f32 * BLOCK_SCALE * PLAYFIELD_VISIBLE_HEIGHT as f32) as i32,
        };

        let window_size = app.video_system.window.size();

        let playfield_pos = Vec2i {
            x: (window_size.0 as i32 - playfield_pixel_size.x) / 2,
            y: (window_size.1 as i32 - playfield_pixel_size.y) / 2,
        };

        let playfield = Playfield::new(playfield_pos, playfield_grid_size, true);

        // rules
        let rules: Rules = RotationSystem::SRS.into();

        // rng
        let mut randomizer: Randomizer = RandomizerType::Random7Bag.into();

        // next pieces preview window
        let mut next_piece_types = [PieceType::I; NEXT_PIECES_COUNT];
        for i in 0..NEXT_PIECES_COUNT {
            next_piece_types[i] = randomizer.next_piece();
        }

        let preview_window_delta_pos = Vec2 { x: 20.0, y: 0.0 };

        // hold window
        let hold_window_delta_pos = Vec2 { x: -20.0, y: 0.0 };

        // locking
        let remaining_lock_delay = rules.lock_delay;

        Self {
            debug_pieces_scene_opened: false,

            topped_out: false,

            playfield,
            rules,
            randomizer,

            current_piece: None,
            current_piece_pos: Vec2i::new(),
            next_piece_types,
            lock_piece_timestamp: 0,

            hold_piece: None,
            has_used_hold: false,

            is_locking: false,
            has_moved: false,
            has_rotated: false,
            has_stepped: false,
            remaining_lock_delay,

            preview_window_delta_pos,
            hold_window_delta_pos,

            movement_last_timestamp_x: app.game_timestamp(),
            movement_last_timestamp_y: app.game_timestamp(),

            has_movement_animation: true,
            movement_animation_show_ghost: false,
            movement_animation_duration: 50_000,
            movement_animation_delta_grid_x: 0.0,
            movement_animation_delta_grid_y: 0.0,
            movement_animation_current_delta_grid: Vec2::new(),

            has_line_clear_animation: true,
            is_line_clear_animation_playing: false,
            line_clear_animation_timestamp: 0,
            line_clear_animation_state: LineClearAnimationState {
                type_: LineClearAnimationType::Classic,
                step: 0,
                lines_to_clear: Vec::new(),
            },

            locking_animation_timestamp: 0,
            locking_animation_duration: 250_000,
        }
    }

    fn new_piece(&mut self) {
        self.current_piece_pos = Vec2i {
            x: self.playfield.grid_size.x / 2 - 2,
            y: self.rules.spawn_row as i32 - 3,
        };

        self.current_piece = Some(Piece {
            type_: self.next_piece_types[0],
            rot: 0,
        });

        // cycle next pieces
        for i in 0..NEXT_PIECES_COUNT-1 { self.next_piece_types[i] = self.next_piece_types[i+1]; }
        self.next_piece_types[7] = self.randomizer.next_piece();

        self.movement_animation_delta_grid_x = 0.0;
        self.movement_animation_delta_grid_y = 0.0;
        self.movement_animation_current_delta_grid = Vec2::new();

        self.has_used_hold = false;

        self.remaining_lock_delay = self.rules.lock_delay;
    }
}

/*
enum TetrisState {
    Falling,
    LineClearAnimation,
}
*/

// @Refactor move these functions to their proper places

/*
fn grid_to_pixels(pos: Vec2, pixel_scale: Vec2) -> Vec2 {
    let bottom = BLOCK_SCALE * pixel_scale.y * PLAYFIELD_VISIBLE_HEIGHT as f32;

    Vec2 {
        x: BLOCK_SCALE * pixel_scale.x * pos.x,
        y: bottom - BLOCK_SCALE * pixel_scale.y * (pos.y + 1.0),
    }
}
*/

fn norm_u64(v: u64, min: u64, max: u64) -> f32 {
    if v <= min { return 0.0; }
    if v >= max { return 1.0; }
    (v - min) as f32 / (max - min) as f32
}

/*
fn norm_f32(v: f32, min: f32, max: f32) -> f32 {
    if v <= min { return 0.0; }
    if v >= max { return 1.0; }
    (v - min) / (max - min)
}
*/

fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn lerp_vec2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    Vec2 {
        x: lerp_f32(a.x, b.x, t),
        y: lerp_f32(a.y, b.y, t)
    }
}
