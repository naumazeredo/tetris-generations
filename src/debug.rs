extern crate sdl2;
extern crate imgui;
extern crate imgui_opengl_renderer;

use imgui::*;
use super::imgui_sdl2::{self};
use super::app::App;
use super::game::GameState;

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

    /*
    pub fn prepare_render(&mut self, app: &App) {
    }
    */

    pub fn render(&mut self, app: &mut App, state: &mut GameState) {
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

                ui.separator();

                Drag::new(im_str!("x")).speed(0.1).build(&ui, &mut state.x);
                Drag::new(im_str!("y")).speed(0.1).build(&ui, &mut state.y);
                Drag::new(im_str!("r")).speed(0.1).build(&ui, &mut state.r);
                Drag::new(im_str!("px")).speed(0.1).build(&ui, &mut state.px);
                Drag::new(im_str!("py")).speed(0.1).build(&ui, &mut state.py);
                Drag::new(im_str!("w")).speed(0.1).build(&ui, &mut state.w);
                Drag::new(im_str!("h")).speed(0.1).build(&ui, &mut state.h);
                Drag::new(im_str!("l")).build(&ui, &mut state.l);

                let mut c: [f32; 4] = state.c.into();
                ColorEdit::new(im_str!("c"), &mut c).build(&ui);
                state.c = super::render::Color::from(c);
            });

        self.imgui_sdl2.prepare_render(&ui, &app.window);
        self.imgui_renderer.render(ui);
    }
}
