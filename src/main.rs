// Remove console on Windows if not in debug build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate bitflags;
extern crate sdl2;
extern crate imgui;
extern crate imgui_opengl_renderer;

mod app;
mod linalg;
mod tasks;

use app::{
    App,
    imgui::*,
    renderer::*,
    game_state::GameState,
};

use linalg::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    App::<State>::new().run();
}

#[derive(ImDraw)]
struct State {
    x: f32, y: f32,
    r: f32,
    px: f32, py: f32,
    w: f32, h: f32,
    c: Color,
    l: i32,
    texture: Texture,
    texture_flip: TextureFlip,
    test_imdraw: Vec2,
}

impl GameState for State {
    fn new(_app: &mut App<'_, Self>) -> Self {
        Self {
            x: 100., y: 100.,
            r: 0.,
            px: 0., py: 0.,
            w: 32., h: 32.,
            c: color::WHITE,
            l: 0,
            texture: load_texture("assets/gfx/default.png"),
            texture_flip: TextureFlip::NO,
            test_imdraw: Vec2::new(),
        }
    }

    fn update(&mut self, _app: &mut App<'_, Self>) {
    }

    fn render(&mut self, app: &mut App<'_, Self>) {
        app.renderer.queue_draw_sprite(
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

        app.renderer.render_queued_draws();

        // @Refactor maybe this debug info really should be managed by the App. This way
        //           we don't have to explicitly call render_queued, which seems way cleaner.
        //           Maybe not, since we can add framebuffers and have more control of rendering here.
        app.render_debug(self, |ui, state| {
            state.imdraw("State", ui);
        });
    }

    fn handle_input(&mut self, app: &mut App<'_, Self>, event: &Event) -> bool {
        if app.handle_debug_event(&event) { return true; }

        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                app.running = false;
            },
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                app.schedule_task(1_000_000, |id, state, app| {
                    println!("task {} {}", id, app.time.game_time);
                    state.x = 10.;
                });
            },
            Event::KeyDown { keycode: Some(Keycode::F11), .. } => {
                use sdl2::video::FullscreenType;

                let window = &mut app.video.window;
                let new_fullscreen_state = match window.fullscreen_state() {
                    //FullscreenType::Off => FullscreenType::True,
                    //FullscreenType::True => FullscreenType::Desktop,
                    //FullscreenType::Desktop => FullscreenType::Off,

                    FullscreenType::Off => FullscreenType::Desktop,
                    _ => FullscreenType::Off,
                };

                window.set_fullscreen(new_fullscreen_state).unwrap();
            },
            _ => {}
        }

        false
    }
}
