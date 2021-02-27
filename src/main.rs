// Remove console on Windows if not in debug build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, allow(dead_code))]

#![feature(option_expect_none)]

#[macro_use] extern crate bitflags;
extern crate imgui;
extern crate imgui_opengl_renderer;

#[macro_use] mod app;
mod entities;
mod linalg;

use app::*;
use entities::*;
use linalg::*;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;

fn main() {
    App::<State>::new().run();
}

#[derive(ImDraw)]
pub struct State {
    pub entity_containers: EntityContainers,
    pub entity_id: MyEntityId,
    pub animated_entity_id: MyEntityId,

    pub input_mapping: InputMapping,
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

        let mut entity_containers = EntityContainers::new();

        let entity_id = entity_containers.create::<MyEntity>(
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

        //let animated_entity_id = entity_containers.create::<AnimatedEntity>(
        let animated_entity_id = entity_containers.create_animated::<MyEntity>(
            Transform {
                pos: Vec2 { x: 100., y: 500. },
                rot: 0.,
                layer: 0,
            },
            animation_set
        );

        let animated_entity = entity_containers.get_mut(animated_entity_id).unwrap();
        //animated_entity.change_animation_set(animation_set, app);
        animated_entity.play_animation(app);

        // input
        let mut input_mapping = InputMapping::new();

        let mut button = Button::new();
        button.add_key(Scancode::A);
        button.add_key(Scancode::Z);

        input_mapping.add_button_mapping("FIRE".to_string(), button);

        Self {
            entity_containers,
            entity_id,
            animated_entity_id,
            input_mapping,
        }
    }

    fn update(&mut self, app: &mut App<'_, Self>) {
        app.update_input_mapping(&mut self.input_mapping);

        let button = self.input_mapping.button("FIRE".to_string());
        let pressed = button.pressed();
        let released = button.released();
        let long_press = button.pressed_for(5_000_000, app.time.game_time);

        if pressed { println!("pressed!"); }
        if released { println!("released"); }
        if long_press { println!("FIRE!"); }

        if let Some(my_entity) = self.entity_containers.get_mut(self.entity_id) {
            my_entity.entity_mut().transform.pos.x += 10.0 * app.time.frame_duration();
        }
    }

    fn render(&mut self, app: &mut App<'_, Self>) {
        self.entity_containers.render(&mut app.renderer);

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
            Event::KeyDown { scancode: Some(Scancode::Escape), .. } => {
                app.running = false;
            },
            Event::KeyDown { scancode: Some(Scancode::Num1), .. } => {
                app.schedule_task(1_000_000, |id, _state, app| {
                    println!("task {} {}", id, app.time.game_time);
                });
            },
            Event::KeyDown { scancode: Some(Scancode::K), .. } => {
                self.entity_containers.destroy(self.entity_id);
            },
            Event::KeyDown { scancode: Some(Scancode::A), .. } => {
                /*
                println!("active: {:?}", self.my_entity_container.is_active(self.entity));
                self.my_entity_container.set_active(self.entity, true);
                println!("active: {:?}", self.my_entity_container.is_active(self.entity));
                */
            },
            Event::KeyDown { scancode: Some(Scancode::V), .. } => {
                /*
                println!("visible: {:?}", self.my_entity_container.is_visible(self.entity));
                self.my_entity_container.set_visible(self.entity, true);
                println!("visible: {:?}", self.my_entity_container.is_visible(self.entity));
                */
            },
            Event::KeyDown { scancode: Some(Scancode::F11), .. } => {
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
