use crate::app::*;
use crate::linalg::Vec2i;

use super::*;

use crate::game::{
    rules::{
        RotationSystem,
        Rules,
        instance::RulesInstance,
    },
    network::{MultiplayerMessages, Connect, Update},
};

#[derive(Debug, ImDraw)]
pub struct MultiPlayerScene {
    quit: bool,

    rules_instance: RulesInstance,
    seed: u64, // @TODO move this to RulesInstance

    music_id: MusicId,

    server: Server,
}

impl SceneTrait for MultiPlayerScene {
    fn update(
        &mut self,
        app: &mut App,
        persistent: &mut PersistentData
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
                                instance: self.rules_instance.to_network(),
                                rotation_system: self.rules_instance.rules().rotation_system,
                                randomizer: self.rules_instance.randomizer().clone(),
                            };

                            let message = MultiplayerMessages::Connect(connect);
                            self.server.send(client_id, message).unwrap();
                        }

                        _ => {}
                    }

                    if let ServerEvent::ClientHeartbeat(client_id) = e {
                        let update = Update {
                            timestamp: app.game_timestamp(),
                            instance: self.rules_instance.to_network(),
                        };

                        let message = MultiplayerMessages::Update(update);
                        self.server.send(client_id, message).unwrap();
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
            self.rules_instance.update(app, &persistent.input_mapping);
        }
    }

    fn render(
        &mut self,
        app: &mut App,
        persistent: &mut PersistentData
    ) {
        if app.is_paused() {
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
            app.new_ui(window_layout);

            // Ui
            app.text("PAUSED");
            app.text(&format!("IP: {}", self.server.addr()));

            if app.button("RESUME") {
                app.resume();
            }

            if app.button("RESTART") {
                println!("restart");
            }

            if app.button("QUIT") {
                self.quit = true;
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
        app: &mut App,
        _persistent: &mut PersistentData,
        event: &sdl2::event::Event
    ) -> bool {
        use sdl2::event::Event;
        use sdl2::keyboard::Scancode;

        match event {
            Event::KeyDown { scancode: Some(Scancode::F), .. } => {
                app.play_music(self.music_id);
            }

            _ => {}
        }

        false
    }

    fn transition(&mut self, _app: &mut App, _persistent: &mut PersistentData) -> Option<SceneTransition> {
        if self.quit { Some(SceneTransition::Pop) } else { None }
    }

    fn on_enter(&mut self, app: &mut App, _persistent: &mut PersistentData,) {
        app.restart_time_system();
        //app.play_music(self.music_id);
    }
}

impl MultiPlayerScene {
    pub fn new(app: &mut App, persistent: &mut PersistentData) -> Self {
        // rules
        let seed = app.system_time();
        let rules: Rules = RotationSystem::SRS.into();
        let rules_instance = RulesInstance::new(rules, seed, app, persistent);

        let music_id = app.load_music("assets/sfx/Original-Tetris-theme.ogg");

        let server = Server::new("127.0.0.1:42042").unwrap();

        Self {
            quit: false,

            rules_instance,
            seed,

            music_id,

            server,
        }
    }
}
