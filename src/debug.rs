extern crate sdl2;
extern crate imgui;
extern crate imgui_opengl_renderer;

use super::imgui_sdl2::{self};
use super::engine::Engine;

pub struct Debug {
    imgui: imgui::Context,
    imgui_sdl2: imgui_sdl2::ImguiSdl2,
    imgui_renderer: imgui_opengl_renderer::Renderer,
}

impl Debug {
    pub fn new(window: &sdl2::video::Window) -> Self {

        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        let imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);

        let video_subsystem = window.subsystem();

        let imgui_renderer = imgui_opengl_renderer::Renderer::new(
            &mut imgui,
            |s| video_subsystem.gl_get_proc_address(s) as _
        );

        Self {
            imgui,
            imgui_sdl2,
            imgui_renderer
        }
    }
}

pub fn handle_event(debug: &mut Debug, event: &sdl2::event::Event) -> bool {
    debug.imgui_sdl2.handle_event(&mut debug.imgui, event);
    return debug.imgui_sdl2.ignore_event(&event);
}

pub fn render(engine: &mut Engine) {
    // debug
    engine.debug.imgui_sdl2.prepare_frame(
        engine.debug.imgui.io_mut(),
        &engine.window,
        &engine.event_pump.mouse_state()
    );

    let ui = engine.debug.imgui.frame();

    imgui::Window::new(imgui::im_str!("Debug"))
        .build(&ui, || {
            ui.text(format!("Application average {:.3} ms/frame ({:.1} FPS)",
                1000.0 / ui.io().framerate, ui.io().framerate));
        });

    //ui.show_demo_window(&mut true);

    engine.debug.imgui_sdl2.prepare_render(&ui, &engine.window);
    engine.debug.imgui_renderer.render(ui);

}
