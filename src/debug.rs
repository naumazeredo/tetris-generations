extern crate sdl2;
extern crate imgui;
extern crate imgui_opengl_renderer;

use imgui::*;
use super::imgui_sdl2;

// TODO move all this to Render?
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

    pub fn render<S, F: Fn(&Ui, &mut S)>(
        &mut self,
        window: &sdl2::video::Window,
        event_pump: &sdl2::EventPump,
        state: &mut S,
        render_info: F
    ) {
        self.imgui_sdl2.prepare_frame(
            self.imgui.io_mut(),
            window,
            &event_pump.mouse_state()
        );

        let ui = self.imgui.frame();

        imgui::Window::new(imgui::im_str!("Debug"))
            .build(&ui, || {
                ui.text(format!("Application average {:.3} ms/frame ({:.1} FPS)",
                    1000.0 / ui.io().framerate, ui.io().framerate));

                ui.separator();

                render_info(&ui, state);
            });

        self.imgui_sdl2.prepare_render(&ui, window);
        self.imgui_renderer.render(ui);
    }
}
