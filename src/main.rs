// Remove console on Windows if not in debug build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, allow(dead_code))]

//#![feature(option_expect_none)]

#[macro_use] extern crate bitflags;
extern crate imgui;
extern crate imgui_opengl_renderer;

// @Important maybe remove this dependency
extern crate rand_pcg;
extern crate rand_core;

#[macro_use] mod app;
mod linalg;
mod game;

use app::*;
use linalg::*;

use game::{
    input::*,
    piece::PieceType,
    playfield::Playfield,
    randomizer::*,
    rules::{ Rules, RotationSystem },
};

fn main() {
    let config = AppConfig {
        window_name: "LD48".to_string(),
        window_size: (1280, 960),
    };

    App::<State>::new(config).run();
}

#[derive(ImDraw)]
pub struct State {
    pub test: Test,

    // @Fix clicking on this bool in imgui window will make imgui consume all events
    pub show_debug: bool,

    pub input_mapping: InputMapping,
    pub font: Font,
    pub sprites: Sprites,

    pub pixel_scale: Vec2,
    pub playfield: Playfield,
    pub piece: Piece,
    pub rules: Rules,

    pub move_task: Option<Task>,
}

#[derive(ImDraw)]
pub struct Sprites {
    pub blank: Sprite,
    pub block: Sprite,
}

#[derive(ImDraw)]
pub struct Piece {
    // @Maybe add rotation to the Piece
    pub type_: PieceType,
    pub pos: Vec2i,
    pub rot: i32,
}

#[derive(ImDraw)]
pub struct Test {
    pub movement_delay: u64,
    pub rng: RandomizerRandomGenerator,
}

const BLOCK_SCALE : f32 = 8.0;

impl GameState for State {
    fn new(app: &mut App<'_, Self>) -> Self {
        // Fonts
        let font = app.bake_font("assets/fonts/Monocons.ttf").unwrap();

        // Sprites
        let build_sprite = |tex, x, y, w, h| {
            Sprite {
                texture: tex,
                texture_flip: TextureFlip::NO,
                uvs: (Vec2i { x, y }, Vec2i { x: w + x, y: h + y }),
                pivot: Vec2 { x: 0.0, y: 0.0 },
                size: Vec2 { x:  w as f32, y: h as f32 },
            }
        };

        let blank_texture = app.get_texture("assets/gfx/blank.png");
        let blank = build_sprite(blank_texture, 0, 0, 1, 1);

        let block_texture = app.get_texture("assets/gfx/block.png");
        let block = build_sprite(block_texture, 0, 0, 8, 8);

        // input
        let input_mapping = get_default_input_mapping();

        // pixel scaling
        let pixel_scale = Vec2 { x: 5.0, y: 5.0 };

        // playfield positioning
        let playfield_size = Vec2i { x: 10, y: 20 };

        let playfield_pixel_size = Vec2i {
            x: (pixel_scale.x * BLOCK_SCALE * playfield_size.x as f32) as i32,
            y: (pixel_scale.y * BLOCK_SCALE * playfield_size.y as f32) as i32,
        };

        let window_size = app.video_system.window.size();

        let playfield_pos = Vec2i {
            x: (window_size.0 as i32 - playfield_pixel_size.x) / 2,
            y: 100
        };

        // allocate blocks

        let mut blocks = Vec::new();
        blocks.resize((playfield_size.x * playfield_size.y) as usize, false);

        Self {
            test: Test {
                movement_delay: 250_000,
                rng: RandomizerRandomGenerator::new(),
            },

            show_debug: false,
            input_mapping,
            font,
            sprites: Sprites {
                blank,
                block,
            },
            pixel_scale,
            playfield: Playfield {
                pos: playfield_pos,
                size: playfield_size,
                blocks
            },
            piece: Piece {
                // @XXX This is wrong! We should be calling the rng next_piece
                type_: PieceType::S,
                pos: Vec2i { x: playfield_size.x / 2 - 2, y: -4 },
                rot: 0,
            },
            rules: RotationSystem::SRS.into(),
            move_task: Some(app.schedule_task(1_000_000, classic_move)),
        }
    }

