extern crate sdl2;
extern crate imgui_opengl_renderer;

use sdl2::event::Event;

use crate::time::Time;
use crate::render::Render;
use crate::debug::Debug;
use crate::game_state::GameState;
use crate::tasks::TaskSystem;

pub struct App<'a> {
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub timer_subsystem: sdl2::TimerSubsystem,

    // We need to hold this to not get freed
    #[allow(dead_code)]
    sdl_image_context: sdl2::image::Sdl2ImageContext,

    // @TODO video/window struct
    pub window: sdl2::video::Window,
    pub gl_context: sdl2::video::GLContext,

    pub time: Time,
    pub render: Render,
    pub debug: Debug,
    pub task_system: TaskSystem<'a>,

    pub event_pump: sdl2::EventPump,

    pub running: bool,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        // @TODO check results

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let timer_subsystem = sdl_context.timer().unwrap();

        use sdl2::image::InitFlag;
        let sdl_image_context = sdl2::image::init(InitFlag::PNG).unwrap();

        // OpenGL setup
        // @Refactor move to window struct

        let gl_attr = video_subsystem.gl_attr();

        // Don't use deprecated OpenGL functions
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

        gl_attr.set_context_flags().debug().set();
        gl_attr.set_context_version(3, 2);

        // @TODO test with these to be pixel perfect
        // Enable anti-aliasing
        gl_attr.set_multisample_buffers(1);
        gl_attr.set_multisample_samples(4);

        // @TODO use config info
        let window = video_subsystem.window("Codename Dash", 1280, 960)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        window.gl_make_current(&gl_context).unwrap();

        let time = Time::new(&timer_subsystem);
        let render = Render::new();
        let debug = Debug::new(&window);
        let task_system = TaskSystem::new();

        // @TODO input handler
        let event_pump = sdl_context.event_pump().unwrap();

        // @TODO video system
        // @XXX testing how to get some display info
        let video_driver = video_subsystem.current_video_driver();
        println!("Video driver: {}", video_driver);

        let num_video_displays = video_subsystem.num_video_displays().unwrap();
        println!("Video displays: {}", num_video_displays);

        for i in 0..num_video_displays {
            let display_mode = video_subsystem.desktop_display_mode(i).unwrap();
            let display_name = video_subsystem.display_name(i).unwrap();
            let display_dpi  = video_subsystem.display_dpi(i).unwrap();
            println!(
                "{}: {}x{} @ {} Hz dpi:({}, {}, {})",
                display_name,
                display_mode.w, display_mode.h, display_mode.refresh_rate,
                display_dpi.0, display_dpi.1, display_dpi.2
            );
        }

        Self {
            sdl_context,
            video_subsystem,
            timer_subsystem,
            sdl_image_context,
            window,
            gl_context,
            time,
            render,
            debug,
            task_system,
            event_pump,
            running: true,
        }
    }

    pub fn run<S: GameState>(&mut self) {
        let mut state = S::new(self);

        while self.running {

            self.time.new_frame(&self.timer_subsystem);
            self.task_system.run(&mut state, self.time.game_time);

            // @TODO input handler
            let events: Vec<Event> = self.event_pump.poll_iter().collect();
            for event in events {
                if self.debug.handle_event(&event) { continue; }
                if state.handle_input(self, &event) { continue; }
            }

            //update(state, self);
            state.update(self);

            // Render
            self.render.prepare_render();
            //render(state, self);
            state.render(self);
            self.window.gl_swap_window();
        }
    }
}
