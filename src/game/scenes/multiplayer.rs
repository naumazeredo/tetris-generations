use crate::app::*;
use crate::linalg::Vec2i;

use super::*;

use crate::game::{
    network::{MultiplayerMessages, Connect, Update},
    render::*,
    rules::{
        RotationSystem,
        Rules,
    },
    tetris_game::TetrisGame,
};

#[derive(Debug, ImDraw)]
pub struct MultiPlayerScene {
    quit: bool,
    tetris_game: TetrisGame,
    server: Server,

    playfield_pos: Vec2i,
    hold_piece_window_pos: Vec2i,
    next_pieces_preview_window_pos: Vec2i,
}

impl SceneTrait for MultiPlayerScene {
    type Scene = Scene;
    type PersistentData = PersistentData;

    fn update(
        &mut self,
        dt: u64,
        app: &mut App,
        persistent: &mut Self::PersistentData
    ) {
        // Networking
        loop {
            match self.server.next_event() {
                Ok(None) => {},
                Ok(Some(e)) => {
                    //println!("server event: {:?}", e);

                    match e {
                        ServerEvent::ClientConnect(client_id) => {
                            let connect = Connect {
                                timestamp: app.game_timestamp(),
                                tetris_game: self.tetris_game.to_network(),
                                rotation_system: self.tetris_game.rules().rotation_system,
                                randomizer: self.tetris_game.randomizer().clone(),
                            };

                            let message = MultiplayerMessages::Connect(connect);
                            self.server.send(client_id, message).unwrap();
                        }

                        _ => {}
                    }

                    continue;
                },
                Err(err) => println!("server event error: {:?}", err),
            }
            break;
        }

        // pause
        let options_button = persistent.input_mapping.button("options".to_string());
        if options_button.pressed() {
            if app.is_paused() { app.resume(); }
            else { app.pause(); }
        }

        if !app.is_paused() {
            let has_updated = self.tetris_game.update(dt, &persistent.input_mapping, app);
            if has_updated {
                let update = Update {
                    timestamp: app.game_timestamp(),
                    tetris_game: self.tetris_game.to_network(),
                };

                let message = MultiplayerMessages::Update(update);
                self.server.broadcast(message).unwrap();
            }
        }
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
            ui::Text::new(&format!("IP: {}", self.server.addr()), app);

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

        self.tetris_game.update_animations();
        self.tetris_game.render_playfield(self.playfield_pos, true, &mut app.batch(), persistent);
        self.tetris_game.render_hold_piece(self.hold_piece_window_pos, true, &mut app.batch(), persistent);
        self.tetris_game.render_next_pieces_preview(self.next_pieces_preview_window_pos, 0, persistent.pixel_scale, true, &mut app.batch(), persistent);

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
            Event::KeyDown { scancode: Some(Scancode::F), .. } => {
                //app.play_music(self.music_id);
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
        if self.quit { Some(SceneTransition::Pop) } else { None }
    }

    fn on_enter(
        &mut self,
        app: &mut App,
        _persistent: &mut Self::PersistentData
    ) {
        app.restart_time_system();
        //app.play_music(self.music_id);
    }
}

impl MultiPlayerScene {
    pub fn new(
        app: &mut App,
        persistent: &mut PersistentData
    ) -> Self {
        // rules
        let seed = app.system_time();
        let rules: Rules = RotationSystem::SRS.into();
        let tetris_game = TetrisGame::new(rules, seed);

        let server = Server::new("127.0.0.1:42042").unwrap();

        // @Refactor use InstanceStyle
        // Playfield rendering
        let playfield_draw_size = get_draw_playfield_size(
            &tetris_game.playfield(),
            persistent.pixel_scale,
            true,
        );

        let window_size = app.window_size();
        let playfield_pos = Vec2i {
            x: (window_size.0 as i32 - playfield_draw_size.x) / 2,
            y: (window_size.1 as i32 - playfield_draw_size.y) / 2,
        };

        let hold_window_size = tetris_game.hold_piece_window_size(true, persistent);
        let hold_piece_window_pos =
            playfield_pos +
            Vec2i { x: -20, y: 0 } +
            Vec2i { x: -hold_window_size.x, y: 0 };

        let next_pieces_preview_window_pos =
            playfield_pos +
            Vec2i { x: 20, y: 0 } +
            Vec2i { x: playfield_draw_size.x, y: 0 };

        Self {
            quit: false,

            tetris_game,

            server,

            playfield_pos,
            hold_piece_window_pos,
            next_pieces_preview_window_pos,
        }
    }
}
