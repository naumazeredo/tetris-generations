//#![windows_subsystem = "windows"]
extern crate sdl2;

mod game;
mod engine;
mod time;
mod debug;
mod imgui_sdl2;

use engine::Engine;

pub fn main() {
    let mut engine = Engine::new();
    engine.run();
}
