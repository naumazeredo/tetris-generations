use crate::app::*;
use crate::linalg::Vec2i;
use crate::game::rules::{RotationSystem, Rules};

use super::*;

#[derive(Clone, Debug, ImDraw)]
pub struct SinglePlayerStartMenuScene {
    start_singleplayer_game: bool,
    go_back: bool,

    rules: Rules,
}

impl SceneTrait for SinglePlayerStartMenuScene {
    fn update(
        &mut self,
        _app: &mut App,
        _persistent: &mut PersistentData
    )
    {}

    fn render(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        let window_size = app.window_size();
        let window_size = Vec2i { x: window_size.0 as i32, y: window_size.1 as i32 };
        let menu_size = Vec2i { x: 600, y: 700 };

        // Ui
        let window_layout = Layout {
            pos: Vec2i {
                x: 40,
                y: (window_size.y - menu_size.y) / 2
            },
            size: menu_size
        };
        app.new_ui(window_layout);

        app.text("RULES");
        app.checkbox("hard drop", &mut self.rules.has_hard_drop);
        if self.rules.has_hard_drop {
            app.indent();
            app.checkbox("hard drop lock", &mut self.rules.has_hard_drop_lock);
            app.unindent();
        }

        app.checkbox("soft drop", &mut self.rules.has_soft_drop);
        if self.rules.has_soft_drop {
            app.indent();
            app.checkbox("soft drop lock", &mut self.rules.has_soft_drop_lock);
            app.unindent();
        }

        app.checkbox("hold piece", &mut self.rules.has_hold_piece);
        if self.rules.has_hold_piece {
            app.indent();
            app.checkbox("reset rotation", &mut self.rules.hold_piece_reset_rotation);
            app.unindent();
        }

        app.checkbox("ghost piece", &mut self.rules.has_ghost_piece);

        app.checkbox("spawn drop", &mut self.rules.spawn_drop);

        app.checkbox("IRS", &mut self.rules.has_initial_rotation_system);
        app.checkbox("IHS", &mut self.rules.has_initial_hold_system);

        app.slider_u8("spawn row", &mut self.rules.spawn_row, 0, 24);
        app.slider_u8("next pieces", &mut self.rules.next_pieces_preview_count, 0, 6);

        //pub line_clear_rule: LineClearRule,
        //pub top_out_rule: TopOutRule,

        // @TODO ui for time values
        app.slider_u64("DAS", &mut self.rules.das_repeat_delay, 0, 500_000);
        app.slider_u64("ARR", &mut self.rules.das_repeat_interval, 0, 500_000);

        app.slider_u64("soft drop interval", &mut self.rules.soft_drop_interval, 0, 500_000);
        app.slider_u64("line clear delay", &mut self.rules.line_clear_delay, 0, 500_000);

        //pub gravity_curve: GravityCurve,
        //pub scoring_curve: ScoringRule,
        //pub level_curve: LevelCurve, // @Maybe rename to difficulty curve

        app.slider_u8("start level", &mut self.rules.start_level, self.rules.minimum_level, 50);
        app.slider_u64("entry delay", &mut self.rules.entry_delay, 0, 2_000_000);

        //pub lock_delay: LockDelayRule,
        //pub rotation_system: RotationSystem,

        //pub randomizer_type: RandomizerType,
        //app.input_u64_stretch("seed", &mut self.seed);

        if app.button("START") {
            self.start_singleplayer_game = true;
        }

        if app.button("BACK") {
            self.go_back = true;
        }
    }

    fn handle_input(
        &mut self,
        _app: &mut App,
        _persistent: &mut PersistentData,
        event: &sdl2::event::Event
    ) -> bool {
        match event {
            _ => {}
        }

        false
    }

    fn transition(&mut self, app: &mut App, persistent: &mut PersistentData) -> Option<SceneTransition> {
        if self.start_singleplayer_game {
            self.start_singleplayer_game = false;

            let seed = app.system_time();

            Some(
                SceneTransition::Swap(
                    SinglePlayerScene::new(seed, self.rules.clone(), app, persistent).into()
                )
            )
        } else if self.go_back {
            Some(SceneTransition::Pop)
        } else {
            None
        }
    }
}

impl SinglePlayerStartMenuScene {
    pub fn new() -> Self {
        Self {
            start_singleplayer_game: false,
            go_back: false,

            rules: RotationSystem::NRSR.into(),
        }
    }
}
