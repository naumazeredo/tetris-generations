extern crate sdl2;

use super::engine::Engine;

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
