use crate::app::*;
use crate::linalg::Vec2i;

use super::*;

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
}

#[derive(Debug, ImDraw)]
pub struct MainMenuScene {
    state: State,
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

            State::Options      => self.show_options(app, persistent),
            State::OptionsVideo => self.show_options_video(app, persistent),
            State::OptionsAudio => self.show_options_audio(app, persistent),

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
    pub fn new() -> Self {
        Self {
            state: State::Main,
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

        app.text("CLASSIC");

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

        app.text("MODERN");

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

        app.text("OPTIONS");

        if ui::Button::new("VIDEO", app).pressed {
            self.state = State::OptionsVideo;
        }

        if ui::Button::new("AUDIO", app).pressed {
            self.state = State::OptionsAudio;
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

        app.text("OPTIONS - VIDEO");

        // @TODO cache this
        let display_modes_str = app.display_modes()
            .into_iter()
            .map(|mode| format!("{}x{} {} Hz", mode.w, mode.h, mode.refresh_rate))
            .collect::<Vec<String>>();

        let mut index = 0;
        Combobox::new("RESOLUTION", &display_modes_str, &mut index, app);

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

        app.text("OPTIONS - AUDIO");

        // @TODO Volume mixer

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

        app.text("CLASSIC ONLINE");

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

        app.text("MODERN ONLINE");

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
