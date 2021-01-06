extern crate sdl2;
extern crate imgui;

use super::app::App;
use super::time::Time;
use super::debug::Debug;
use super::render::*;
use super::linalg::*;

pub struct Game {
    pub time: Time,
    pub debug: Debug,
    pub state: GameState,

    pub running: bool,
}

impl Game {
    pub fn new(app: &App) -> Self {
        let time = Time::new(app);
        let debug = Debug::new(&app.window);

        let state = GameState::new();

        Self {
            time,
            debug,
            state,
            running: true,
        }
    }

    pub fn setup(&mut self, _app: &mut App) {
    }

    pub fn update(&mut self, app: &mut App) {
        self.time.new_frame(app);
    }

    pub fn render(&mut self, app: &mut App) {
        draw_sprite(
            &mut app.render,
            //app.render.current_program as Program,
            0 as Program,
            self.state.l,
            self.state.c,
            Vec2 { x: self.state.x, y: self.state.y }, Vec2 { x: self.state.w, y: self.state.h },
            self.state.r,
            Vec2 { x: self.state.px, y: self.state.py },
            Texture::new(),
            TextureFlip::NO,
            (Vec2i::new(), Vec2i::new())
        );

        self.debug.render(app, &mut self.state);
    }

    pub fn handle_input(&mut self, _event: sdl2::event::Event) {
    }
}


pub struct GameState {
    pub x: f32,
    pub y: f32,

    pub r: f32,

    pub px: f32,
    pub py: f32,

    pub w: f32,
    pub h: f32,

    pub c: Color,

    pub l: i32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            x: 100.,
            y: 100.,
            r: 0.,
            px: 0.,
            py: 0.,
            w: 32.,
            h: 32.,
            c: WHITE,
            l: 0,
        }
    }
}