    fn update(&mut self, app: &mut App<'_, Self>) {
        app.update_input_mapping(&mut self.input_mapping);

        // horizontal movement logic
        let mut horizontal_movement = 0;

        let left_button = self.input_mapping.button("LEFT".to_string());
        if left_button.pressed() { horizontal_movement -= 1; }

        let right_button = self.input_mapping.button("RIGHT".to_string());
        if right_button.pressed() { horizontal_movement += 1; }

        self.try_move_piece(horizontal_movement, 0);

        // hard drop
        let up_button = self.input_mapping.button("UP".to_string());
        if up_button.pressed() { self.hard_drop_piece(); }

        // Rotate
        let mut rotation = 0;

        let ccw_button = self.input_mapping.button("rotate_ccw".to_string());
        if ccw_button.pressed() { rotation -= 1; }

        let cw_button = self.input_mapping.button("rotate_cw".to_string());
        if cw_button.pressed() { rotation += 1; }

        self.try_rotate_piece(rotation);
    }

    fn render(&mut self, app: &mut App<'_, Self>) {
        app.queue_draw_text(
            &format!("time: {:.2}", app.game_time()),
            &self.font,
            &TransformBuilder::new().pos_xy(10.0, 42.0).layer(1000).build(),
            32.,
            WHITE
        );

        self.draw_playfield(app);
        self.draw_piece_in_playfield(
            &self.piece,
            app
        );

        app.render_queued();

        if self.show_debug {
            // @Refactor maybe this debug info really should be managed by the App. This way
            //           we don't have to explicitly call render_queued, which seems way cleaner.
            //           Maybe not, since we can add framebuffers and have more control of rendering here.
            app.render_debug(self, |ui, state| {
                state.imdraw("State", ui);
            });
        }
    }

    fn handle_input(&mut self, app: &mut App<'_, Self>, event: &sdl2::event::Event) -> bool {
        use sdl2::event::Event;
        use sdl2::keyboard::Scancode;

        if app.handle_debug_event(&event) { return true; }

        match event {
            Event::KeyDown { scancode: Some(Scancode::F11), .. } => {
                use sdl2::video::FullscreenType;

                let window = &mut app.video_system.window;
                let new_fullscreen_state = match window.fullscreen_state() {
                    //FullscreenType::Off => FullscreenType::True,
                    //FullscreenType::True => FullscreenType::Desktop,
                    //FullscreenType::Desktop => FullscreenType::Off,

                    FullscreenType::Off => FullscreenType::Desktop,
                    _ => FullscreenType::Off,
                };

                window.set_fullscreen(new_fullscreen_state).unwrap();
            }

            Event::KeyDown { scancode: Some(Scancode::F1), .. } => {
                self.show_debug = !self.show_debug;
            }

            Event::KeyDown { scancode: Some(Scancode::F2), .. } => {
                if app.is_paused() {
                    app.resume();
                } else {
                    app.pause();
                }
            }

            _ => {}
        }

        false
    }
}

impl State {
    fn new_piece(&mut self) {
        self.piece.pos = Vec2i {
            x: self.playfield.size.x / 2 - 2,
            y: -4
        };

        self.piece.rot = 0;

        self.piece.type_ = self.test.rng.next_piece();
    }

    fn lock_piece(&mut self) {
        for block_pos in self.piece.type_.blocks(self.piece.rot) {
            self.playfield.set_block(
                self.piece.pos.x + block_pos.x,
                self.piece.pos.y + block_pos.y,
                true
            );
        }
    }

    fn hard_drop_piece(&mut self) {
        while self.try_move_piece(0, 1) {}

        self.lock_piece();
        self.new_piece();
    }

