use crate::app::*;
use crate::linalg::Vec2i;

use super::*;

use crate::game::{
    input::*,
    rules::{
        Rules,
        instance::RulesInstance,
    },
    render::*,
};

#[derive(Debug, ImDraw)]
pub struct SingleplayerScene {
    rules_instance: RulesInstance,
    music_id: MusicId,
    server: Option<Server>,
    quit: bool,

    playfield_pos: Vec2i,
    hold_piece_window_pos: Vec2i,
    next_pieces_preview_window_pos: Vec2i,
}

impl SceneTrait for SingleplayerScene {
    type Scene = Scene;
    type PersistentData = PersistentData;

    fn update(
        &mut self,
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

        self.rules_instance.update(app.last_frame_duration(), &persistent.input_mapping, app);
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

        // @TODO self.rules_instance.render(&instance_style, app, persistent);
        self.rules_instance.update_animations();
        self.rules_instance.render_playfield(self.playfield_pos, true, &mut app.batch(), persistent);
        self.rules_instance.render_hold_piece(self.hold_piece_window_pos, true, &mut app.batch(), persistent);
        self.rules_instance.render_next_pieces_preview(self.next_pieces_preview_window_pos, 0, true, &mut app.batch(), persistent);

        app.queue_draw_text(
            &format!("time: {:.2}", to_seconds(self.rules_instance.timestamp())),
            &TransformBuilder::new().pos_xy(10.0, 42.0).layer(800).build(),
            32.,
            WHITE,
            None,
            None,
        );

        app.queue_draw_text(
            &format!("level: {}", self.rules_instance.level()),
            &TransformBuilder::new().pos_xy(10.0, 84.0).layer(800).build(),
            32.,
            WHITE,
            None,
            None,
        );

        app.queue_draw_text(
            &format!("score: {}", self.rules_instance.score()),
            &TransformBuilder::new().pos_xy(10.0, 126.0).layer(800).build(),
            32.,
            WHITE,
            None,
            None,
        );

        app.queue_draw_text(
            &format!("lines: {}", self.rules_instance.total_lines_cleared()),
            &TransformBuilder::new().pos_xy(10.0, 168.0).layer(800).build(),
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
                //self.rules_instance.next_level();
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

impl SingleplayerScene {
    pub fn new(
        seed: u64,
        rules: Rules,
        app: &mut App,
        persistent: &mut PersistentData
    ) -> Self {
        // rules
        let rules_instance = RulesInstance::new(rules, seed);

        let music_id = app.load_music("assets/sfx/Original-Tetris-theme.ogg");

        // @Refactor use InstanceStyle
        // Playfield rendering
        let playfield_draw_size = get_draw_playfield_size(
            &rules_instance.playfield(),
            persistent.pixel_scale,
            true,
        );

        let window_size = app.window_size();
        let playfield_pos = Vec2i {
            x: (window_size.0 as i32 - playfield_draw_size.x) / 2,
            y: (window_size.1 as i32 - playfield_draw_size.y) / 2,
        };

        let hold_window_size = rules_instance.hold_piece_window_size(true, persistent);
        let hold_piece_window_pos =
            playfield_pos +
            Vec2i { x: -20, y: 0 } +
            Vec2i { x: -hold_window_size.x, y: 0 };

        let next_pieces_preview_window_pos =
            playfield_pos +
            Vec2i { x: 20, y: 0 } +
            Vec2i { x: playfield_draw_size.x, y: 0 };

        Self {
            rules_instance,
            music_id,
            server: None,
            quit: false,

            playfield_pos,
            hold_piece_window_pos,
            next_pieces_preview_window_pos,
        }
    }
}
