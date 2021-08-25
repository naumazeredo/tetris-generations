use crate::app::imgui_wrapper::ImDraw;

#[derive(ImDraw)]
pub(in crate::app) struct SdlContext {
    pub(in crate::app) sdl: sdl2::Sdl,
    pub(in crate::app) event_pump: sdl2::EventPump,
    pub(in crate::app) video_subsystem: sdl2::VideoSubsystem,
    pub(in crate::app) timer_subsystem: sdl2::TimerSubsystem,
    pub(in crate::app) controller_subsystem: sdl2::GameControllerSubsystem,
    //pub(in crate::app) haptic_subsystem: sdl2::HapticSubsystem,
    pub(in crate::app) ttf_context: sdl2::ttf::Sdl2TtfContext,

    // Hidden since we don't need to use it directly but dropping it closes the subsystem
    _sdl_mixer_context: sdl2::mixer::Sdl2MixerContext,
    _sdl_image_context: sdl2::image::Sdl2ImageContext,
}

impl SdlContext {
    pub(in crate::app) fn new() -> Self {
        // @TODO check results

        //sdl2::hint::set("SDL_HINT_JOYSTICK_HIDAPI_PS4_RUMBLE", "1");

        let sdl = sdl2::init().unwrap();
        let event_pump = sdl.event_pump().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let timer_subsystem = sdl.timer().unwrap();
        let controller_subsystem = sdl.game_controller().unwrap();
        //let haptic_subsystem = sdl.haptic().unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();

        let _sdl_mixer_context = sdl2::mixer::init(sdl2::mixer::InitFlag::OGG).unwrap();
        let _sdl_image_context = sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();

        // @XXX Dear ImGUI is starting the text input for us. Is this a problem?
        //video_subsystem.text_input().stop();

        Self {
            sdl,
            event_pump,
            video_subsystem,
            timer_subsystem,
            controller_subsystem,
            //haptic_subsystem,
            ttf_context,
            _sdl_mixer_context,
            _sdl_image_context,
        }
    }
}

// ImDraw
// Not sure how I can automate this and not just copy the whole structure...
// No compile time reflection in Rust is the root of this issue.
impl_imdraw_todo!(sdl2::keyboard::Scancode);
impl_imdraw_todo!(sdl2::mouse::MouseButton);
impl_imdraw_todo!(sdl2::controller::Axis);
impl_imdraw_todo!(sdl2::controller::Button);
impl_imdraw_todo!(sdl2::controller::GameController);

impl_imdraw_blank!(sdl2::Sdl);
impl_imdraw_blank!(sdl2::EventPump);
impl_imdraw_blank!(sdl2::VideoSubsystem);
impl_imdraw_blank!(sdl2::TimerSubsystem);
impl_imdraw_blank!(sdl2::GameControllerSubsystem);
//impl_imdraw_blank!(sdl2::HapticSubsystem);
impl_imdraw_blank!(sdl2::ttf::Sdl2TtfContext);
impl_imdraw_blank!(sdl2::image::Sdl2ImageContext);
impl_imdraw_blank!(sdl2::mixer::Sdl2MixerContext);

impl_imdraw_blank!(sdl2::video::Window);
impl_imdraw_blank!(sdl2::video::GLContext);
