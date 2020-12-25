extern crate sdl2;

use super::engine::Engine;

/*
enum State {
    Start,
    Run,
    GameOver,
}
*/

pub struct Game {
    pub running: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            running: true,
        }
    }
}

pub fn update(_engine: &mut Engine) {
}
