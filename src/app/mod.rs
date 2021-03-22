extern crate sdl2;
extern crate imgui_opengl_renderer;

pub mod animations;
pub mod debug;
pub mod game_state;
pub mod id_manager;
#[macro_use] pub mod imgui;
pub mod input;
pub mod renderer;
pub mod sdl;
pub mod task_system;
pub mod transform;
pub mod time;
pub mod utils;
pub mod video;

pub use {
    animations::*,
    debug::*,
    game_state::*,
    id_manager::*,
    input::*,
    self::imgui::*,
    renderer::*,
    sdl::*,
    task_system::*,
    transform::*,
    time::*,
    video::*,
    utils::*,
};

use sdl2::event::Event;
use sdl2::keyboard::Scancode;

pub struct App<'a, S> {
    pub animation_system: AnimationSystem,
    pub sdl_context: SdlContext,
    pub video: Video,
    pub time: Time,
    pub input: InputSystem,
    pub renderer: Renderer,
    pub debug: Debug,
    pub task_system: TaskSystem<'a, S>,

    pub running: bool,
}

impl<'a, S: 'a + GameState> App<'a, S> {
    pub fn new() -> Self {
        // @TODO check results

        let sdl_context = SdlContext::new();
        let video= Video::new(&sdl_context);

        let input = InputSystem::new(sdl_context.controller_subsystem.clone());
        let time = Time::new(&sdl_context.timer_subsystem); // @TODO clone subsystem instead of using a reference
        let renderer = Renderer::new();

        let animation_system = AnimationSystem::new();
        let debug = Debug::new(&video.window);
        let task_system = TaskSystem::new();

        Self {
            animation_system,

            sdl_context,
            video,

            input,
            time,
            renderer,

            debug,
            task_system,
            running: true,
        }
    }

    pub fn run(&mut self) {
        self.new_frame();
        let mut state = S::new(self);

        while self.running {
            self.new_frame();
            self.run_tasks(&mut state);

            let events: Vec<Event> = self.sdl_context.event_pump.poll_iter().collect();
            for event in events.into_iter() {
                // Handle game input first to allow it consuming the input
                // This can be useful if the game has some meta components, like
                // not allowing you to close the window, or changing how it handles
                // window focus/minimize/maximize, etc
                if state.handle_input(self, &event) { continue; }

                self.handle_input(&event);
            }

            state.update(self);

            // Render
            self.renderer.prepare_render();
            state.render(self);
            self.video.present();
        }
    }

    fn handle_input(&mut self, event: &Event) {
        let timestamp = self.time.game_time;

        match event {
            Event::Quit {..}
            | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => {
                self.running = false;
                return;
            },
            _ => {}
        }

        self.input.handle_input(&event, timestamp);
    }
}
