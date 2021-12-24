use super::*;

impl MainMenuScene {
    pub fn show_classic(
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

    pub fn show_classic_online(
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
}
