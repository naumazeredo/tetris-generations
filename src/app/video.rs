use super::sdl::SdlContext;

pub struct Video {
    pub window: sdl2::video::Window,
    pub gl_context: sdl2::video::GLContext,
}

impl Video {
    pub fn new(sdl_context: &SdlContext) -> Self {
        // OpenGL setup
        // @Refactor move to window struct

        let gl_attr = sdl_context.video_subsystem.gl_attr();

        // Don't use deprecated OpenGL functions
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

        gl_attr.set_context_flags().debug().set();
        gl_attr.set_context_version(3, 2);

        // @TODO test with these to be pixel perfect
        // Enable anti-aliasing
        gl_attr.set_multisample_buffers(1);
        gl_attr.set_multisample_samples(4);

        // @TODO use config info
        let window = sdl_context.video_subsystem.window("Codename Dash", 1280, 960)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|name| sdl_context.video_subsystem.gl_get_proc_address(name) as *const _);

        window.gl_make_current(&gl_context).unwrap();

        // @TODO video system
        // @XXX testing how to get some display info
        let video_driver = sdl_context.video_subsystem.current_video_driver();
        println!("Video driver: {}", video_driver);

        let num_video_displays = sdl_context.video_subsystem.num_video_displays().unwrap();
        println!("Video displays: {}", num_video_displays);

        for i in 0..num_video_displays {
            let display_mode = sdl_context.video_subsystem.desktop_display_mode(i).unwrap();
            let display_name = sdl_context.video_subsystem.display_name(i).unwrap();
            let display_dpi  = sdl_context.video_subsystem.display_dpi(i).unwrap();
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

    pub fn present(&self) {
        self.window.gl_swap_window();
    }
}
