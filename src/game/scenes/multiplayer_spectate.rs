use crate::app::*;
use crate::linalg::Vec2i;
use crate::rand_core::RngCore;

use super::*;

use crate::game::{
    network::MultiplayerMessages,
    rules::{
        RotationSystem,
        Rules,
        instance::RulesInstance,
    },
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
    rules_instance: RulesInstance,
}

impl SceneTrait for MultiPlayerSpectateScene {
    fn update(
        &mut self,
        app: &mut App,
        persistent: &mut PersistentData
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
                                    self.rules_instance = RulesInstance::from_network(
                                        c.instance,
                                        c.rotation_system.into(),
                                        c.randomizer,
                                        c.timestamp,
                                        app,
                                        persistent
                                    );
                                },

                                MultiplayerMessages::Update(u) => {
                                    self.rules_instance.update_from_network(
                                        u.instance,
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
        persistent: &mut PersistentData
    ) {
        // UI
        let window_size = app.window_size();
        let window_size = Vec2i { x: window_size.0 as i32, y: window_size.1 as i32 };
        let menu_size = Vec2i { x: 600, y: 300 };

        // Ui
        let window_layout = Layout {
            pos: Vec2i {
                x: (window_size.x - menu_size.x) / 2,
                y: (window_size.y - menu_size.y) / 2,
            },
            size: menu_size
        };

        if app.is_paused() {
            app.new_ui(window_layout);

            app.text("PAUSED");
            if app.button("RESUME") {
                app.resume();
            }

            if app.button("QUIT") {
                self.state = State::Quitting;
            }
        } else {
            match self.state {
                State::ConnectMenu => {
                    app.new_ui(window_layout);

                    app.text("WATCH GAME");
                    ui::Input::new("Server").build(&mut self.start_menu_server_ip, app);
                    if app.button("CONNECT") {
                        match self.client.connect(self.start_menu_server_ip.clone()) {
                            Ok(_) => self.state = State::Connecting,
                            Err(err) => println!("[game][scenes][multiplayer_spectate] connect problem: {:?}", err),
                        }
                    }

                    if app.button("QUIT") {
                        self.state = State::Quitting;
                    }
                },

                State::Connecting => {
                    app.new_ui(window_layout);
                    app.text("CONNECTING...");

                    if app.button("CANCEL") {
                        self.state = State::ConnectMenu;
                    }

                    if app.button("QUIT") {
                        self.state = State::Quitting;
                    }
                },

                _ => {}
            }
        }

        /*
        let pixel_scale = persistent.pixel_scale;

        let window_size = app.video_system.window.size();

        let playfield_pixel_size = Vec2i {
            x: (pixel_scale as f32 * BLOCK_SCALE * self.rules_instance.playfield_grid_size().x as f32) as i32,
            y: (pixel_scale as f32 * BLOCK_SCALE * PLAYFIELD_VISIBLE_HEIGHT as f32) as i32,
        };

        let playfield_pos = Vec2i {
            x: (window_size.0 as i32 - playfield_pixel_size.x) / 2,
            y: (window_size.1 as i32 - playfield_pixel_size.y) / 2,
        };
        */

        app.queue_draw_text(
            &format!("time: {:.2}", app.game_time()),
            &TransformBuilder::new().pos_xy(10.0, 42.0).layer(800).build(),
            32.,
            WHITE
        );

        app.queue_draw_text(
            &format!("level: {}", self.rules_instance.level()),
            &TransformBuilder::new().pos_xy(10.0, 84.0).layer(800).build(),
            32.,
            WHITE
        );

        app.queue_draw_text(
            &format!("score: {}", self.rules_instance.score()),
            &TransformBuilder::new().pos_xy(10.0, 126.0).layer(800).build(),
            32.,
            WHITE
        );

        app.queue_draw_text(
            &format!("lines: {}", self.rules_instance.total_lines_cleared()),
            &TransformBuilder::new().pos_xy(10.0, 168.0).layer(800).build(),
            32.,
            WHITE
        );

        self.rules_instance.render(app, persistent);
    }

    fn handle_input(
        &mut self,
        _app: &mut App,
        _persistent: &mut PersistentData,
        event: &sdl2::event::Event
    ) -> bool {
        //use sdl2::event::Event;
        //use sdl2::keyboard::Scancode;

        match event {
            _ => {}
        }

        false
    }

    fn transition(&mut self, _app: &mut App, _persistent: &mut PersistentData) -> Option<SceneTransition> {
        if let State::Quitting = self.state {
            Some(SceneTransition::Pop)
        } else {
            None
        }
    }

    fn on_enter(&mut self, app: &mut App, _persistent: &mut PersistentData,) {
        app.restart_time_system();
    }
}

impl MultiPlayerSpectateScene {
    pub fn new(app: &mut App, persistent: &mut PersistentData) -> Self {
        let rules: Rules = RotationSystem::SRS.into();
        let rules_instance = RulesInstance::new(rules, 0, app, persistent);

        let mut rng = rand_pcg::Pcg64::new(app.system_time() as u128, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);
        let client = Client::new(rng.next_u64()).unwrap();

        Self {
            state: State::ConnectMenu,
            start_menu_server_ip: "127.0.0.1:42042".to_owned(),
            rules_instance,
            client,
        }
    }
}
