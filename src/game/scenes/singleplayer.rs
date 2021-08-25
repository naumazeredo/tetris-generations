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

    seed: u64,

    checkbox_test: bool,
    input_i32_test: i32,
    input_u32_test: u32,
    input_str_test: String,
    combobox_index_test: usize,

    music_id: MusicId,
    music_playing: bool,
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
                size: Vec2i { x: 600, y: 628 }
            };
            app.new_ui(window_layout);

            app.text("RULES");
            app.checkbox("hard drop", &mut self.rules_instance.rules.has_hard_drop);
            if self.rules_instance.rules.has_hard_drop {
                app.indent();
                app.checkbox("hard drop lock", &mut self.rules_instance.rules.has_hard_drop_lock);
                app.unindent();
            }

            app.checkbox("soft drop", &mut self.rules_instance.rules.has_soft_drop);
            if self.rules_instance.rules.has_soft_drop {
                app.indent();
                app.checkbox("soft drop lock", &mut self.rules_instance.rules.has_soft_drop_lock);
                app.unindent();
            }

            app.checkbox("hold piece", &mut self.rules_instance.rules.has_hold_piece);
            if self.rules_instance.rules.has_hold_piece {
                app.indent();
                app.checkbox("reset rotation", &mut self.rules_instance.rules.hold_piece_reset_rotation);
                app.unindent();
            }

            app.checkbox("ghost piece", &mut self.rules_instance.rules.has_ghost_piece);

            app.checkbox("spawn drop", &mut self.rules_instance.rules.spawn_drop);

            app.checkbox("IRS", &mut self.rules_instance.rules.has_initial_rotation_system);
            app.checkbox("IHS", &mut self.rules_instance.rules.has_initial_hold_system);

            app.slider_u8("spawn row", &mut self.rules_instance.rules.spawn_row, 0, 24);
            app.slider_u8("next pieces", &mut self.rules_instance.rules.next_pieces_preview_count, 0, 6);

            //pub line_clear_rule: LineClearRule,
            //pub top_out_rule: TopOutRule,

            //app.combobox("combobox", &mut self.combobox_index_test, combobox::COMBOBOX_TEST_OPTIONS);

            // @TODO ui for time values
            app.slider_u64("DAS", &mut self.rules_instance.rules.das_repeat_delay, 0, 500_000);
            app.slider_u64("ARR", &mut self.rules_instance.rules.das_repeat_interval, 0, 500_000);

            app.slider_u64("soft drop interval", &mut self.rules_instance.rules.soft_drop_interval, 0, 500_000);
            app.slider_u64("line clear delay", &mut self.rules_instance.rules.line_clear_delay, 0, 500_000);

            //pub gravity_curve: GravityCurve,
            //pub scoring_curve: ScoringRule,
            //pub level_curve: LevelCurve, // @Maybe rename to difficulty curve
            //pub start_level: u8,
            app.slider_u8("start level", &mut self.rules_instance.rules.start_level, 1, 255);

            app.slider_u64("entry delay", &mut self.rules_instance.rules.entry_delay, 0, 2_000_000);

            //pub lock_delay: LockDelayRule,
            //pub rotation_system: RotationSystem,

            //pub randomizer_type: RandomizerType,
            //app.input_u64_stretch("seed", &mut self.seed);

            if app.button("Restart") {
                println!("restart");
            }

            if app.button("Resume") {
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

            Event::KeyDown { scancode: Some(Scancode::F), .. } => {
                app.play_music(self.music_id);
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

        let music_id = app.load_music("assets/sfx/Original-Tetris-theme.opus");

        Self {
            debug_pieces_scene_opened: false,
            rules_instance,

            seed,

            checkbox_test: false,
            input_i32_test: 0,
            input_u32_test: 0,
            input_str_test: String::new(),
            combobox_index_test: 0,

            music_id,
            music_playing: false,
        }
    }
}
