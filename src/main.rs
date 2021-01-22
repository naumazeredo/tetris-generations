// Remove console on Windows if not in debug build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use] extern crate bitflags;
extern crate sdl2;
extern crate imgui;
extern crate imgui_opengl_renderer;

#[macro_use] mod app;
mod linalg;

use app::*;
use linalg::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    App::<State>::new().run();
}

#[derive(ImDraw)]
struct State {
    transform: Transform,
    sprite: Sprite,
    animator: Animator,
}

impl GameState for State {
    fn new(app: &mut App<'_, Self>) -> Self {
        let texture = load_texture("assets/gfx/template-anim-128x32-4frames.png");

        let mut build_frame = |x, y| {
            app.build_frame(
                Sprite {
                    texture,
                    texture_flip: TextureFlip::NO,
                    uvs: (Vec2i { x, y }, Vec2i { x: 32 + x, y: 32 + y }),
                    pivot: Vec2 { x: 16., y: 16. },
                    size: Vec2 { x: 32., y: 32. },
                },
                1_000_000,
            )
        };

        let frame_0 = build_frame(0, 0);
        let frame_1 = build_frame(32, 0);
        let frame_2 = build_frame(64, 0);
        let frame_3 = build_frame(96, 0);

        let animation_0 = app.build_animation(vec![frame_0, frame_2], Repetitions::Infinite);
        let animation_1 = app.build_animation(vec![frame_0, frame_1, frame_2, frame_3],
                                              Repetitions::Finite(5));

        let animation_set = app.build_animation_set(vec![animation_0, animation_1]);

        let transform = Transform {
            pos: Vec2 { x: 100., y: 400. },
            rot: 0.,
            layer: 0,
        };

        let animator = app.build_animator(animation_set, transform);

        animator.play(app);

        Self {
            transform,
            sprite: Sprite {
                texture: load_texture("assets/gfx/default.png"),
                texture_flip: TextureFlip::NO,
                uvs: (Vec2i { x: 0, y: 0 }, Vec2i { x: 32, y: 32 }),
                pivot: Vec2 { x: 0., y: 0. },
                size: Vec2 { x: 32., y: 32. },
            },
            animator: animator,
        }
    }

    fn update(&mut self, _app: &mut App<'_, Self>) {
    }

    fn render(&mut self, app: &mut App<'_, Self>) {
        /*
        app.renderer.queue_draw_sprite(
            0 as Program,
            self.c,
            &self.transform,
            &self.sprite,
        );
        */

        app.render_animators();

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
                    state.transform.pos.x = 10.;
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
