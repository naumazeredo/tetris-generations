mod classic;
mod custom;
mod modern;
mod options;

use crate::app::*;
use crate::linalg::Vec2i;
use crate::game::{
    playfield::Playfield,
    randomizer::RandomizerDefinedSequence,
    render::*,
    rules::{
        Rules,
        RotationSystem,
        ROTATION_SYSTEM_NAMES,
        instance::RulesInstance,
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
    Custom,

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
        app: &mut App,
        persistent: &mut Self::PersistentData
    ) {
    }

    fn render(
        &mut self,
        app: &mut App,
        persistent: &mut Self::PersistentData
    ) {
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
                Some(
                    SceneTransition::Push(
                        SingleplayerScene::new(
                            persistent.rng.next_u64(),
                            RotationSystem::NRSR.into(),
                            app,
                            persistent
                        ).into()
                    )
                )
            }

            State::ModernLocal => {
                Some(
                    SceneTransition::Push(
                        SingleplayerScene::new(
                            persistent.rng.next_u64(),
                            RotationSystem::SRS.into(),
                            app,
                            persistent
                        ).into()
                    )
                )
            }

            /*
            State::ModernOnlineSolo =>
                Some(SceneTransition::Push(MultiPlayerScene::new(app, persistent).into())),

            State::ModernOnlineSpectate =>
                Some(SceneTransition::Push(MultiPlayerSpectateScene::new(app, persistent).into())),
            */

            _ => None
        }
    }
}

impl MainMenuScene {
    pub fn new(app: &mut App) -> Self {
        Self {
            //state: State::Main,
            state:        State::CustomRules,
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
}
