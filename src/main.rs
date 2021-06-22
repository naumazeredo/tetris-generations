// Remove console on Windows if not in debug build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, allow(dead_code))]

//#![feature(option_expect_none)]

#[macro_use] extern crate bitflags;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate enum_dispatch;

// @Important maybe remove this dependency
extern crate rand_pcg;
extern crate rand_core;

#[macro_use] mod app;
mod linalg;
mod game;

use app::*;
use game::scenes::*;

fn main() {
    let config = AppConfig {
        window_name: "Tetris for all".to_string(),
        window_size: (1280, 960),
    };

    app::run::<State>(config);
}

#[derive(ImDraw)]
pub struct State {
    pub persistent: PersistentData,
    pub scene_manager: SceneManager,
}

const BLOCK_SCALE : f32 = 8.0;

impl GameState for State {
    fn new(app: &mut App<'_, Self>) -> Self {
        // persistent data
        let mut persistent = PersistentData::new(app);

        // scene
        let scene_manager = SceneManager::new(
            Scene::SinglePlayerScene(SinglePlayerScene::new(app, &mut persistent))
        );

        Self {
            persistent,
            scene_manager,
        }
    }

    fn update(&mut self, app: &mut App<'_, Self>) {
        app.update_input_mapping(&mut self.persistent.input_mapping);

        self.scene_manager.update(app, &mut self.persistent);
    }

    fn render(&mut self, app: &mut App<'_, Self>) {
        self.scene_manager.render(app, &mut self.persistent);
    }

    fn handle_input(&mut self, app: &mut App<'_, Self>, event: &sdl2::event::Event) -> bool {
        use sdl2::event::Event;
        use sdl2::keyboard::Scancode;

        if self.scene_manager.handle_input(app, &mut self.persistent, event) { return true; }

        match event {
            Event::KeyDown { scancode: Some(Scancode::F11), .. } => {
                use sdl2::video::FullscreenType;

                let window = &mut app.video_system.window;
                let new_fullscreen_state = match window.fullscreen_state() {
                    //FullscreenType::Off => FullscreenType::True,
                    //FullscreenType::True => FullscreenType::Desktop,
                    //FullscreenType::Desktop => FullscreenType::Off,

                    FullscreenType::Off => FullscreenType::Desktop,
                    _ => FullscreenType::Off,
                };

                window.set_fullscreen(new_fullscreen_state).unwrap();
            }

            // Restart all
            Event::KeyDown { scancode: Some(Scancode::R), .. } => {
                app.restart_time_system();

                self.scene_manager = SceneManager::new(
                    Scene::SinglePlayerScene(SinglePlayerScene::new(app, &mut self.persistent))
                );
            }

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
