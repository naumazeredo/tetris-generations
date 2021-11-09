use crate::app::*;
use crate::linalg::Vec2i;

use super::*;

use crate::game::{
    rules::{
        Rules,
        instance::RulesInstance,
    }
};

#[derive(Clone, Debug, ImDraw)]
pub struct SinglePlayerScene {
    debug_pieces_scene_opened: bool,
    quit: bool,

    rules_instance: RulesInstance,
    seed: u64, // @TODO move this to RulesInstance

    music_id: MusicId,
}

impl SceneTrait for SinglePlayerScene {
    fn update(
        &mut self,
        app: &mut App,
        persistent: &mut PersistentData
    ) {
        // pause
        let options_button = persistent.input_mapping.button("options".to_string());
        if options_button.pressed() {
            if app.is_paused() { app.resume(); }
            else { app.pause(); }
        }

        if app.is_paused() { return; }

        self.rules_instance.update(app, &persistent.input_mapping);
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
            Text::new("PAUSED", app);
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
            Event::KeyDown { scancode: Some(Scancode::F3), .. } => {
                app.set_time_scale(0.1);
            }

            Event::KeyDown { scancode: Some(Scancode::F4), .. } => {
                app.set_time_scale(1.0);
            }

            Event::KeyDown { scancode: Some(Scancode::F10), .. } => {
                self.debug_pieces_scene_opened = true;
            }

            Event::KeyDown { scancode: Some(Scancode::W), .. } => {
                //self.playfield.has_grid = !self.playfield.has_grid;
            }

            Event::KeyDown { scancode: Some(Scancode::D), .. } => {
                //self.rules_instance.next_level();
            }

            Event::KeyDown { scancode: Some(Scancode::F), .. } => {
                app.play_music(self.music_id);
            }

            _ => {}
        }

        false
    }

    fn transition(&mut self, _app: &mut App, _persistent: &mut PersistentData) -> Option<SceneTransition> {
        if self.debug_pieces_scene_opened {
            self.debug_pieces_scene_opened = false;
            Some(SceneTransition::Push(Scene::DebugPiecesScene(DebugPiecesScene::new())))
        } else if self.quit {
            Some(SceneTransition::Pop)
        } else {
            None
        }
    }

    fn on_enter(&mut self, app: &mut App, _persistent: &mut PersistentData,) {
        app.restart_time_system();
        //app.play_music(self.music_id);
    }
}

impl SinglePlayerScene {
    pub fn new(seed: u64, rules: Rules, app: &mut App, persistent: &mut PersistentData) -> Self {
        // rules
        let rules_instance = RulesInstance::new(rules, seed, app, persistent);

        let music_id = app.load_music("assets/sfx/Original-Tetris-theme.ogg");

        Self {
            debug_pieces_scene_opened: false,
            quit: false,

            rules_instance,
            seed,

            music_id,
        }
    }
}
