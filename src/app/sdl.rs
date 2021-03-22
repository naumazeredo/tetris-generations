use crate::app::imgui::ImDraw;

pub struct SdlContext {
    pub sdl: sdl2::Sdl,
    pub event_pump: sdl2::EventPump,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub timer_subsystem: sdl2::TimerSubsystem,
    pub controller_subsystem: sdl2::GameControllerSubsystem,

    // Hidden since we don't need to use it directly but dropping it closes the subsystem
    _sdl_image_context: sdl2::image::Sdl2ImageContext,
}

impl SdlContext {
    pub fn new() -> Self {
        // @TODO check results

        let sdl = sdl2::init().unwrap();
        let event_pump = sdl.event_pump().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let timer_subsystem = sdl.timer().unwrap();
        let controller_subsystem = sdl.game_controller().unwrap();

        let _sdl_image_context = sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();

        Self {
            sdl,
            event_pump,
            video_subsystem,
            timer_subsystem,
            controller_subsystem,
            _sdl_image_context,
        }
    }
}

// ImDraw
impl_imdraw_todo!(sdl2::keyboard::Scancode);
impl_imdraw_todo!(sdl2::mouse::MouseButton);
impl_imdraw_todo!(sdl2::controller::Button);
impl_imdraw_todo!(sdl2::controller::Axis);
