extern crate sdl2;

use super::app::App;
use super::time::Time;
use super::debug::Debug;

pub struct Game {
    pub time: Time,
    pub debug: Debug,

    pub running: bool,
}

impl Game {
    pub fn new(app: &App) -> Self {
        let time = Time::new(app);
        let debug = Debug::new(&app.window);

        Self {
            time,
            debug,
            running: true,
        }
    }

    pub fn setup(&mut self, _app: &mut App) {
    }

    pub fn update(&mut self, app: &mut App) {
        self.time.new_frame(app);
    }

    pub fn render(&mut self, app: &App) {
        self.debug.render(app);
    }

    pub fn handle_input(&mut self, _event: sdl2::event::Event) {
    }
}

