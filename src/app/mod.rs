extern crate sdl2;
extern crate imgui_opengl_renderer;

pub mod animations;
pub mod debug;
pub mod entity;
pub mod game_state;
#[macro_use] pub mod imgui;
pub mod renderer;
pub mod sdl;
pub mod tasks;
pub mod time;
pub mod video;

pub use {
    animations::*,
    debug::*,
    entity::*,
    game_state::*,
    self::imgui::*,
    renderer::*,
    sdl::*,
    tasks::*,
    time::*,
    video::*,
};

use sdl2::event::Event;

pub struct App<'a, S> {
    pub animation_system: AnimationSystem,
    pub sdl_context: SdlContext,
    pub video: Video,
    pub time: Time,
    pub renderer: Renderer,
    pub debug: Debug,
    pub tasks: TaskSystem<'a, S>,

    pub event_pump: sdl2::EventPump,

    pub running: bool,
}

impl<'a, S: 'a + GameState> App<'a, S> {
    pub fn new() -> Self {
        // @TODO check results

        let sdl_context = SdlContext::new();
        let video= Video::new(&sdl_context);

        let animation_system = AnimationSystem::new();
        let time = Time::new(&sdl_context.timer_subsystem);
        let renderer = Renderer::new();
        let debug = Debug::new(&video.window);
        let tasks = TaskSystem::new();

        // @TODO input handler
        let event_pump = sdl_context.sdl.event_pump().unwrap();

        Self {
            animation_system,
            sdl_context,
            video,
            time,
            renderer,
            debug,
            tasks,
            event_pump,
            running: true,
        }
    }

    pub fn run(&mut self) {
        self.new_frame();
        let mut state = S::new(self);

        while self.running {
            self.new_frame();
            self.run_tasks(&mut state);

            // @TODO input handler
            let events: Vec<Event> = self.event_pump.poll_iter().collect();
            for event in events {
                if state.handle_input(self, &event) { continue; }
            }

            state.update(self);

            // Render
            self.renderer.prepare_render();
            state.render(self);
            self.video.present();
        }
    }
}
