use crate::app::*;
use crate::linalg::Vec2i;

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
}

impl VideoInfo {
    fn load(app: &App) -> Self {
        Self {
            screen_mode: app.window_screen_mode(),
            display_index: app.window_display_index(),
            display_mode: app.window_display_mode(),
            window_size: app.window_size(),
        }
    }
}

impl_imdraw_todo!(VideoInfo);


#[derive(Debug, ImDraw)]
pub struct MainMenuScene {
    state: State,
    video_info: VideoInfo,
}

impl SceneTrait for MainMenuScene {
    fn update(
        &mut self,
        _app: &mut App,
        _persistent: &mut PersistentData
    )
    {}

    fn render(
        &mut self,
        app: &mut App,
        persistent: &mut PersistentData
    ) {
        match self.state {
            State::Main => self.show_main(app, persistent),

            State::Classic       => self.show_classic(app, persistent),
            State::ClassicOnline => self.show_classic_online(app, persistent),

            State::Modern       => self.show_modern(app, persistent),
            State::ModernOnline => self.show_modern_online(app, persistent),

            //State::Custom  => {}

            State::Options => self.show_options(app, persistent),
            State::OptionsVideo => self.show_options_video(app, persistent),
            State::OptionsAudio => self.show_options_audio(app, persistent),
            State::OptionsControls => self.show_options_controls(app, persistent),

            // @Remove
            _ => self.state = State::Main,
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

    fn transition(
        &mut self,
        app: &mut App,
        persistent: &mut PersistentData
    ) -> Option<SceneTransition> {
        match self.state {
            State::ModernOnlineSolo =>
                Some(SceneTransition::Push(MultiPlayerScene::new(app, persistent).into())),

            State::ModernOnlineSpectate =>
                Some(SceneTransition::Push(MultiPlayerSpectateScene::new(app, persistent).into())),

            _ => None
        }
    }
}

impl MainMenuScene {
    pub fn new(app: &mut App) -> Self {
        Self {
            state: State::Main,
            video_info: VideoInfo::load(app),
        }
    }

    fn show_main(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        let window_size = app.window_size();
        let window_size = Vec2i { x: window_size.0 as i32, y: window_size.1 as i32 };
        let menu_size = Vec2i { x: 600, y: 300 };

        // Ui
        let window_layout = Layout {
            pos: Vec2i {
                x: 40,
                y: (window_size.y - menu_size.y) / 2
            },
            size: menu_size
        };
        app.new_ui(window_layout);

        if ui::Button::new("CLASSIC", app).pressed {
            self.state = State::Classic;
        }

        if ui::Button::new("MODERN", app).pressed {
            self.state = State::Modern;
        }

        if ui::Button::new("CUSTOM", app).pressed {
            self.state = State::Custom;
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
        let menu_size = Vec2i { x: 600, y: 300 };

        // Ui
        let window_layout = Layout {
            pos: Vec2i {
                x: 40,
                y: (window_size.y - menu_size.y) / 2
            },
            size: menu_size
        };
        app.new_ui(window_layout);

        Text::new("CLASSIC", app);

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
        let menu_size = Vec2i { x: 600, y: 300 };

        // Ui
        let window_layout = Layout {
            pos: Vec2i {
                x: 40,
                y: (window_size.y - menu_size.y) / 2
            },
            size: menu_size
        };
        app.new_ui(window_layout);

        Text::new("MODERN", app);

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
        let menu_size = Vec2i { x: 600, y: 300 };

        // Ui
        let window_layout = Layout {
            pos: Vec2i {
                x: 40,
                y: (window_size.y - menu_size.y) / 2
            },
            size: menu_size
        };
        app.new_ui(window_layout);

        Text::new("OPTIONS", app);

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
        let menu_size = Vec2i { x: 600, y: 300 };

        // Ui
        let window_layout = Layout {
            pos: Vec2i {
                x: 40,
                y: (window_size.y - menu_size.y) / 2
            },
            size: menu_size
        };
        app.new_ui(window_layout);

        Text::new("OPTIONS - VIDEO", app);

        // Fullscreen options
        let mut screen_mode_index = match self.video_info.screen_mode {
            FullscreenType::Off     => 0,
            FullscreenType::True    => 1,
            FullscreenType::Desktop => 2,
        };

        let screen_mode_combobox =
            Combobox::builder("SCREEN MODE", &SCREEN_MODES).build(&mut screen_mode_index, app);

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
        let display_state = Combobox::builder("DISPLAY", &DISPLAY_NAMES[..num_displays])
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

        let window_size_state = Combobox::builder("RESOLUTION", &window_sizes_strs)
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

        let refresh_rates_state = Combobox::builder("REFRESH RATE", &refresh_rates_strs)
            .disabled(!is_fullscreen)
            .build(&mut rate_index, app);

        if refresh_rates_state.changed && is_fullscreen {
            let rate = (window_sizes_and_rates[size_index].1)[rate_index];
            self.video_info.display_mode.refresh_rate = rate as i32;
            app.set_window_display_mode(self.video_info.display_mode);
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
        let menu_size = Vec2i { x: 600, y: 300 };

        // Ui
        let window_layout = Layout {
            pos: Vec2i {
                x: 40,
                y: (window_size.y - menu_size.y) / 2
            },
            size: menu_size
        };
        app.new_ui(window_layout);

        Text::new("OPTIONS - AUDIO", app);

        // Music volume
        let mut music_volume = app.music_volume();
        let music_volume_slider =
            SliderI32::builder("MUSIC", 0, app.max_volume())
                .build(&mut music_volume, app);

        if music_volume_slider.changed {
            app.set_music_volume(music_volume);
        }

        // SFX volume
        let mut sfx_volume = app.sfx_volume();
        let sfx_volume_slider =
            SliderI32::builder("SFX", 0, app.max_volume())
                .build(&mut sfx_volume, app);

        if sfx_volume_slider.changed {
            app.set_sfx_volume(sfx_volume);
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
        let menu_size = Vec2i { x: 600, y: 300 };

        // Ui
        let window_layout = Layout {
            pos: Vec2i {
                x: 40,
                y: (window_size.y - menu_size.y) / 2
            },
            size: menu_size
        };
        app.new_ui(window_layout);

        Text::new("OPTIONS - CONTROLS", app);

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
        let menu_size = Vec2i { x: 600, y: 300 };

        // Ui
        let window_layout = Layout {
            pos: Vec2i {
                x: 40,
                y: (window_size.y - menu_size.y) / 2
            },
            size: menu_size
        };
        app.new_ui(window_layout);

        Text::new("CLASSIC ONLINE", app);

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
            self.state = State::Main;
        }
    }

    fn show_modern_online(
        &mut self,
        app: &mut App,
        _persistent: &mut PersistentData
    ) {
        let window_size = app.window_size();
        let window_size = Vec2i { x: window_size.0 as i32, y: window_size.1 as i32 };
        let menu_size = Vec2i { x: 600, y: 300 };

        // Ui
        let window_layout = Layout {
            pos: Vec2i {
                x: 40,
                y: (window_size.y - menu_size.y) / 2
            },
            size: menu_size
        };
        app.new_ui(window_layout);

        Text::new("MODERN ONLINE", app);

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
            self.state = State::Main;
        }
    }
}
