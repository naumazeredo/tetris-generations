extern crate sdl2;
extern crate imgui;
extern crate imgui_opengl_renderer;

use super::imgui_sdl2::{self};
use super::app::App;

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

    pub fn handle_event(&mut self, event: &sdl2::event::Event) -> bool {
        self.imgui_sdl2.handle_event(&mut self.imgui, event);
        return self.imgui_sdl2.ignore_event(&event);
    }

    pub fn render(&mut self, app: &App) {
        self.imgui_sdl2.prepare_frame(
            self.imgui.io_mut(),
            &app.window,
            &app.event_pump.mouse_state()
        );

        let ui = self.imgui.frame();

        imgui::Window::new(imgui::im_str!("Debug"))
            .build(&ui, || {
                ui.text(format!("Application average {:.3} ms/frame ({:.1} FPS)",
                    1000.0 / ui.io().framerate, ui.io().framerate));
            });

        self.imgui_sdl2.prepare_render(&ui, &app.window);
        self.imgui_renderer.render(ui);
    }
}
