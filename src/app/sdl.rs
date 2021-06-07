use crate::app::imgui_wrapper::ImDraw;

pub(in crate::app) struct SdlContext {
    pub(in crate::app) sdl: sdl2::Sdl,
    pub(in crate::app) event_pump: sdl2::EventPump,
    pub(in crate::app) video_subsystem: sdl2::VideoSubsystem,
    pub(in crate::app) timer_subsystem: sdl2::TimerSubsystem,
    pub(in crate::app) controller_subsystem: sdl2::GameControllerSubsystem,
    pub(in crate::app) ttf_context: sdl2::ttf::Sdl2TtfContext,

    // Hidden since we don't need to use it directly but dropping it closes the subsystem
    _sdl_image_context: sdl2::image::Sdl2ImageContext,
}

impl SdlContext {
    pub(in crate::app) fn new() -> Self {
        // @TODO check results

        let sdl = sdl2::init().unwrap();
        let event_pump = sdl.event_pump().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let timer_subsystem = sdl.timer().unwrap();
        let controller_subsystem = sdl.game_controller().unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();

        let _sdl_image_context = sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();

        Self {
            sdl,
            event_pump,
            video_subsystem,
            timer_subsystem,
            controller_subsystem,
            ttf_context,
            _sdl_image_context,
        }
    }
}

// ImDraw
// Not sure how I can automate this and not just copy the whole structure...
// No compile time reflection in Rust is the root of this issue.
impl_imdraw_todo!(sdl2::keyboard::Scancode);
impl_imdraw_todo!(sdl2::mouse::MouseButton);
impl_imdraw_todo!(sdl2::controller::Button);
impl_imdraw_todo!(sdl2::controller::Axis);
