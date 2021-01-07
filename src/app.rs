extern crate sdl2;
extern crate imgui_opengl_renderer;

use super::time::Time;
use super::render::Render;
use super::debug::Debug;

pub struct App {
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub timer_subsystem: sdl2::TimerSubsystem,

    // TODO video/window struct
    pub window: sdl2::video::Window,
    pub gl_context: sdl2::video::GLContext,

    pub time: Time,
    pub render: Render,
    pub debug: Debug,

    pub event_pump: sdl2::EventPump,

    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let timer_subsystem = sdl_context.timer().unwrap();

        // OpenGL setup

        let gl_attr = video_subsystem.gl_attr();

        // Don't use deprecated OpenGL functions
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

        gl_attr.set_context_flags().debug().set();
        gl_attr.set_context_version(3, 2);

        // TODO test with these to be pixel perfect
        // Enable anti-aliasing
        gl_attr.set_multisample_buffers(1);
        gl_attr.set_multisample_samples(4);

        // TODO use config info
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

        // TODO input handler
        let event_pump = sdl_context.event_pump().unwrap();

        // TODO video system
        // XXX testing how to get some display info
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
            window,
            gl_context,
            time,
            render,
            debug,
            event_pump,
            running: true,
        }
    }

    pub fn run<S, U: Fn(&mut S, &mut App), R: Fn(&mut S, &mut App)>(
        &mut self,
        state: &mut S,
        update: U,
        render: R,
        //handle_event: Fn(&mut S, &mut App, sdl2::event::Event)
    ) {
        while self.running {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;

            self.time.new_frame(&self.timer_subsystem);

            // TODO input handler
            for event in self.event_pump.poll_iter() {
                if self.debug.handle_event(&event) { continue; }

                match event {
                    Event::Quit {..} |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            self.running = false;
                        },
                        Event::KeyDown { keycode: Some(Keycode::F11), .. } => {
                            use sdl2::video::FullscreenType;

                            let new_fullscreen_state = match self.window.fullscreen_state() {
                                //FullscreenType::Off => FullscreenType::True,
                                //FullscreenType::True => FullscreenType::Desktop,
                                //FullscreenType::Desktop => FullscreenType::Off,

                                FullscreenType::Off => FullscreenType::Desktop,
                                _ => FullscreenType::Off,
                            };

                            self.window.set_fullscreen(new_fullscreen_state).unwrap();
                        },
                    _ => {}
                }
            }

            update(state, self);

            // Render
            self.render.prepare_render();
            render(state, self);
            self.window.gl_swap_window();
        }
    }
}
