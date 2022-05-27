use crate::app::*;
use crate::linalg::Vec2i;

use super::*;

use crate::game::{
    input::*,
    render::*,
    rules::{
        Rules,
    },
    tetris_game::{ TetrisGame, TetrisLayout },
};

#[derive(Debug, ImDraw)]
pub struct SinglePlayerScene {
    tetris_game: TetrisGame,
    music_id: MusicId,
    server: Option<Server>,
    quit: bool,

    is_preview: bool,

    tetris_layout: TetrisLayout,
}

impl SceneTrait for SinglePlayerScene {
    type Scene = Scene;
    type PersistentData = PersistentData;

    fn update(
        &mut self,
        dt: u64,
        app: &mut App,
        persistent: &mut Self::PersistentData
    ) {
        // pause
        let options_button = persistent.input_mapping.button(KEY_OPTIONS.to_string());
        if options_button.pressed() {
            if app.is_paused() { app.resume(); }
            else { app.pause(); }
        }

        if app.is_paused() { return; }

        self.tetris_game.update(dt, &persistent.input_mapping, app);
    }

    fn render(
        &mut self,
        app: &mut App,
        persistent: &mut Self::PersistentData
    ) {
        if app.is_paused() {
            let window_size = app.window_size();
            let window_size = Vec2i { x: window_size.0 as i32, y: window_size.1 as i32 };
            let menu_size = Vec2i { x: 600, y: 300 };

            // Ui
            let window_layout = ui::Layout {
                pos: Vec2i {
                    x: (window_size.x - menu_size.x) / 2,
                    y: (window_size.y - menu_size.y) / 2,
                },
                size: menu_size
            };

            ui::Ui::builder(window_layout).build(app);

            ui::Text::new("PAUSED", app);

            // Ui
            if ui::Button::new("RESUME", app).pressed {
                app.resume();
            }

            if ui::Button::new("RESTART", app).pressed {
                println!("restart");
            }

            if ui::Button::new("QUIT", app).pressed {
                self.quit = true;
            }
        }

        self.tetris_game.update_and_render(self.tetris_layout, &mut app.batch(), persistent);

        /*
        self.tetris_game.update_animations();
        self.tetris_game.render_playfield(self.playfield_pos, true, &mut app.batch(), persistent);
        self.tetris_game.render_hold_piece(self.hold_piece_window_pos, true, &mut app.batch(), persistent);
        self.tetris_game.render_next_pieces_preview(self.next_pieces_preview_window_pos, 0, true, &mut app.batch(), persistent);
        */

        if self.is_preview {
            app.queue_draw_text(
                "PREVIEW",
                TransformBuilder::new().pos_xy(10.0, 42.0).layer(800).build(),
                32.,
                WHITE,
                None,
                None,
            );
        }

        app.queue_draw_text(
            &format!("time: {:.2}", to_seconds(self.tetris_game.timestamp())),
            TransformBuilder::new().pos_xy(10.0, 84.0).layer(800).build(),
            32.,
            WHITE,
            None,
            None,
        );

        app.queue_draw_text(
            &format!("level: {}", self.tetris_game.level()),
            TransformBuilder::new().pos_xy(10.0, 126.0).layer(800).build(),
            32.,
            WHITE,
            None,
            None,
        );

        app.queue_draw_text(
            &format!("score: {}", self.tetris_game.score()),
            TransformBuilder::new().pos_xy(10.0, 168.0).layer(800).build(),
            32.,
            WHITE,
            None,
            None,
        );

        app.queue_draw_text(
            &format!("lines: {}", self.tetris_game.total_lines_cleared()),
            TransformBuilder::new().pos_xy(10.0, 210.0).layer(800).build(),
            32.,
            WHITE,
            None,
            None,
        );
    }

    fn handle_input(
        &mut self,
        event: &sdl2::event::Event,
        app: &mut App,
        _persistent: &mut Self::PersistentData,
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

            Event::KeyDown { scancode: Some(Scancode::W), .. } => {
                //self.playfield.has_grid = !self.playfield.has_grid;
            }

            Event::KeyDown { scancode: Some(Scancode::D), .. } => {
                //self.tetris_game.next_level();
            }

            Event::KeyDown { scancode: Some(Scancode::F), .. } => {
                app.play_music(self.music_id);
            }

            _ => {}
        }

        false
    }

    fn transition(
        &mut self,
        _app: &mut App,
        _persistent: &mut Self::PersistentData
    ) -> Option<SceneTransition<Self::Scene>> {
        if self.quit {
            Some(SceneTransition::Pop)
        } else {
            None
        }
    }

    fn on_enter(&mut self, app: &mut App, _persistent: &mut Self::PersistentData,) {
        app.restart_time_system();
        //app.play_music(self.music_id);
    }
}

impl SinglePlayerScene {
    pub fn new(
        seed: u64,
        rules: Rules,
        app: &mut App,
        persistent: &mut PersistentData
    ) -> Self {
        // rules
        let tetris_game = TetrisGame::new(rules, seed);
        let tetris_layout = tetris_game.new_layout(app, persistent);

        let music_id = app.load_music("assets/sfx/Original-Tetris-theme.ogg");

        Self {
            tetris_game,
            music_id,
            server: None,
            quit: false,

            is_preview: false,

            tetris_layout,
        }
    }
}
