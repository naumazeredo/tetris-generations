// Remove console on Windows if not in debug build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate bitflags;
extern crate sdl2;
extern crate imgui;

mod app;
mod debug;
mod game_state;
#[macro_use] mod imdraw;
mod imgui_sdl2;
mod linalg;
mod render;
mod tasks;
mod time;

use app::App;
use render::*;
use linalg::*;
use imgui::*;
use imdraw::ImDraw;
use game_state::GameState;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    App::new().run::<State>();
}

#[derive(ImDraw)]
struct State {
    x: f32,
    y: f32,

    r: f32,

    px: f32,
    py: f32,

    w: f32,
    h: f32,

    c: Color,

    l: i32,

    texture: Texture,
    texture_flip: TextureFlip,

    test_imdraw: Vec2,
}

impl GameState for State {
    fn new(_app: &mut App) -> Self {
        Self {
            x: 100.,
            y: 100.,
            r: 0.,
            px: 0.,
            py: 0.,
            w: 32.,
            h: 32.,
            c: render::WHITE,
            l: 0,
            texture: load_texture("assets/gfx/default.png"),
            texture_flip: TextureFlip::NO,
            test_imdraw: Vec2::new(),
        }
    }

    fn update(&mut self, _app: &mut App) {
    }

    fn render(&mut self, app: &mut App) {
        app.render.queue_draw_sprite(
            0 as Program,
            self.l,
            self.c,
            Vec2 { x: self.x, y: self.y }, Vec2 { x: self.w, y: self.h },
            self.r,
            Vec2 { x: self.px, y: self.py },
            self.texture,
            self.texture_flip,
            (Vec2i { x: 0, y: 0 }, Vec2i { x: 32, y: 32 })
        );

        // @Refactor maybe this debug info really should be managed by the App. This way
        //           we don't have to explicitly call render_queued, which seems way cleaner.
        //           Maybe not since we can add framebuffers and have more control of rendering here.

        app.render.render_queued_draws();

        // @TODO remove debug from App? Will I need to pass it in the callback functions also?
        //       Ideally we could just add it to the State, but we can't pass the state to its
        //       method if we do this :/
        app.debug.render(&app.window, &app.event_pump, self, |ui, state| {
            state.imdraw("State", ui);
        });
    }

    fn handle_input(&mut self, app: &mut App, event: &Event) -> bool {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                app.running = false;
            },
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                app.task_system.schedule(app.time.game_time + 1_000_000, |id, current_time| {
                    println!("task {} {}", id, current_time);
                });
            },
            Event::KeyDown { keycode: Some(Keycode::F11), .. } => {
                use sdl2::video::FullscreenType;

                let new_fullscreen_state = match app.window.fullscreen_state() {
                    //FullscreenType::Off => FullscreenType::True,
                    //FullscreenType::True => FullscreenType::Desktop,
                    //FullscreenType::Desktop => FullscreenType::Off,

                    FullscreenType::Off => FullscreenType::Desktop,
                    _ => FullscreenType::Off,
                };

                app.window.set_fullscreen(new_fullscreen_state).unwrap();
            },
            _ => {}
        }

        false
    }
}
