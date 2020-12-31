//#![windows_subsystem = "windows"]
extern crate sdl2;

mod app;
mod debug;
mod game;
mod imgui_sdl2;
mod linalg;
mod time;

use app::App;
use game::Game;

pub fn main() {
    let mut app = App::new();
    let mut game = Game::new(&app);
    app.run(&mut game);
}
