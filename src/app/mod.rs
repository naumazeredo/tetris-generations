extern crate sdl2;
extern crate imgui_opengl_renderer;

//pub mod animation_system;
pub mod asset_system;
pub mod audio;
pub mod debug;
pub mod font_system;
pub mod game_state;
pub mod id_manager;
#[macro_use] pub mod imgui_wrapper;
pub mod input;
pub mod network;
#[macro_use] pub mod renderer;
pub mod sdl;
pub mod transform;
pub mod time_system;
pub mod utils;
pub mod ui;
pub mod video_system;

pub use {
    //animation_system::*,
    audio::*,
    font_system::*,
    game_state::*,
    id_manager::*,
    input::*,
    imgui_wrapper::*,
    network::*,
    renderer::*,
    transform::*,
    time_system::*,
    utils::*,
    video_system::*,
};

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::event::WindowEvent;

use asset_system::*;
use debug::*;
use ui::UiSystem; // We don't give public access to ui structs since it has too many name conflicts
use sdl::*;
//use time_system::*;

#[derive(ImDraw)]
pub struct App<'a> {
    //animation_system: AnimationSystem,
    asset_system: AssetSystem,
    audio_system: AudioSystem<'a>,
    font_system: FontSystem,
    input_system: InputSystem,
    renderer: Renderer,
    sdl_context: SdlContext,
    time_system: TimeSystem,
    ui_system: UiSystem,
    video_system: VideoSystem, // @Maybe give public access to be able to mess with window freely

    running: bool,
    // @Fix clicking on this bool in imgui window will make imgui consume all events
    show_debug_window: bool,

    // Fixed timestep variables
    time_step_interval: u64,
    time_step_accumulator: u64,
}

impl App<'_> {
    fn new(config: AppConfig) -> Self {
        // @TODO check results

        let time_step_interval = config.time_step_interval;

        let sdl_context = SdlContext::new();
        let video_system = VideoSystem::new(config, sdl_context.video_subsystem.clone());
        let audio_system = AudioSystem::new();

        let font_system = FontSystem::new(&sdl_context.ttf_context);
        let input_system = InputSystem::new(sdl_context.controller_subsystem.clone());
        let time_system = TimeSystem::new(sdl_context.timer_subsystem.clone());
        let renderer = Renderer::new(video_system.window.size());

        //let animation_system = AnimationSystem::new();

        let ui_system = UiSystem::new();

        let asset_system = AssetSystem::new();

        Self {
            //animation_system,
            asset_system,
            audio_system,

            sdl_context,
            video_system,

            font_system,
            input_system,
            time_system,

            ui_system,
            renderer,

            running: true,

            show_debug_window: false,

            time_step_interval,
            time_step_accumulator: 0,
        }
    }

    fn run<S: GameState + ImDraw>(&mut self, mut state: S, mut debug: Debug) {
        self.new_frame();

        while self.running {
            self.new_frame();

            let events: Vec<Event> = self.sdl_context.event_pump.poll_iter().collect();
            for event in events.into_iter() {
                // Update input system
                // This needs to be done before state since it needs to be consistent.
                // We might remove the asserts from the input system and make it handle the
                // inconsistencies
                let timestamp = self.time_system.real_time;
                self.input_system.handle_input(&event, timestamp);

                if self.ui_system.handle_input(&event) { continue; }
                if debug.handle_input(&event) { continue; }

                // Handle game input first to allow it consuming the input
                // This can be useful if the game has some meta components, like
                // not allowing you to close the window, or changing how it handles
                // window focus/minimize/maximize, etc
                if state.handle_input(&event, self) { continue; }

                self.handle_input(&event);
            }

            /*
            // @Fix input system doesn't work properly with fixed timestep updates. Enabling fixed
            //      timestep right (at 60Hz, with 144Hz rendering) makes 2 frames have the same
            //      input state since real_time doesn't update when the accumulator is below the
            //      interval. Should input always use real time?
            // Fixed timestep update

            // Avoid spiral of death
            if self.time_system.game_frame_duration >= 100_000 {
                self.time_system.game_frame_duration = 100_000;
                println!("[app][run] application is running too slow!");
            }

            self.time_step_accumulator += self.time_system.real_frame_duration;

            while self.time_step_accumulator >= self.time_step_interval {
                state.update(self.time_step_interval, self);

                self.advance_time(self.time_step_interval);
                self.time_step_accumulator -= self.time_step_interval;
            }

            // @TODO interpolate remaining time
            */

            self.update_ui_system_input_state();
            state.update(self.last_frame_real_duration(), self);

            self.advance_time(self.last_frame_real_duration());

            self.update_ui_system_input_state();

            // Render
            Renderer::prepare_render();
            state.render(self);
            self.queue_draw_uis();
            self.render_queued();

            if self.show_debug_window {
                debug.render(self, |ui, app| {
                    state.imdraw("State", ui);
                    app.imdraw("App", ui);
                });
            }

            // @Maybe move this to renderer
            self.video_system.swap_buffers();
        }
    }

    pub fn exit(&mut self) {
        self.running = false;
    }

    fn handle_input(&mut self, event: &Event) {
        match event {
            Event::Quit {..} => { self.running = false; }

            Event::KeyDown { scancode: Some(Scancode::F1), .. } => {
                self.show_debug_window = !self.show_debug_window;
            }

            Event::Window { win_event: WindowEvent::SizeChanged(w, h), .. } |
            Event::Window { win_event: WindowEvent::Resized(w, h), .. } => {
                self.renderer.window_resize_callback((*w as u32, *h as u32));
            }

            _ => {}
        }
    }
}

pub struct AppConfig {
    pub window_name: String,
    pub window_size: (u32, u32),
    pub window_position: Option<(i32, i32)>,
    pub window_resizable: bool,
    // pub start_screen_mode: FullscreenType,

    pub time_step_interval: u64,
}

/*
// @Maybe use init_state and remove GameState::new from the trait
pub fn run<S: GameState, F>(config: AppConfig, init_state: F)
where
    F: FnOnce(&mut App<S>) -> S
{
    let mut app = App::new(config);
    let state = init_state(&mut app);
*/

pub fn run<S: GameState + ImDraw>(config: AppConfig)
{
    let mut app = App::new(config);
    let state = S::new(&mut app);
    let debug = Debug::new(&app.video_system.window);

    app.run(state, debug);
}
