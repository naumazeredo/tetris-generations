// @TODO create ScreenMode to abstract FullscreenType (because FullscreenType is a bad name, with
//       bad variant names)
// @TODO create VSyncMode to abstract SwapInterval (because LateSwapTearing is hard to remember)
pub use sdl2::video::{DisplayMode, FullscreenType, SwapInterval};
use super::{
    App,
    AppConfig,
    ImDraw,
};

// @Maybe refactor? Giving public access to be able to mess with window freely
#[derive(ImDraw)]
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

        let mut window_builder =
            video_subsystem.window(
                &config.window_name,
                config.window_size.0,
                config.window_size.1
            );

        window_builder.opengl();

        if let Some((x, y)) = config.window_position {
            window_builder.position(x, y);
        } else {
            window_builder.position_centered();
        }

        if config.window_resizable {
            window_builder.resizable();
        }

        let window = window_builder.build().unwrap();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        // @TODO enable debug output

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

impl App<'_> {
    pub fn window_size(&self) -> (u32, u32) {
        self.video_system.window.size()
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) {
        self.video_system.window.set_size(width, height).unwrap();
        // @TODO this should set the display mode, in case the window is fullscreen
    }

    pub fn set_window_display_mode(&mut self, display_mode: DisplayMode) {
        self.video_system.window.set_display_mode(display_mode).unwrap();
        self.set_window_size(display_mode.w as u32, display_mode.h as u32);
    }

    pub fn window_display_index(&self) -> i32 {
        self.video_system.window.display_index().unwrap()
    }

    pub fn move_window_to_display(&mut self, display_index: u32) {
        // @XXX workaround on SDL issue: can't move to another window while not on windowed mode
        let screen_mode = self.window_screen_mode();
        if screen_mode != FullscreenType::Off {
            self.set_window_screen_mode(FullscreenType::Off);
        }

        let display_bounds =
            self.sdl_context.video_subsystem.display_bounds(display_index as i32).unwrap();

        let size = self.window_size();
        let w  = size.0  as i32;
        let h  = size.1  as i32;

        self.video_system.window.set_position(
            (display_bounds.x() + (display_bounds.width()  as i32 - w) / 2).into(),
            (display_bounds.y() + (display_bounds.height() as i32 - h) / 2).into()
        );

        if screen_mode != FullscreenType::Off {
            self.set_window_screen_mode(screen_mode);
        }
    }

    // @TODO return an Option
    pub fn window_display_mode(&self) -> DisplayMode {
        match self.video_system.window.display_mode() {
            Ok(mode) => mode,
            Err(err) => {
                // @TODO logging
                println!("Display mode not found: {}", err);
                panic!("");
            }
        }
    }

    pub fn num_displays(&self) -> usize {
        self.sdl_context.video_subsystem.num_video_displays().unwrap() as usize
    }

    // @XXX this seems quite useless: "Generic PnP Monitor"...
    pub fn display_names(&self) -> Vec<String> {
        let num_displays = self.num_displays();
        (0..num_displays)
            .map(|index|
                self.sdl_context.video_subsystem.display_name(index as i32).unwrap()
            )
            .collect()
    }

    pub fn available_display_modes(&self) -> Vec<DisplayMode> {
        let display_index = self.window_display_index();
        let num_display_modes = self.sdl_context.video_subsystem.num_display_modes(display_index).unwrap();

        (0..num_display_modes)
            .map(|mode_index| {
                self.sdl_context.video_subsystem.display_mode(display_index, mode_index).unwrap()
            })
            .collect()
    }

    pub fn available_window_sizes_and_rates(&self) -> Vec<((u32, u32), Vec<u32>)> {
        let display_index = self.window_display_index();
        let num_display_modes = self.sdl_context.video_subsystem.num_display_modes(display_index).unwrap();

        let mut sizes_and_rates = Vec::new();
        let mut size = (0, 0);
        let mut rates = Vec::new();

        (0..num_display_modes)
            .for_each(|mode_index| {
                let display_mode =
                    self.sdl_context.video_subsystem.display_mode(display_index, mode_index).unwrap();

                let mode_size = (display_mode.w as u32, display_mode.h as u32);
                if size == (0, 0) { size = mode_size; }
                if size != mode_size {
                    rates.sort_unstable();
                    rates.reverse();
                    sizes_and_rates.push((size, std::mem::take(&mut rates)));

                    size = mode_size;
                }

                rates.push(display_mode.refresh_rate as u32);
            });

        if !rates.is_empty() {
            rates.sort_unstable();
            rates.reverse();
            sizes_and_rates.push((size, std::mem::take(&mut rates)));
        }

        sizes_and_rates
    }

    pub fn window_screen_mode(&self) -> FullscreenType {
        self.video_system.window.fullscreen_state()
    }

    pub fn set_window_screen_mode(&mut self, screen_mode: FullscreenType) {
        self.video_system.window.set_fullscreen(screen_mode).unwrap();
    }

    pub fn set_vsync(&self, mut vsync: SwapInterval) -> SwapInterval {
        if vsync == SwapInterval::LateSwapTearing {
            if self.sdl_context.video_subsystem.gl_set_swap_interval(vsync).is_ok() {
                return vsync;
            }

            vsync = SwapInterval::VSync;
        }

        self.sdl_context.video_subsystem.gl_set_swap_interval(vsync).unwrap();
        return vsync;
    }

    pub fn vsync(&self) -> SwapInterval {
        self.sdl_context.video_subsystem.gl_get_swap_interval()
    }
}
