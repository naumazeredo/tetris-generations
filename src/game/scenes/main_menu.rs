use crate::app::*;
use crate::linalg::{Vec2, Vec2i};

use crate::game::rules::{
    Rules,
    RotationSystem,
    ROTATION_SYSTEM_NAMES,
    line_clear::{LINE_CLEAR_RULE_NAMES, LineClearRule},
    topout::TopOutRule,
};
use super::*;

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

#[derive(Copy, Clone, Debug)]
enum FocusedRule {
    RotationSystem,

    HardDrop,
    HardDropLock,

    SoftDrop,
    SoftDropLock,

    LockDelayRule,

    Ghost,
    Hold,
    HoldResetRotation,

    InitialRotation,
    InitialHold,
    SpawnDrop,
    SpawnRow,
    NextPiecesPreview,

    LineClearRule,
    TopOutRule,

    DelayedAutoShift,
    AutoRepeatRate,

    SoftDropInterval,
    LineClearDelay,

    GravityCurve,
    ScoringCurve,
    LevelCurve,
    StartLevel,
    MinimumLevel,

    EntryDelay,

    RandomizerType,
}

impl_imdraw_todo!(FocusedRule);

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
        _app: &mut App,
        _persistent: &mut Self::PersistentData
    )
    {}

    fn render(
        &mut self,
        app: &mut App,
        persistent: &mut Self::PersistentData
    ) {
        match self.state {
            State::Main => self.show_main(app, persistent),

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

    fn show_classic(
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
        ui::Ui::builder(window_layout).build(app);
        ui::Text::builder("CLASSIC").build(app);

        if ui::Button::new("LOCAL", app).pressed {
            self.state = State::ClassicLocal;
        }

        if ui::Button::new("ONLINE", app).pressed {
            self.state = State::ClassicOnline;
        }

        if ui::Button::new("BACK", app).pressed {
            self.state = State::Main;
        }
    }

    fn show_modern(
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
        ui::Ui::builder(window_layout).build(app);
        ui::Text::builder("MODERN").build(app);

        if ui::Button::new("LOCAL", app).pressed {
            self.state = State::ModernLocal;
        }

        if ui::Button::new("ONLINE", app).pressed {
            self.state = State::ModernOnline;
        }

        if ui::Button::new("BACK", app).pressed {
            self.state = State::Main;
        }
    }

    fn show_options(
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
        ui::Ui::builder(window_layout).build(app);
        ui::Text::builder("OPTIONS").build(app);

        if ui::Button::new("VIDEO", app).pressed {
            self.state = State::OptionsVideo;
        }

        if ui::Button::new("AUDIO", app).pressed {
            self.state = State::OptionsAudio;
        }

        if ui::Button::new("CONTROLS", app).pressed {
            self.state = State::OptionsControls;
        }

        if ui::Button::new("BACK", app).pressed {
            self.state = State::Main;
        }
    }

    fn show_options_video(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        // @TODO cache strings and dyn arrays

        // @Important reloading video info every time. In case we don't we need to check which UI
        // elements changed to know what/when to reload this info
        self.video_info = VideoInfo::load(app);

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

        ui::Ui::builder(window_layout).build(app);
        ui::Text::builder("OPTIONS - VIDEO").build(app);

        //Text::new("OPTIONS - VIDEO", app);

        // @TODO most video options should have a confirm button in 15 sec or change back if the
        //       the user can't click it. This will avoid issues

        // Fullscreen options
        let mut screen_mode_index = match self.video_info.screen_mode {
            FullscreenType::Off     => 0,
            FullscreenType::True    => 1,
            FullscreenType::Desktop => 2,
        };

        let screen_mode_combobox =
            ui::Combobox::builder("SCREEN MODE", &SCREEN_MODES).build(&mut screen_mode_index, app);

        if screen_mode_combobox.changed {
            self.video_info.screen_mode = match screen_mode_index {
                0 => FullscreenType::Off,
                1 => FullscreenType::True,
                _ => FullscreenType::Desktop,
            };

            app.set_window_screen_mode(self.video_info.screen_mode);
        }

        // Display
        let num_displays = app.num_displays();
        let mut display_index = self.video_info.display_index as usize;
        let display_state = ui::Combobox::builder("DISPLAY", &DISPLAY_NAMES[..num_displays])
            .build(&mut display_index, app);

        if display_state.changed {
            app.move_window_to_display(display_index as u32);
        }

        // Resolution and refresh rate
        let window_sizes_and_rates = app.available_window_sizes_and_rates();
        let is_fullscreen = app.window_screen_mode() == FullscreenType::True;
        let is_desktop    = app.window_screen_mode() == FullscreenType::Desktop;

        // Resolution
        let current_size = (self.video_info.window_size.0, self.video_info.window_size.1);

        let mut window_sizes_strs = window_sizes_and_rates
            .iter()
            .map(|size_and_rate| format!("{}x{}", size_and_rate.0.0, size_and_rate.0.1))
            .collect::<Vec<String>>();

        let mut size_index = window_sizes_and_rates
            .iter()
            .position(|size_and_rate| size_and_rate.0 == current_size)
            .unwrap_or_else(|| {
                window_sizes_strs.push("(CUSTOM)".to_owned());
                window_sizes_strs.len()-1
            });

        let window_size_state = ui::Combobox::builder("RESOLUTION", &window_sizes_strs)
            .disabled(is_desktop)
            .build(&mut size_index, app);

        if window_size_state.changed {
            // size_index == len only if it was manually resized (or OS resized it)
            if size_index < window_sizes_and_rates.len() {
                let size = window_sizes_and_rates[size_index].0;

                if !is_fullscreen {
                    app.set_window_size(size.0, size.1);
                } else {
                    if !(window_sizes_and_rates[size_index].1).contains(&(self.video_info.display_mode.refresh_rate as u32)) {
                        self.video_info.display_mode.refresh_rate = (window_sizes_and_rates[size_index].1)[0] as i32;
                    }

                    self.video_info.display_mode.w = size.0 as i32;
                    self.video_info.display_mode.h = size.1 as i32;
                    app.set_window_display_mode(self.video_info.display_mode);
                }
            }
        }

        // Refresh rate
        let refresh_rates_strs;
        let mut rate_index;
        if is_fullscreen && size_index < window_sizes_and_rates.len() {
            rate_index = window_sizes_and_rates[size_index].1
                .iter()
                .position(|&rate| rate == self.video_info.display_mode.refresh_rate as u32)
                .unwrap();

            refresh_rates_strs = window_sizes_and_rates[size_index].1
                .iter()
                .map(|rate| format!("{} Hz", rate))
                .collect::<Vec<String>>();
        } else {
            rate_index = 0;
            refresh_rates_strs = vec![format!("{} Hz", self.video_info.display_mode.refresh_rate)];
        }

        let refresh_rates_state = ui::Combobox::builder("REFRESH RATE", &refresh_rates_strs)
            .disabled(!is_fullscreen)
            .build(&mut rate_index, app);

        if refresh_rates_state.changed && is_fullscreen {
            let rate = (window_sizes_and_rates[size_index].1)[rate_index];
            self.video_info.display_mode.refresh_rate = rate as i32;
            app.set_window_display_mode(self.video_info.display_mode);
        }

        // VSync
        let mut vsync            = self.video_info.vsync != SwapInterval::Immediate;
        let mut adaptative_vsync = self.video_info.vsync == SwapInterval::LateSwapTearing;

        let vsync_checkbox =
            ui::Checkbox::builder("VSYNC")
            .build(&mut vsync, app);

        let adaptative_vsync_checkbox =
            ui::Checkbox::builder("  ADAPTATIVE")
            .disabled(!vsync)
            .build(&mut adaptative_vsync, app);

        if vsync_checkbox.changed || adaptative_vsync_checkbox.changed {
            self.video_info.vsync = if vsync {
                if adaptative_vsync {
                    SwapInterval::LateSwapTearing
                } else {
                    SwapInterval::VSync
                }
            } else {
                SwapInterval::Immediate
            };

            app.set_vsync(self.video_info.vsync);
        }

        /*
        if ui::Button::new("APPLY", app).pressed {
        }
        */

        if ui::Button::new("BACK", app).pressed {
            self.state = State::Options;
        }
    }

    fn show_options_audio(
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
        ui::Ui::builder(window_layout).build(app);
        ui::Text::builder("OPTIONS - AUDIO").build(app);

        // Music volume
        let mut music_volume = (app.music_volume() * 255.0) as i32;
        let music_volume_slider =
            ui::SliderI32::builder("MUSIC", 0, 255)
            .build(&mut music_volume, app);

        if music_volume_slider.changed {
            app.set_music_volume(music_volume as f32 / 255.0);
        }

        // SFX volume
        let mut sfx_volume = (app.sfx_volume() * 255.0) as i32;
        let sfx_volume_slider =
            ui::SliderI32::builder("SFX", 0, 255)
                .build(&mut sfx_volume, app);

        if sfx_volume_slider.changed {
            app.set_sfx_volume(sfx_volume as f32 / 255.0);
        }

        if ui::Button::new("BACK", app).pressed {
            self.state = State::Options;
        }
    }

    fn show_options_controls(
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
        ui::Ui::builder(window_layout).build(app);
        ui::Text::builder("OPTIONS - CONTROLS").build(app);

        if ui::Button::new("BACK", app).pressed {
            self.state = State::Options;
        }
    }

    fn show_classic_online(
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
        ui::Ui::builder(window_layout).build(app);
        ui::Text::builder("CLASSIC ONLINE").build(app);

        if ui::Button::new("SOLO", app).pressed {
            self.state = State::ClassicOnlineSolo;
        }

        if ui::Button::new("BATTLE", app).pressed {
            self.state = State::ClassicOnlineBattle;
        }

        if ui::Button::new("SPECTATE", app).pressed {
            self.state = State::ClassicOnlineSpectate;
        }

        if ui::Button::new("BACK", app).pressed {
            self.state = State::Classic;
        }
    }

    fn show_modern_online(
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
        ui::Ui::builder(window_layout).build(app);
        ui::Text::builder("MODERN ONLINE").build(app);

        if ui::Button::new("SOLO", app).pressed {
            self.state = State::ModernOnlineSolo;
        }

        if ui::Button::new("BATTLE", app).pressed {
            self.state = State::ModernOnlineBattle;
        }

        if ui::Button::new("SPECTATE", app).pressed {
            self.state = State::ModernOnlineSpectate;
        }

        if ui::Button::new("BACK", app).pressed {
            self.state = State::Modern;
        }
    }

    fn show_custom_rules(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        //let window_size = app.window_size();
        //let window_size = Vec2i { x: window_size.0 as i32, y: window_size.1 as i32 };
        let menu_size = Vec2i { x: 580, y: 880 };

        // Ui
        let window_layout = ui::Layout {
            pos: Vec2i { x: 40, y: 40, },
            size: menu_size
        };
        ui::Ui::builder(window_layout)
            .fixed_height()
            .build(app);
        ui::Text::builder("CUSTOM GAME").build(app);

        let mut rules_box_placer = ui::PagedBox::builder(26).build(app).unwrap();
        {
            // @TODO combobox for presets
            let mut preset_index = 0;
            ui::Combobox::builder("PRESET", &["(CUSTOM)"])
                .build_with_placer(&mut preset_index, &mut rules_box_placer, app);

            // Rotation Systems
            // @TODO ComboboxEnum
            let mut rotation_system = match self.custom_rules.rotation_system {
                RotationSystem::Original => 0,
                RotationSystem::NRSL     => 1,
                RotationSystem::NRSR     => 2,
                RotationSystem::Sega     => 3,
                RotationSystem::ARS      => 4,
                RotationSystem::SRS      => 5,
                RotationSystem::DTET     => 6,
            };
            let state = ui::Combobox::builder("ROTATION SYSTEM", ROTATION_SYSTEM_NAMES)
                .build_with_placer(&mut rotation_system, &mut rules_box_placer, app);
            self.custom_rules.rotation_system = match rotation_system {
                0 => RotationSystem::Original,
                1 => RotationSystem::NRSL,
                2 => RotationSystem::NRSR,
                3 => RotationSystem::Sega,
                4 => RotationSystem::ARS,
                5 => RotationSystem::SRS,
                _ => RotationSystem::DTET,
            };

            if state.focused { self.focused_rule = Some(FocusedRule::RotationSystem); }

            // Hard drop

            let state = ui::Checkbox::builder("HARD DROP")
                .build_with_placer(&mut self.custom_rules.has_hard_drop, &mut rules_box_placer, app);

            if state.focused { self.focused_rule = Some(FocusedRule::HardDrop); }

            ui::Checkbox::builder("  HARD DROP LOCK")
                .disabled(!self.custom_rules.has_hard_drop)
                .build_with_placer(&mut self.custom_rules.has_hard_drop_lock, &mut rules_box_placer, app);

            // Soft drop

            let state = ui::Checkbox::builder("SOFT DROP")
                .build_with_placer(&mut self.custom_rules.has_soft_drop, &mut rules_box_placer, app);

            if state.focused { self.focused_rule = Some(FocusedRule::SoftDrop); }

            ui::Checkbox::builder("  SOFT DROP LOCK")
                .disabled(!self.custom_rules.has_soft_drop)
                .build_with_placer(&mut self.custom_rules.has_soft_drop_lock, &mut rules_box_placer, app);

            ui::SliderU64::builder("  INTERVAL", 0, 500_000)
                .build_with_placer(&mut self.custom_rules.soft_drop_interval, &mut rules_box_placer, app);

            ui::Checkbox::builder("HOLD PIECE")
                .build_with_placer(&mut self.custom_rules.has_hold_piece, &mut rules_box_placer, app);
            ui::Checkbox::builder("  RESET ROTATION")
                .disabled(!self.custom_rules.has_hold_piece)
                .build_with_placer(&mut self.custom_rules.hold_piece_reset_rotation, &mut rules_box_placer, app);

            ui::Checkbox::builder("GHOST PIECE")
                .build_with_placer(&mut self.custom_rules.has_ghost_piece, &mut rules_box_placer, app);

            ui::Checkbox::builder("SPAWN DROP")
                .build_with_placer(&mut self.custom_rules.spawn_drop, &mut rules_box_placer, app);

            ui::Checkbox::builder("IRS")
                .build_with_placer(&mut self.custom_rules.has_initial_rotation_system, &mut rules_box_placer, app);
            ui::Checkbox::builder("IHS")
                .build_with_placer(&mut self.custom_rules.has_initial_hold_system, &mut rules_box_placer, app);

            ui::SliderU8::builder("SPAWN ROW", 0, 24)
                .build_with_placer(&mut self.custom_rules.spawn_row, &mut rules_box_placer, app);
            ui::SliderU8::builder("NEXT PIECES", 0, 8)
                .build_with_placer(&mut self.custom_rules.next_pieces_preview_count, &mut rules_box_placer, app);

            // @TODO ComboboxEnum
            let mut line_clear = match self.custom_rules.line_clear_rule {
                LineClearRule::Naive   => 0,
                LineClearRule::Sticky  => 1,
                LineClearRule::Cascade => 2,
            };
            ui::Combobox::builder("LINE CLEAR", LINE_CLEAR_RULE_NAMES)
                .build_with_placer(&mut line_clear, &mut rules_box_placer, app);
            self.custom_rules.line_clear_rule = match line_clear {
                0 => LineClearRule::Naive,
                1 => LineClearRule::Sticky,
                _ => LineClearRule::Cascade,
            };

            ui::SliderU64::builder("  DELAY", 0, 1_000_000)
                .build_with_placer(&mut self.custom_rules.line_clear_delay, &mut rules_box_placer, app);

            ui::Text::builder("TOP OUT RULE")
                .build_with_placer(&mut rules_box_placer, app);

            let mut block_out = self.custom_rules.top_out_rule.contains(TopOutRule::BLOCK_OUT);
            let block_out_state = ui::Checkbox::builder("  BLOCK OUT")
                .build_with_placer(&mut block_out, &mut rules_box_placer, app);
            if block_out_state.changed {
                self.custom_rules.top_out_rule.toggle(TopOutRule::BLOCK_OUT);
                if self.custom_rules.top_out_rule.is_empty() {
                    self.custom_rules.top_out_rule.toggle(TopOutRule::BLOCK_OUT);
                }
            }

            let mut lock_out = self.custom_rules.top_out_rule.contains(TopOutRule::LOCK_OUT);
            let lock_out_state = ui::Checkbox::builder("  LOCK OUT").
                build_with_placer(&mut lock_out, &mut rules_box_placer, app);
            if lock_out_state.changed {
                self.custom_rules.top_out_rule.toggle(TopOutRule::LOCK_OUT);
                if self.custom_rules.top_out_rule.is_empty() {
                    self.custom_rules.top_out_rule.toggle(TopOutRule::LOCK_OUT);
                }
            }

            let mut partial_lock_out = self.custom_rules.top_out_rule.contains(TopOutRule::PARTIAL_LOCK_OUT);
            let partial_lock_out_state = ui::Checkbox::builder("  PARTIAL LOCK OUT")
                .build_with_placer(&mut partial_lock_out, &mut rules_box_placer, app);
            if partial_lock_out_state.changed {
                self.custom_rules.top_out_rule.toggle(TopOutRule::PARTIAL_LOCK_OUT);
                if self.custom_rules.top_out_rule.is_empty() {
                    self.custom_rules.top_out_rule.toggle(TopOutRule::PARTIAL_LOCK_OUT);
                }
            }

            let mut garbage_out = self.custom_rules.top_out_rule.contains(TopOutRule::GARBAGE_OUT);
            let garbage_out_state = ui::Checkbox::builder("  GARBAGE OUT")
                .build_with_placer(&mut garbage_out, &mut rules_box_placer, app);
            if garbage_out_state.changed {
                self.custom_rules.top_out_rule.toggle(TopOutRule::GARBAGE_OUT);
                if self.custom_rules.top_out_rule.is_empty() {
                    self.custom_rules.top_out_rule.toggle(TopOutRule::GARBAGE_OUT);
                }
            }

            // @TODO ui for time values (in frames?)
            // @TODO slider float
            ui::SliderU64::builder("DAS", 0, 500_000)
                .build_with_placer(&mut self.custom_rules.das_repeat_delay, &mut rules_box_placer, app);
            ui::SliderU64::builder("ARR", 0, 500_000)
                .build_with_placer(&mut self.custom_rules.das_repeat_interval, &mut rules_box_placer, app);

            //pub gravity_curve: GravityCurve,
            //pub scoring_curve: ScoringRule,
            //pub level_curve: LevelCurve, // @Maybe rename to difficulty curve

            ui::SliderU64::builder("ENTRY DELAY", 0, 2_000_000)
                .build_with_placer(&mut self.custom_rules.entry_delay, &mut rules_box_placer, app);

            ui::SliderU8::builder("START LEVEL", self.custom_rules.minimum_level, 50)
                .build_with_placer(&mut self.custom_rules.minimum_level, &mut rules_box_placer, app);

            //pub lock_delay: LockDelayRule,

            //pub randomizer_type: RandomizerType,
            //app.input_u64_stretch("seed", &mut self.seed);
        }

        if ui::Button::new("START GAME", app).pressed {
            self.state = State::Custom;
        }

        if ui::Button::new("BACK", app).pressed {
            self.state = State::Main;
        }

        self.show_custom_rules_preview(app, _persistent);
    }

    fn show_custom_rules_preview(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        match self.focused_rule {
            Some(focused_rule) => {
                match focused_rule {
                    FocusedRule::RotationSystem => self.show_custom_rules_info_rotation_system(app, _persistent),
                    FocusedRule::HardDrop => self.show_custom_rules_info_hard_drop(app, _persistent),
                    //FocusedRule::SoftDrop => self.show_custom_rules_info_soft_drop(app, _persistent),
                    _ => self.show_custom_rules_info(app, _persistent),
                }
            }

            None => self.show_custom_rules_info(app, _persistent),
        }

        /*
        RotationSystem,

        HardDrop,
        HardDropLock,

        SoftDrop,
        SoftDropLock,

        LockDelayRule,

        Ghost,
        Hold,
        HoldResetRotation,

        InitialRotation,
        InitialHold,
        SpawnDrop,
        SpawnRow,
        NextPiecesPreview,

        LineClearRule,
        TopOutRule,

        DelayedAutoShift,
        AutoRepeatRate,

        SoftDropInterval,
        LineClearDelay,

        GravityCurve,
        ScoringCurve,
        LevelCurve,
        StartLevel,
        MinimumLevel,

        EntryDelay,

        RandomizerType,
        */
    }

    fn show_custom_rules_info(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        let menu_size = Vec2i { x: 580, y: 880 };

        // Ui
        let window_layout = ui::Layout {
            pos: Vec2i { x: 660, y: 40, },
            size: menu_size
        };
        ui::Ui::builder(window_layout).fixed_height().build(app);
        ui::Text::builder("RULE DESCRIPTION").build(app);
    }

    fn show_custom_rules_info_rotation_system(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        let menu_size = Vec2i { x: 580, y: 880 };

        // Ui
        let window_layout = ui::Layout {
            pos: Vec2i { x: 660, y: 40, },
            size: menu_size
        };
        ui::Ui::builder(window_layout).fixed_height().build(app);
        ui::Text::builder("ROTATION SYSTEM").build(app);
        ui::Text::builder("NINTENDO ROTATION SYSTEM RIGHTHAND").build(app);
    }

    fn show_custom_rules_info_hard_drop(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        let menu_size = Vec2i { x: 580, y: 880 };

        // Ui
        let window_layout = ui::Layout {
            pos: Vec2i { x: 660, y: 40, },
            size: menu_size
        };
        ui::Ui::builder(window_layout).fixed_height().build(app);
        ui::Text::builder("HARD DROP").build(app);

        let text =
            "A hard drop is a move in which a Tetromino drops \
            instantly drop directly below. It can't be moved or \
            rotated afterwards. It is a higher scoring move than a soft drop.";
        ui::Text::builder(text).multiline(true).build(app);

        /*
        // Render example playfield
        let mut batch = Batch::new();

        //let playfield = Playfield::new(Vec2i::new(), Vec2i { x: 6, y: 6 }, false);


        let texture = app.get_texture_or_create(
            "main_menu/custom/hard_drop/playfield",
            text_draw_size.x.ceil() as u32,
            text_draw_size.y.ceil() as u32,
            None
        );

        let framebuffer = app.get_framebuffer_or_create(
            "main_menu/custom/hard_drop/playfield",
            texture
        );
        framebuffer.clear(BufferClear::new().color(color::TRANSPARENT));

        app.render_batch(batch, Some(framebuffer));
        ui::Texture::new(texture, app);
        */
    }
}
