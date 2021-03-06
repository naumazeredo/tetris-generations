mod classic;
mod custom;
mod modern;
mod options;

use crate::app::*;
use crate::linalg::{ Vec2i, Vec2 };
use crate::game::{
    playfield::Playfield,
    randomizer::RandomizerDefinedSequence,
    render::*,
    rules::{
        Rules,
        RotationSystem,
        ROTATION_SYSTEM_NAMES,
        line_clear::{LINE_CLEAR_RULE_NAMES, LineClearRule},
        topout::TopOutRule,
    },
};
use super::*;
use custom::*;

const DISPLAY_NAMES : &[&str] = &["DISPLAY 1", "DISPLAY 2", "DISPLAY 3", "DISPLAY 4", "DISPLAY 5", "DISPLAY 6"];
const SCREEN_MODES  : &[&str] = &["WINDOWED", "FULLSCREEN", "DESKTOP"];

#[derive(Debug, ImDraw)]
enum State {
    Main,

    Classic,
    ClassicLocal,
    ClassicOnline,
    ClassicOnlineSolo,
    ClassicOnlineBattle,
    ClassicOnlineSpectate,

    Modern,
    ModernLocal,
    ModernOnline,
    ModernOnlineSolo,
    ModernOnlineBattle,
    ModernOnlineSpectate,

    CustomRules,
    CustomLocal,
    CustomPreview,

    Options,
    OptionsVideo,
    OptionsAudio,
    OptionsControls,
}

#[derive(Copy, Clone, Debug)]
struct VideoInfo {
    screen_mode: FullscreenType,
    display_index: i32,
    display_mode: DisplayMode,
    window_size: (u32, u32),
    vsync: SwapInterval,
}

impl VideoInfo {
    fn load(app: &App) -> Self {
        Self {
            screen_mode:   app.window_screen_mode(),
            display_index: app.window_display_index(),
            display_mode:  app.window_display_mode(),
            window_size:   app.window_size(),
            vsync:         app.vsync(),
        }
    }
}

impl_imdraw_todo!(VideoInfo);

#[derive(Debug, ImDraw)]
pub struct MainMenuScene {
    state: State,
    video_info: VideoInfo,

    custom_rules: Rules, // @TODO list of presets (and player presets also)
    focused_rule: Option<FocusedRule>,
}

impl SceneTrait for MainMenuScene {
    type Scene = Scene;
    type PersistentData = PersistentData;

    fn update(
        &mut self,
        _dt: u64,
        _app: &mut App,
        _persistent: &mut Self::PersistentData
    ) {
    }

    fn render(
        &mut self,
        app: &mut App,
        persistent: &mut Self::PersistentData
    ) {
        match self.state {
            State::CustomRules => {},
            _ => self.render_background(app, persistent),
        }

        match self.state {
            State::Main            => self.show_main(app, persistent),

            State::Classic         => self.show_classic(app, persistent),
            State::ClassicOnline   => self.show_classic_online(app, persistent),

            State::Modern          => self.show_modern(app, persistent),
            State::ModernOnline    => self.show_modern_online(app, persistent),

            State::CustomRules     => self.show_custom_rules(app, persistent),

            State::Options         => self.show_options(app, persistent),
            State::OptionsVideo    => self.show_options_video(app, persistent),
            State::OptionsAudio    => self.show_options_audio(app, persistent),
            State::OptionsControls => self.show_options_controls(app, persistent),

            // @Remove
            _ => self.state = State::Main,
        }
    }

    fn handle_input(
        &mut self,
        event: &sdl2::event::Event,
        _app: &mut App,
        _persistent: &mut Self::PersistentData,
    ) -> bool {
        match event {
            _ => {}
        }

        false
    }

    fn transition(
        &mut self,
        app: &mut App,
        persistent: &mut Self::PersistentData
    ) -> Option<SceneTransition<Self::Scene>> {
        match self.state {
            State::ClassicLocal => {
                self.state = State::Main;

                Some(
                    SceneTransition::Push(
                        SinglePlayerScene::new(
                            persistent.rng.next_u64(),
                            RotationSystem::NRSR.into(),
                            app,
                            persistent
                        ).into()
                    )
                )
            }

            State::ModernLocal => {
                self.state = State::Main;

                Some(
                    SceneTransition::Push(
                        SinglePlayerScene::new(
                            persistent.rng.next_u64(),
                            RotationSystem::SRS.into(),
                            app,
                            persistent
                        ).into()
                    )
                )
            }

            State::ModernOnlineSolo => {
                self.state = State::Main;

                Some(
                    SceneTransition::Push(
                        MultiPlayerScene::new(
                            app,
                            persistent
                        ).into()
                    )
                )
            }

            State::ModernOnlineSpectate => {
                self.state = State::Main;

                Some(
                    SceneTransition::Push(
                        MultiPlayerSpectateScene::new(
                            app,
                            persistent
                        ).into()
                    )
                )
            }

            State::CustomLocal => {
                self.state = State::Main;

                Some(
                    SceneTransition::Push(
                        SinglePlayerScene::new(
                            persistent.rng.next_u64(),
                            self.custom_rules.clone(),
                            app,
                            persistent
                        ).into()
                    )
                )
            }

            State::CustomPreview => {
                self.state = State::CustomRules;

                Some(
                    SceneTransition::Push(
                        SinglePlayerScene::new(
                            persistent.rng.next_u64(),
                            self.custom_rules.clone(),
                            app,
                            persistent
                        ).into()
                    )
                )
            }

            _ => None
        }
    }
}

impl MainMenuScene {
    pub fn new(app: &mut App) -> Self {
        Self {
            state: State::Main,
            video_info:   VideoInfo::load(app),
            custom_rules: RotationSystem::NRSR.into(),
            focused_rule: None,
        }
    }

    fn show_main(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        let window_size = app.window_size();
        let window_size = Vec2i { x: window_size.0 as i32, y: window_size.1 as i32 };
        let menu_size = Vec2i { x: 580, y: 300 };

        // Ui
        let window_layout = ui::Layout {
            pos: Vec2i {
                x: 40,
                y: (window_size.y - menu_size.y) / 2
            },
            size: menu_size
        };
        ui::Ui::builder(window_layout).build(app); // @TODO ui::builder(...

        if ui::Button::new("CLASSIC", app).pressed {
            self.state = State::Classic;
        }

        if ui::Button::new("MODERN", app).pressed {
            self.state = State::Modern;
        }

        if ui::Button::new("CUSTOM", app).pressed {
            self.state = State::CustomRules;
        }

        if ui::Button::new("OPTIONS", app).pressed {
            self.state = State::Options;
        }

        if ui::Button::new("QUIT", app).pressed {
            app.exit();
        }
    }

    fn render_background(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        let text = "TETRIS GENERATIONS";
        let text_size = 64.;

        let text_draw_size: Vec2i = app.calculate_draw_text_size(
            text,
            text_size,
            None,
            None
        ).into();

        let window_size = app.window_size();
        let text_pos = Vec2 {
            x: ((window_size.0 as i32 - text_draw_size.x) / 2) as f32,
            y: 50.0
        };

        app.queue_draw_text(
            text,
            TransformBuilder::new().pos(text_pos).layer(100).build(),
            text_size,
            WHITE,
            None,
            None,
        );
    }
}
