// Remove console on Windows if not in debug build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate bitflags;
extern crate sdl2;

mod app;
mod debug;
mod game;
mod imgui_sdl2;
mod linalg;
mod render;
mod time;

use app::App;
use game::Game;

pub fn main() {
    let mut app = App::new();
    let mut game = Game::new(&app);
    app.run(&mut game);
}
