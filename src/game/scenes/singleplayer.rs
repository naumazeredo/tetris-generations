use crate::BLOCK_SCALE;
use crate::app::*;
use crate::linalg::*;
use crate::State;

use super::*;

use crate::game::{
    randomizer::*,
    rules::{ Rules, RotationSystem, movement::* },
    piece::{ Piece },
    playfield::{ Playfield, PLAYFIELD_VISIBLE_HEIGHT },
};

// @Refactor
// This struct is best described as an instantiation of the Rules, where it basically handles all
// everything the rules describes. It's better to just refactor this whole Rules logic into a struct
// with a good naming and clear objective. SinglePlayerScene should just hold the instantiation of
// the Rules and the drawing instead
#[derive(Clone, Debug)]
pub struct SinglePlayerScene {
    debug_pieces_scene_opened: bool,

    pub last_gravity_movement: u64,

    pub playfield: Playfield,
    pub piece: Piece,
    pub rules: Rules,

    pub movement_delay: u64,
    pub randomizer: Randomizer,
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

        try_move_piece(&mut self.piece, &self.playfield, horizontal_movement, 0);

        // hard drop
        let up_button = persistent.input_mapping.button("UP".to_string());
        if up_button.pressed() {
            if try_hard_drop_piece(&mut self.piece, &mut self.playfield, &self.rules) {
                self.new_piece();
                self.last_gravity_movement = app.game_timestamp();
            }
        }