    fn try_move_piece(&mut self, dx: i32, dy: i32) -> bool {
        for block_pos in self.piece.type_.blocks(self.piece.rot) {
            let new_x = self.piece.pos.x + block_pos.x + dx;
            let new_y = self.piece.pos.y + block_pos.y + dy;
            if self.playfield.block(new_x, new_y) {
                return false;
            }
        }

        self.piece.pos += Vec2i { x: dx, y: dy };
        true
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

    // @Refactor this shouldn't be this general unless it has more usability outside of just
    //           drawing the current piece. Or, at least, this should be outside the State
    fn draw_piece_in_playfield(
        &self,
        piece: &Piece,
        app: &mut App<'_, Self>
    ) {
        for block_pos in piece.type_.blocks(piece.rot) {
            self.draw_block_in_playfield(piece.pos.x + block_pos.x, piece.pos.y + block_pos.y, app);
        }
    }

    // @Refactor this should be outside of State
    fn draw_block_in_playfield(&self, pos_x: i32, pos_y: i32, app: &mut App<'_, Self>) {
        if pos_x < 0 || pos_x >= self.playfield.size.x ||
           pos_y < 0 || pos_y >= self.playfield.size.y {

            return;
        }

        let pos = Vec2 {
            x: self.playfield.pos.x as f32 + BLOCK_SCALE * self.pixel_scale.x * pos_x as f32,
            y: self.playfield.pos.y as f32 + BLOCK_SCALE * self.pixel_scale.y * pos_y as f32,
        };

        app.queue_draw_sprite(
            &TransformBuilder::new()
                .pos(pos)
                .scale(self.pixel_scale)
                .layer(10)
                .build(),
            &self.sprites.block,
            WHITE
        );
    }

    fn draw_playfield(&self, app: &mut App<'_, Self>) {
        // left
        let pos = Vec2::from(self.playfield.pos) - self.pixel_scale;
        let scale = Vec2 {
            x: self.pixel_scale.x,
            y: self.pixel_scale.y * (2.0 + BLOCK_SCALE * self.playfield.size.y as f32),
        };
        app.queue_draw_sprite(
            &TransformBuilder::new().pos(pos).scale(scale).build(),
            &self.sprites.blank,
            BLACK
        );

        // right
        let pos = Vec2::from(self.playfield.pos) + Vec2 {
            x: BLOCK_SCALE * self.pixel_scale.x * self.playfield.size.x as f32,
            y: -self.pixel_scale.y
        };
        app.queue_draw_sprite(
            &TransformBuilder::new().pos(pos).scale(scale).build(),
            &self.sprites.blank,
            BLACK
        );

        // top
        let pos = Vec2::from(self.playfield.pos) - self.pixel_scale;
        let scale = Vec2 {
            x: self.pixel_scale.x * (2.0 + BLOCK_SCALE * self.playfield.size.x as f32),
            y: self.pixel_scale.y,
        };
        app.queue_draw_sprite(
            &TransformBuilder::new().pos(pos).scale(scale).build(),
            &self.sprites.blank,
            BLACK
        );

        // bottom
        let pos = Vec2::from(self.playfield.pos) + Vec2 {
            x: -self.pixel_scale.x,
            y: BLOCK_SCALE * self.pixel_scale.y * self.playfield.size.y as f32,
        };
        app.queue_draw_sprite(
            &TransformBuilder::new().pos(pos).scale(scale).build(),
            &self.sprites.blank,
            BLACK
        );

        // bg
        let pos = Vec2::from(self.playfield.pos);
        let scale = BLOCK_SCALE * Vec2 {
            x: self.pixel_scale.x * self.playfield.size.x as f32,
            y: self.pixel_scale.y * self.playfield.size.y as f32,
        };
        app.queue_draw_sprite(
            // @TODO fix layer negative not showing
            &TransformBuilder::new().pos(pos).scale(scale).layer(0).build(),
            &self.sprites.blank,
            Color { r: 0.2, g: 0.2, b: 0.2, a: 1.0 },
        );

        // blocks

        // @Refactor cache playfield/draw to framebuffer
        for i in 0..self.playfield.size.y {
            for j in 0..self.playfield.size.x {
                if self.playfield.block(j, i) {
                    self.draw_block_in_playfield(j, i, app);
                }
            }
        }
    }
}

// Move functions

fn classic_move(_: u64, state: &mut State, app: &mut App<State>) {
    if !state.try_move_piece(0, 1) {
        state.lock_piece();
        state.new_piece();
    }

    state.move_task = Some(app.schedule_task(state.test.movement_delay, classic_move));
}
