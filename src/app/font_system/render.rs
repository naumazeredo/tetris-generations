use crate::{
    app::{
        App,
        font_system::FontId,
        renderer::{
            color::Color,
            text::{
                queue_draw_text,
                queue_draw_text_with_max_width,
            }
        },
    },
    transform::Transform,
};

impl App<'_> {
    pub fn queue_draw_text(
        &mut self,
        //program: ShaderProgram,
        text: &str,
        transform: &Transform,
        font_size: f32,
        color: Color,
    ) {
        queue_draw_text(
            &mut self.renderer,
            &self.font_system,
            //program,
            text,
            self.font_system.default_font_id,
            transform,
            font_size,
            color,
        );
    }

    pub fn queue_draw_text_with_font(
        &mut self,
        //program: ShaderProgram,
        text: &str,
        font: FontId,
        transform: &Transform,
        font_size: f32,
        color: Color,
    ) {
        queue_draw_text(
            &mut self.renderer,
            &self.font_system,
            //program,
            text,
            font,
            transform,
            font_size,
            color,
        );
    }

    pub fn queue_draw_text_with_max_width(
        &mut self,
        //program: ShaderProgram,
        text: &str,
        transform: &Transform,
        font_size: f32,
        max_width: u32,
        color: Color,
    ) {
        queue_draw_text_with_max_width(
            &mut self.renderer,
            &self.font_system,
            //program,
            text,
            self.font_system.default_font_id,
            transform,
            font_size,
            max_width,
            color,
        );
    }
}
