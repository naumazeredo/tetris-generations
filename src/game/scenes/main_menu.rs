use crate::app::*;
use crate::linalg::Vec2i;

use super::*;

#[derive(Clone, Debug, ImDraw)]
pub struct MainMenuScene {
    start_singleplayer_game: bool,
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
        _persistent: &mut PersistentData
    ) {
        let window_size = app.window_size();
        let window_size = Vec2i { x: window_size.0 as i32, y: window_size.1 as i32 };
        let menu_size = Vec2i { x: 300, y: 300 };

        // Ui
        let window_layout = Layout {
            pos: Vec2i {
                x: (window_size.x - menu_size.x) / 2,
                y: (window_size.y - menu_size.y) / 2
            },
            size: menu_size
        };
        app.new_ui(window_layout);

        app.text("MENU");

        if app.button("NEW GAME") {
            self.start_singleplayer_game = true;
        }

        if app.button("QUIT") {
            println!("quit");
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
            Some(SceneTransition::Push(SinglePlayerScene::new(seed, app, persistent).into()))
        } else {
            None
        }
    }
}

impl MainMenuScene {
    pub fn new() -> Self {
        Self {
            start_singleplayer_game: false,
        }
    }
}
