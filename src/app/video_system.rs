use super::AppConfig;

// @Maybe refactor? Giving public access to be able to mess with window freely
pub struct VideoSystem {
    pub window: sdl2::video::Window,
    pub(in crate::app) gl_context: sdl2::video::GLContext,
}

impl VideoSystem {
    pub(in crate::app) fn new(config: AppConfig, video_subsystem: sdl2::VideoSubsystem) -> Self {
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
        let window =
            video_subsystem.window(
                &config.window_name,
                config.window_size.0,
                config.window_size.1
            )
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        window.gl_make_current(&gl_context).unwrap();

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
            window,
            gl_context,
        }
    }

    pub(in crate::app) fn swap_buffers(&self) {
        self.window.gl_swap_window();
    }
}
