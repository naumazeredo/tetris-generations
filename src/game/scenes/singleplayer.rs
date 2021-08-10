use crate::State;
use crate::app::*;
use crate::linalg::Vec2i;

use super::*;

use crate::game::{
    rules::{
        RotationSystem,
        Rules,
        instance::RulesInstance,
    }
};

#[derive(Clone, Debug, ImDraw)]
pub struct SinglePlayerScene {
    debug_pieces_scene_opened: bool,
    rules_instance: RulesInstance,

    checkbox_test: bool,
    input_i32_test: i32,
    input_str_test: String,
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

        self.rules_instance.update(app, persistent);
    }

    fn render(
        &mut self,
        app: &mut App<'_, State>,
        persistent: &mut PersistentData
    ) {
        if app.is_paused() {
            // Ui
            let window_layout = Layout {
                pos: Vec2i { x: 12, y: 12 },
                size: Vec2i { x: 300, y: 400 }
            };
            app.new_ui(window_layout);

            app.text("test 1");
            app.indent();
            app.text("test 2");
            app.unindent();
            app.text("test 3");

            if app.button("button") {
                println!("test");
            }

            app.checkbox("checkbox", &mut self.checkbox_test);

            if self.checkbox_test {
                app.indent();
                app.text("test 4");
                app.unindent();
            }

            app.input_i32("my i32", &mut self.input_i32_test);
            app.input_str("my str", &mut self.input_str_test);

            if app.button("resume") {
                app.resume();
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
                //self.playfield.has_grid = !self.playfield.has_grid;
            }

            Event::KeyDown { scancode: Some(Scancode::D), .. } => {
                //self.rules_instance.next_level();
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
    pub fn new(seed: u64, app: &mut App<'_, State>, persistent: &mut PersistentData) -> Self {
        // rules
        let mut rules: Rules = RotationSystem::NRSR.into();
        rules.start_level = 5;
        let rules_instance = RulesInstance::new(rules, seed, app, persistent);

        Self {
            debug_pieces_scene_opened: false,
            rules_instance,

            checkbox_test: false,
            input_i32_test: 0,
            input_str_test: String::new(),
        }
    }
}