        // soft drop
        let down_button = persistent.input_mapping.button("DOWN".to_string());
        if down_button.pressed_repeat(self.rules.soft_drop_interval, app) {
            if try_soft_drop_piece(&mut self.piece, &self.playfield, &self.rules) {
                self.last_gravity_movement = app.game_timestamp();
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
        if app.game_timestamp() >= self.last_gravity_movement + self.rules.gravity_interval {
            self.last_gravity_movement = app.game_timestamp();

            if try_apply_gravity(&mut self.piece, &self.playfield).is_none() {
                lock_piece(&self.piece, &mut self.playfield);
                self.new_piece();
            }
        }

        // line clear
        self.rules.try_clear_lines(&mut self.playfield);
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
        self.draw_piece_in_playfield(
            &self.piece,
            app,
            persistent
        );

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
        let piece = Piece {
            type_: randomizer.next_piece(),
            pos: Vec2i { x: playfield_grid_size.x / 2 - 2, y: rules.spawn_row as i32 - 3 },
            rot: 0,
        };

        Self {
            debug_pieces_scene_opened: false,
            last_gravity_movement: app.game_timestamp(),
            playfield,
            piece,
            rules,
            movement_delay: 250_000,
            randomizer,
        }
    }

    fn new_piece(&mut self) {
        self.piece.pos = Vec2i {
            x: self.playfield.grid_size.x / 2 - 2,
            y: self.rules.spawn_row as i32 - 3,
        };

        self.piece.rot = 0;
        self.piece.type_ = self.randomizer.next_piece();
    }

    fn try_rotate_piece(&mut self, delta_rot: i32) -> bool {
        for block_pos in self.piece.type_.blocks(self.piece.rot + delta_rot) {
            let x = self.piece.pos.x + block_pos.x;
            let y = self.piece.pos.y + block_pos.y;
            if self.playfield.block(x, y) {
                return false;
            }
        }

        self.piece.rot += delta_rot;
        true
    }

    fn draw_piece_in_playfield(
        &self,
        piece: &Piece,
        app: &mut App<'_, State>,
        persistent: &mut PersistentData
    ) {
        for block_pos in piece.type_.blocks(piece.rot) {
            self.draw_block_in_playfield(
                piece.pos.x + block_pos.x,
                piece.pos.y + block_pos.y,
                app,
                persistent
            );
        }
    }

    // @Refactor this should be outside of State (maybe in game/playfield. The annoying part is the
    //           need to include App and State)
    fn draw_block_in_playfield(
        &self,
        pos_x: i32,
        pos_y: i32,
        app: &mut App<'_, State>,
        persistent: &mut PersistentData
    ) {
        if pos_x < 0 || pos_x >= self.playfield.grid_size.x ||
           pos_y < 0 || pos_y >= PLAYFIELD_VISIBLE_HEIGHT {

            return;
        }

        let pixel_scale = persistent.pixel_scale;
        let bottom = self.playfield.pos.y as f32 +
            BLOCK_SCALE * pixel_scale.y * PLAYFIELD_VISIBLE_HEIGHT as f32;

        let pos = Vec2 {
            x: self.playfield.pos.x as f32 + BLOCK_SCALE * pixel_scale.x * pos_x as f32,
            y: bottom - BLOCK_SCALE * pixel_scale.y * (pos_y + 1) as f32,
        };

        app.queue_draw_sprite(
            &TransformBuilder::new()
                .pos(pos)
                .scale(pixel_scale)
                .layer(10)
                .build(),
            &persistent.sprites.block,
            WHITE
        );
    }

    fn draw_playfield(
        &self,
        app: &mut App<'_, State>,
        persistent: &mut PersistentData
    ) {
        let pixel_scale = persistent.pixel_scale;

        // left
        let pos = Vec2::from(self.playfield.pos) - pixel_scale;
        let scale = Vec2 {
            x: pixel_scale.x,
            y: pixel_scale.y * (2.0 + BLOCK_SCALE * PLAYFIELD_VISIBLE_HEIGHT as f32),
        };
        app.queue_draw_sprite(
            &TransformBuilder::new().pos(pos).scale(scale).build(),
            &persistent.sprites.blank,
            BLACK
        );

        // right
        let pos = Vec2::from(self.playfield.pos) + Vec2 {
            x: BLOCK_SCALE * pixel_scale.x * self.playfield.grid_size.x as f32,
            y: -pixel_scale.y
        };
        app.queue_draw_sprite(
            &TransformBuilder::new().pos(pos).scale(scale).build(),
            &persistent.sprites.blank,
            BLACK
        );

        // top
        let pos = Vec2::from(self.playfield.pos) - pixel_scale;
        let scale = Vec2 {
            x: pixel_scale.x * (2.0 + BLOCK_SCALE * self.playfield.grid_size.x as f32),
            y: pixel_scale.y,
        };
        app.queue_draw_sprite(
            &TransformBuilder::new().pos(pos).scale(scale).build(),
            &persistent.sprites.blank,
            BLACK
        );

        // bottom
        let pos = Vec2::from(self.playfield.pos) + Vec2 {
            x: -pixel_scale.x,
            y: BLOCK_SCALE * pixel_scale.y * PLAYFIELD_VISIBLE_HEIGHT as f32,
        };
        app.queue_draw_sprite(
            &TransformBuilder::new().pos(pos).scale(scale).build(),
            &persistent.sprites.blank,
            BLACK
        );

        // bg
        let pos = Vec2::from(self.playfield.pos);
        let scale = BLOCK_SCALE * Vec2 {
            x: pixel_scale.x * self.playfield.grid_size.x as f32,
            y: pixel_scale.y * PLAYFIELD_VISIBLE_HEIGHT as f32,
        };
        app.queue_draw_sprite(
            // @TODO fix layer negative not showing
            &TransformBuilder::new().pos(pos).scale(scale).layer(0).build(),
            &persistent.sprites.blank,
            Color { r: 0.2, g: 0.2, b: 0.2, a: 1.0 },
        );

        // blocks

        // @Refactor cache playfield/draw to framebuffer
        for i in 0..PLAYFIELD_VISIBLE_HEIGHT {
            for j in 0..self.playfield.grid_size.x {
                if self.playfield.block(j, i) {
                    self.draw_block_in_playfield(j, i, app, persistent);
                }
            }
        }
    }
}
