// Remove console on Windows if not in debug build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, allow(dead_code))]

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
    entity_container: EntityContainer,
    entity: Entity,
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

        // Entities

        let entity_container = EntityContainer::new();
        let entity = entity_container.create_entity(
            Transform {
                pos: Vec2 { x: 100., y: 400. },
                rot: 0.,
                layer: 0,
            },
            Sprite {
                texture,
                texture_flip: TextureFlip::NO,
                uvs: (Vec2i { x: 0, y: 0 }, Vec2i { x: 32, y: 32 }),
                pivot: Vec2 { x: 16., y: 16. },
                size: Vec2 { x: 32., y: 32. },
            },
        );


        fn test(_id: u64, state: &mut State, app: &mut App<State>) {
            println!("------------");
            println!("all:      {}", state.entity_container.all().transforms().len());
            println!("active:   {}", state.entity_container.active().transforms().len());
            println!("inactive: {}", state.entity_container.inactive().transforms().len());
            println!("visible:  {}", state.entity_container.visible().transforms().len());
            println!("hidden:   {}", state.entity_container.hidden().transforms().len());

            app.schedule_task(3_000_000, test);
        }

        app.schedule_task(3_000_000, test);

        Self {
            entity_container,
            entity,
        }
    }

    fn update(&mut self, app: &mut App<'_, Self>) {
        let mut entities = self.entity_container.active_mut();
        let transforms = entities.transforms_mut();
        for transform in transforms.iter_mut() {
            transform.pos.x += 10. * app.time.frame_duration();
        }
    }

    fn render(&mut self, app: &mut App<'_, Self>) {
        self.entity_container.render(&mut app.renderer);

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
                app.schedule_task(1_000_000, |id, _state, app| {
                    println!("task {} {}", id, app.time.game_time);
                });
            },
            Event::KeyDown { keycode: Some(Keycode::K), .. } => {
                self.entity.destroy();
            },
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                println!("active: {}", self.entity.is_active());
                self.entity.set_active(true);
            },
            Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                self.entity.set_visible(true);
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
