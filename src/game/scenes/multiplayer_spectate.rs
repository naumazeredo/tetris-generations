use crate::app::*;
use crate::linalg::Vec2i;

use super::*;

use crate::game::{
    network::MultiplayerMessages,
    render::*,
    rules::{
        RotationSystem,
        Rules,
    },
    tetris_game::TetrisGame,
};

#[derive(Debug, ImDraw)]
enum State {
    Normal,
    ConnectMenu,
    Connecting,
    Paused,
    Quitting,
}

#[derive(Debug, ImDraw)]
pub struct MultiPlayerSpectateScene {
    state: State,
    start_menu_server_ip: String,

    client: Client,
    tetris_game: TetrisGame,

    playfield_pos: Vec2i,
    hold_piece_window_pos: Vec2i,
    next_pieces_preview_window_pos: Vec2i,
}

impl SceneTrait for MultiPlayerSpectateScene {
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
            match self.client.next_event() {
                Ok(None) => {},
                Ok(Some(event)) => {
                    //println!("client event: {:?}", event);

                    match event {
                        ClientEvent::ServerConnectionAccept => {
                            self.state = State::Normal;
                        },

                        ClientEvent::ServerTimedOut => {
                            println!("server timed out!");
                            self.state = State::ConnectMenu;
                        },

                        ClientEvent::DisconnectedByServer => {
                            println!("disconnected by server!");
                            self.state = State::ConnectMenu;
                        },

                        ClientEvent::Data(data_payload) => {
                            match MultiplayerMessages::parse(data_payload.data()).unwrap() {
                                MultiplayerMessages::Connect(c) => {
                                    self.tetris_game = TetrisGame::from_network(
                                        c.tetris_game,
                                        c.rotation_system.into(),
                                        c.randomizer,
                                        c.timestamp,
                                        app,
                                        persistent
                                    );
                                },

                                MultiplayerMessages::Update(u) => {
                                    self.tetris_game.update_from_network(
                                        u.tetris_game,
                                        u.timestamp,
                                        app
                                    );
                                },
                            }
                        },

                        _ => {}
                    }

                    continue;
                },

                Err(_err) => {}, //println!("client event error: {:?}", err),
            }
            break;
        }

        // pause
        let options_button = persistent.input_mapping.button("options".to_string());
        if options_button.pressed() {
            if app.is_paused() { app.resume(); }
            else { app.pause(); }
        }
    }

    fn render(
        &mut self,
        app: &mut App,
        persistent: &mut Self::PersistentData
    ) {
        // UI
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

        if app.is_paused() {
            ui::Ui::builder(window_layout).build(app);

            ui::Text::new("PAUSED", app);
            if ui::Button::new("RESUME", app).pressed {
                app.resume();
            }

            if ui::Button::new("QUIT", app).pressed {
                self.state = State::Quitting;
            }
        } else {
            match self.state {
                State::ConnectMenu => {
                    ui::Ui::builder(window_layout).build(app);

                    ui::Text::new("WATCH GAME", app);
                    ui::Input::builder("Server").build(&mut self.start_menu_server_ip, app);
                    if ui::Button::new("CONNECT", app).pressed {
                        match self.client.connect(self.start_menu_server_ip.clone()) {
                            Ok(_) => self.state = State::Connecting,
                            Err(err) => println!("[game][scenes][multiplayer_spectate] connect problem: {:?}", err),
                        }
                    }

                    if ui::Button::new("QUIT", app).pressed {
                        self.state = State::Quitting;
                    }
                },

                State::Connecting => {
                    ui::Ui::builder(window_layout).build(app);
                    ui::Text::new("CONNECTING...", app);

                    if ui::Button::new("CANCEL", app).pressed {
                        self.state = State::ConnectMenu;
                    }

                    if ui::Button::new("QUIT", app).pressed {
                        self.state = State::Quitting;
                    }
                },

                _ => {}
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
        _app: &mut App,
        _persistent: &mut Self::PersistentData,
    ) -> bool {
        //use sdl2::event::Event;
        //use sdl2::keyboard::Scancode;

        match event {
            _ => {}
        }

        false
    }

    fn transition(
        &mut self,
        _app: &mut App,
        _persistent: &mut Self::PersistentData
    ) -> Option<SceneTransition<Self::Scene>> {
        if let State::Quitting = self.state {
            Some(SceneTransition::Pop)
        } else {
            None
        }
    }

    fn on_enter(&mut self, app: &mut App, _persistent: &mut Self::PersistentData,) {
        app.restart_time_system();
    }
}

impl MultiPlayerSpectateScene {
    pub fn new(
        app: &mut App,
        persistent: &mut PersistentData
    ) -> Self {
        let rules: Rules = RotationSystem::SRS.into();
        let tetris_game = TetrisGame::new(rules, 0);

        let client = Client::new(persistent.rng.next_u64()).unwrap();

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
            state: State::ConnectMenu,
            start_menu_server_ip: "127.0.0.1:42042".to_owned(),
            tetris_game,
            client,

            playfield_pos,
            hold_piece_window_pos,
            next_pieces_preview_window_pos,
        }
    }
}
