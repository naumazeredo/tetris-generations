// Remove console on Windows if not in debug build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//#![cfg_attr(not(debug_assertions), deny(dead_code))]
#![cfg_attr(debug_assertions, allow(dead_code))]
#![cfg_attr(debug_assertions, allow(incomplete_features))]

#![feature(int_log)]
#![feature(generic_const_exprs)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]

#[macro_use] mod app;
mod linalg;
mod game;

use app::*;
use game::scenes::*;

fn main() {
    let config = AppConfig {
        window_name: "Tetris for all".to_string(),
        window_size: (1280, 960),
        window_position: None,
        window_resizable: false,
    };

    app::run::<State>(config);
}

#[derive(ImDraw)]
pub struct State {
    pub persistent: PersistentData,
    pub scene_manager: SceneManager<Scene>,

    seed: u64,
}

const BLOCK_SCALE : u32 = 8;

impl GameState for State {
    fn new(app: &mut App) -> Self {
        // persistent data
        let persistent = PersistentData::new(app);

        // seed
        let seed = app.system_time();

        // scene
        let scene_manager = SceneManager::new(
            MainMenuScene::new(app).into()
        );

        Self {
            persistent,
            scene_manager,
            seed,
        }
    }

    fn update(&mut self, app: &mut App) {
        self.persistent.input_mapping.update(app);
        self.scene_manager.update(app, &mut self.persistent);
    }

    fn render(&mut self, app: &mut App) {
        self.scene_manager.render(app, &mut self.persistent);
    }

    fn handle_input(&mut self, event: &sdl2::event::Event, app: &mut App) -> bool {
        use sdl2::event::Event;
        use sdl2::keyboard::Scancode;

        if self.scene_manager.handle_input(event, app, &mut self.persistent) { return true; }

        match event {
            Event::KeyDown { scancode: Some(Scancode::Q), .. } => {
                if self.persistent.pixel_scale > 1 {
                    self.persistent.pixel_scale -= 1;
                }
            }

            Event::KeyDown { scancode: Some(Scancode::E), .. } => {
                if self.persistent.pixel_scale < 255 {
                    self.persistent.pixel_scale += 1;
                }
            }

            _ => {}
        }

        false
    }
}
