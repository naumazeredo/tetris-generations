// @Refactor remove debug with ImGui and create our own editor immediate gui (or not immediate)

use crate::app::imgui::imgui_sdl2;
use imgui::*;
use super::{App, GameState};

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
}

impl<S: GameState> App<'_, S> {
    pub fn handle_debug_event(&mut self, event: &sdl2::event::Event) -> bool {
        self.debug.imgui_sdl2.handle_event(&mut self.debug.imgui, event);
        return self.debug.imgui_sdl2.ignore_event(&event);
    }

    pub fn render_debug<F: Fn(&Ui, &mut S)>(
        &mut self,
        state: &mut S,
        render_info: F
    ) {
        self.debug.imgui_sdl2.prepare_frame(
            self.debug.imgui.io_mut(),
            &self.video_system.window,
            &self.sdl_context.event_pump.mouse_state()
        );

        let ui = self.debug.imgui.frame();

        imgui::Window::new(imgui::im_str!("Debug"))
            .build(&ui, || {
                ui.text(format!("Application average {:.3} ms/frame ({:.1} FPS)",
                    1000.0 / ui.io().framerate, ui.io().framerate));

                ui.separator();

                render_info(&ui, state);
            });

        self.debug.imgui_sdl2.prepare_render(&ui, &self.video_system.window);
        self.debug.imgui_renderer.render(ui);
    }
}
