pub struct SdlContext {
    pub sdl: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub timer_subsystem: sdl2::TimerSubsystem,
    _sdl_image_context: sdl2::image::Sdl2ImageContext,
}

impl SdlContext {
    pub fn new() -> Self {
        // @TODO check results

        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let timer_subsystem = sdl.timer().unwrap();

        let _sdl_image_context = sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();

        Self {
            sdl,
            video_subsystem,
            timer_subsystem,
            _sdl_image_context,
        }
    }
}
