extern crate sdl2;
extern crate imgui_opengl_renderer;

pub mod animation_system;
pub mod asset_system;
pub mod debug;
pub mod game_state;
pub mod id_manager;
#[macro_use] pub mod imgui_wrapper;
pub mod input;
pub mod renderer;
pub mod sdl;
pub mod task_system;
pub mod transform;
pub mod time_system;
pub mod utils;
pub mod video_system;

pub use {
    animation_system::*,
    game_state::*,
    id_manager::*,
    input::*,
    imgui_wrapper::*,
    renderer::*,
    task_system::*,
    transform::*,
    utils::*,
    video_system::*,
};

use sdl2::event::Event;
use sdl2::keyboard::Scancode;

use asset_system::*;
use debug::*;
use sdl::*;
use time_system::*;

pub struct App<'a, S> {
    asset_system: AssetSystem,
    animation_system: AnimationSystem,
    debug: Debug,
    input_system: InputSystem,
    renderer: Renderer,
    sdl_context: SdlContext,
    task_system: TaskSystem<'a, S>,
    time_system: TimeSystem,

    running: bool,

    // @Maybe refactor? Giving public access to be able to mess with window freely
    pub video_system: VideoSystem,
}

impl<'a, S: GameState> App<'a, S> {
    pub fn new(config: AppConfig) -> Self {
        // @TODO check results

        let sdl_context = SdlContext::new();
        let video_system = VideoSystem::new(config, sdl_context.video_subsystem.clone());

        let input_system = InputSystem::new(sdl_context.controller_subsystem.clone());
        let time_system = TimeSystem::new(sdl_context.timer_subsystem.clone());
        let renderer = Renderer::new();

        let animation_system = AnimationSystem::new();
        let debug = Debug::new(&video_system.window);
        let task_system = TaskSystem::new();

        Self {
            asset_system: AssetSystem::new(),
            animation_system,

            sdl_context,
            video_system,

            input_system,
            time_system,
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
                // Update input system
                // This needs to be done before state since it needs to be consistent.
                // We might remove the asserts from the input system and make it handle the
                // inconsistencies
                let timestamp = self.time_system.game_time;
                self.input_system.handle_input(&event, timestamp);

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
            self.video_system.swap_buffers();
        }
    }

    pub fn exit(&mut self) {
        self.running = false;
    }

    fn handle_input(&mut self, event: &Event) {
        match event {
            Event::Quit {..}
            | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => {
                self.running = false;
                return;
            },
            _ => {}
        }
    }
}

pub struct AppConfig {
    pub window_name: String,
    pub window_size: (u32, u32),
}
