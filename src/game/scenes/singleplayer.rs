use crate::BLOCK_SCALE;
use crate::app::*;
use crate::linalg::*;
use crate::State;

use super::*;

use crate::game::{
    randomizer::*,
    rules::{ Rules, RotationSystem, movement::* },
    piece::{ Piece, PieceType },
    playfield::{ Playfield, PLAYFIELD_VISIBLE_HEIGHT },
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

    rules: Rules,
    playfield: Playfield,
    randomizer: Randomizer,
    current_piece: Piece,
    next_piece_types: [PieceType; NEXT_PIECES_COUNT],

    preview_pos: Vec2,

    movement_delay: u64,
    movement_last_timestamp_x: u64,
    movement_last_timestamp_y: u64,

    has_movement_animation: bool,
    movement_animation_show_ghost: bool,
    movement_animation_duration: u64,
    movement_animation_delta_grid_x: f32,
    movement_animation_delta_grid_y: f32,
    movement_animation_current_delta_grid: Vec2,
}

impl SceneTrait for SinglePlayerScene {
    fn update(
        &mut self,
        app: &mut App<'_, State>,
        persistent: &mut PersistentData
    ) {
        if app.is_paused() { return; }

        // horizontal movement logic
        let mut horizontal_movement = 0;

        let left_button = persistent.input_mapping.button("LEFT".to_string());
        if left_button.pressed_repeat_with_delay(
            self.rules.das_repeat_delay,
            self.rules.das_repeat_interval,
            app
        ) {
            horizontal_movement -= 1;
        }

        let right_button = persistent.input_mapping.button("RIGHT".to_string());
        if right_button.pressed_repeat_with_delay(
            self.rules.das_repeat_delay,
            self.rules.das_repeat_interval,
            app
        ) {
            horizontal_movement += 1;
        }

        if horizontal_movement != 0 && try_move_piece(&mut self.current_piece, &self.playfield, horizontal_movement, 0) {
            self.movement_last_timestamp_x = app.game_timestamp();
            self.movement_animation_delta_grid_x =
                self.movement_animation_current_delta_grid.x - horizontal_movement as f32;
        }

        // hard drop
        let up_button = persistent.input_mapping.button("UP".to_string());
        if up_button.pressed() {
            if try_hard_drop_piece(&mut self.current_piece, &mut self.playfield, &self.rules) {
                self.new_piece();

                // @TODO move to new_piece
                self.movement_last_timestamp_x = app.game_timestamp();
                self.movement_last_timestamp_y = app.game_timestamp();
            }
        }

        // soft drop
        let down_button = persistent.input_mapping.button("DOWN".to_string());
        if down_button.pressed_repeat(self.rules.soft_drop_interval, app) {
            if try_soft_drop_piece(&mut self.current_piece, &self.playfield, &self.rules) {
                self.movement_last_timestamp_y = app.game_timestamp();
                self.movement_animation_delta_grid_y = self.movement_animation_current_delta_grid.y + 1.0;
            }
        }

        // Rotate
        let mut rotation = 0;

        let ccw_button = persistent.input_mapping.button("rotate_ccw".to_string());
        if ccw_button.pressed() { rotation -= 1; }

        let cw_button = persistent.input_mapping.button("rotate_cw".to_string());
        if cw_button.pressed() { rotation += 1; }

        self.try_rotate_piece(rotation);

        // Gravity
        // @TODO move this to Rules (or something)
        if app.game_timestamp() >= self.movement_last_timestamp_y + self.rules.gravity_interval {
            self.movement_last_timestamp_y = app.game_timestamp();
            self.movement_animation_delta_grid_y = self.movement_animation_current_delta_grid.y + 1.0;

            if try_apply_gravity(&mut self.current_piece, &self.playfield).is_none() {
                lock_piece(&self.current_piece, &mut self.playfield);
                self.new_piece();

                // @TODO move to new_piece
                self.movement_last_timestamp_x = app.game_timestamp();
                self.movement_last_timestamp_y = app.game_timestamp();
            }
        }

        // line clear
        self.rules.try_clear_lines(&mut self.playfield);

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
            // @Cleanup this shouldn't be necessary. It's necessary since we can disable the
            //          movement animation in the middle of the game, and we are using these
            //          variables to render
            self.movement_animation_delta_grid_y = 0.0;
            self.movement_animation_current_delta_grid.y = 0.0;
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

        self.draw_playfield(app, persistent);

        if self.movement_animation_show_ghost {
            self.draw_piece_in_playfield(
                &self.current_piece,
                Vec2::new(),
                Color { r: 1., g: 1., b: 1., a: 0.1 },
                app,
                persistent
            );
        }

        // render ghost piece
        if self.rules.has_ghost_piece {
            // @TODO cache the ghost piece and only recalculate the position when piece moves
            let mut ghost_piece = self.current_piece.clone();
            full_drop_piece(&mut ghost_piece, &self.playfield);
            self.draw_piece_in_playfield(
                &ghost_piece,
                Vec2::new(),
                Color { r: 1., g: 1., b: 1., a: 0.1 },
                app,
                persistent
            );
        }

        // render piece
        self.draw_piece_in_playfield(
            &self.current_piece,
            self.movement_animation_current_delta_grid,
            WHITE,
            app,
            persistent
        );

        // render preview
        if self.rules.next_pieces_preview_count > 0 {
            self.draw_rect_window(
                self.preview_pos,
                Vec2 {
                    x: persistent.pixel_scale.x * BLOCK_SCALE * 4.0,
                    y: persistent.pixel_scale.y * BLOCK_SCALE * 4.0,
                },
                persistent.pixel_scale,
                app,
                persistent
            );

            self.draw_piece_centered(
                self.next_piece_types[0],
                //self.preview_pos + persistent.pixel_scale * BLOCK_SCALE * 2.0,
                self.preview_pos,
                0,
                WHITE,
                app,
                persistent
            );
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
            Event::KeyDown { scancode: Some(Scancode::F2), .. } => {
                if app.is_paused() {
                    app.resume();
                } else {
                    app.pause();
                }
            }

            Event::KeyDown { scancode: Some(Scancode::F3), .. } => {
                app.set_time_scale(0.01);
            }

            Event::KeyDown { scancode: Some(Scancode::F4), .. } => {
                app.set_time_scale(1.0);
            }

            Event::KeyDown { scancode: Some(Scancode::F10), .. } => {
                self.debug_pieces_scene_opened = true;
                app.pause();
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
            x: (pixel_scale.x * BLOCK_SCALE * playfield_grid_size.x as f32) as i32,
            y: (pixel_scale.y * BLOCK_SCALE * playfield_grid_size.y as f32) as i32,
        };

        let window_size = app.video_system.window.size();

        let playfield_pos = Vec2i {
            x: (window_size.0 as i32 - playfield_pixel_size.x) / 2,
            y: 100
        };

        let playfield = Playfield::new(playfield_pos, playfield_grid_size);

        // rules
        let rules: Rules = RotationSystem::Original.into();

        // rng
        let mut randomizer: Randomizer = RandomizerType::Random7Bag.into();

        // @Refactor this will be calculated in the update method, since we don't just drop
        //           into the Tetris gameplay, we will have a menu and such
        let current_piece = Piece {
            type_: randomizer.next_piece(),
            pos: Vec2i { x: playfield_grid_size.x / 2 - 2, y: rules.spawn_row as i32 - 3 },
            rot: 0,
        };

        // next pieces preview
        let mut next_piece_types = [PieceType::I; NEXT_PIECES_COUNT];
        for i in 0..NEXT_PIECES_COUNT {
            next_piece_types[i] = randomizer.next_piece();
        }

        let preview_pos = Vec2 {
            x: playfield_pos.x as f32 + persistent.pixel_scale.x * BLOCK_SCALE * playfield_grid_size.x as f32 + 20.0,
            y: playfield_pos.y as f32,
        };

        Self {
            debug_pieces_scene_opened: false,
            playfield,
            rules,
            randomizer,
            current_piece,
            next_piece_types,
            preview_pos,

            movement_delay: 250_000,
            movement_last_timestamp_x: app.game_timestamp(),
            movement_last_timestamp_y: app.game_timestamp(),

            has_movement_animation: true,
            movement_animation_show_ghost: false,
            movement_animation_duration: 50_000,
            movement_animation_delta_grid_x: 0.0,
            movement_animation_delta_grid_y: 0.0,
            movement_animation_current_delta_grid: Vec2::new(),
        }
    }

    fn new_piece(&mut self) {
        self.current_piece.pos = Vec2i {
            x: self.playfield.grid_size.x / 2 - 2,
            y: self.rules.spawn_row as i32 - 3,
        };

        self.current_piece.rot = 0;

        self.current_piece.type_ = self.next_piece_types[0];
        for i in 0..NEXT_PIECES_COUNT-1 { self.next_piece_types[i] = self.next_piece_types[i+1]; }
        self.next_piece_types[7] = self.randomizer.next_piece();

        self.movement_animation_delta_grid_x = 0.0;
        self.movement_animation_delta_grid_y = 0.0;
        self.movement_animation_current_delta_grid = Vec2::new();
    }

    fn try_rotate_piece(&mut self, delta_rot: i32) -> bool {
        for block_pos in self.current_piece.type_.blocks(self.current_piece.rot + delta_rot) {
            let x = self.current_piece.pos.x + block_pos.x;
            let y = self.current_piece.pos.y + block_pos.y;
            if self.playfield.block(x, y) {
                return false;
            }
        }

        self.current_piece.rot += delta_rot;
        true
    }

    // @TODO pass pixel_scale instead of persistent
    // @Refactor color should be passed by render stack commands
    fn draw_piece_in_playfield(
        &self,
        piece: &Piece,
        delta_grid: Vec2,
        color: Color,
        app: &mut App<'_, State>,
        persistent: &PersistentData
    ) {
        for block_pos in piece.type_.blocks(piece.rot) {
            self.draw_block_in_playfield(
                piece.pos + *block_pos,
                delta_grid,
                color,
                app,
                persistent
            );
        }
    }

    // @TODO pass pixel_scale instead of persistent
    // @Refactor this should be outside of State (maybe in game/playfield. The annoying part is the
    //           need to include App and State)
    fn draw_block_in_playfield(
        &self,
        pos: Vec2i,
        delta_grid: Vec2,
        color: Color,
        app: &mut App<'_, State>,
        persistent: &PersistentData
    ) {
        if pos.x < 0 || pos.x >= self.playfield.grid_size.x ||
           pos.y < 0 || pos.y >= PLAYFIELD_VISIBLE_HEIGHT {

            return;
        }

        let pixel_scale = persistent.pixel_scale;
        let bottom = self.playfield.pos.y as f32 +
            BLOCK_SCALE * pixel_scale.y * PLAYFIELD_VISIBLE_HEIGHT as f32;

        let pos = Vec2 {
            x: self.playfield.pos.x as f32 + BLOCK_SCALE * pixel_scale.x * (pos.x as f32 + delta_grid.x),
            y: bottom - BLOCK_SCALE * pixel_scale.y * (pos.y as f32 + 1.0 + delta_grid.y),
        };

        self.draw_block(pos, color, app, persistent);
    }

    // @TODO pass pixel_scale instead of persistent
    fn draw_block(
        &self,
        pos: Vec2,
        color: Color,
        app: &mut App<'_, State>,
        persistent: &PersistentData
    ) {
        app.queue_draw_sprite(
            &TransformBuilder::new()
                .pos(pos)
                .scale(persistent.pixel_scale)
                .layer(10)
                .build(),
            &persistent.sprites.block,
            color
        );
    }

    // @TODO pass pixel_scale instead of persistent
    fn draw_piece(
        &self,
        piece_type: PieceType,
        pos: Vec2,
        rot: i32,
        color: Color,
        app: &mut App<'_, State>,
        persistent: &PersistentData
    ) {
        for block_pos in piece_type.blocks(rot) {
            let block_pos = Vec2 { x: block_pos.x as f32, y: (3 - block_pos.y) as f32 };
            self.draw_block(
                pos + block_pos * BLOCK_SCALE * persistent.pixel_scale,
                color,
                app,
                persistent
            );
        }
    }

    // @TODO pass pixel_scale instead of persistent
    fn draw_piece_centered(
        &self,
        piece_type: PieceType,
        pos: Vec2,
        rot: i32,
        color: Color,
        app: &mut App<'_, State>,
        persistent: &PersistentData
    ) {
        let min_max_x = piece_type.min_max_x(rot);
        let min_max_y = piece_type.min_max_y(rot);

        let delta =
            Vec2 {
                x: (min_max_x.0 + min_max_x.1 + 1) as f32 / 2.0,
                y: -((min_max_y.0 + min_max_y.1 + 1) as f32 / 2.0),
            };

        for block_pos in piece_type.blocks(rot) {
            let block_pos = Vec2 { x: (block_pos.x + 2) as f32, y: (1 - block_pos.y) as f32 };
            self.draw_block(
                pos + (block_pos - delta) * BLOCK_SCALE * persistent.pixel_scale,
                color,
                app,
                persistent
            );
        }
    }

    // @TODO pass pixel_scale instead of persistent
    fn draw_playfield(
        &self,
        app: &mut App<'_, State>,
        persistent: &PersistentData
    ) {
        self.draw_rect_window(
            Vec2::from(self.playfield.pos),
            Vec2 {
                x: persistent.pixel_scale.x * BLOCK_SCALE * self.playfield.grid_size.x as f32,
                y: persistent.pixel_scale.y * BLOCK_SCALE * PLAYFIELD_VISIBLE_HEIGHT as f32,
            },
            persistent.pixel_scale,
            app,
            persistent
        );

        // blocks

        // @Refactor cache playfield/draw to framebuffer
        for i in 0..PLAYFIELD_VISIBLE_HEIGHT {
            for j in 0..self.playfield.grid_size.x {
                if self.playfield.block(j, i) {
                    self.draw_block_in_playfield(
                        Vec2i { x: j, y: i },
                        Vec2::new(),
                        WHITE,
                        app,
                        persistent
                    );
                }
            }
        }
    }

    // @TODO pass pixel_scale instead of persistent
    fn draw_rect_window(
        &self,
        pos: Vec2,
        size: Vec2,
        border_size: Vec2,
        app: &mut App<'_, State>,
        persistent: &PersistentData,
    ) {
        // left
        let rect_pos = pos - border_size;
        let scale = Vec2 {
            x: border_size.x,
            y: 2.0 * border_size.y + size.y,
        };
        app.queue_draw_sprite(
            &TransformBuilder::new().pos(rect_pos).scale(scale).build(),
            &persistent.sprites.blank,
            BLACK
        );

        // right
        let rect_pos = pos + Vec2 { x: size.x, y: -border_size.y };
        app.queue_draw_sprite(
            &TransformBuilder::new().pos(rect_pos).scale(scale).build(),
            &persistent.sprites.blank,
            BLACK
        );

        // top
        let rect_pos = pos - border_size;
        let scale = Vec2 {
            x: 2.0 * border_size.x + size.x,
            y: border_size.y,
        };
        app.queue_draw_sprite(
            &TransformBuilder::new().pos(rect_pos).scale(scale).build(),
            &persistent.sprites.blank,
            BLACK
        );

        // bottom
        let rect_pos = pos + Vec2 { x: -border_size.x, y: size.y };
        app.queue_draw_sprite(
            &TransformBuilder::new().pos(rect_pos).scale(scale).build(),
            &persistent.sprites.blank,
            BLACK
        );

        // bg
        app.queue_draw_sprite(
            // @TODO fix layer negative not showing
            &TransformBuilder::new().pos(pos).scale(size).layer(0).build(),
            &persistent.sprites.blank,
            Color { r: 0.2, g: 0.2, b: 0.2, a: 1.0 },
        );
    }
}

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
