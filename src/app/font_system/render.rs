use crate::{
    app::{
        App,
        font_system::FontId,
        renderer::{
            color::Color,
            font::queue_draw_text,
        },
    },
    transform::Transform,
};

impl<S> App<'_, S>{
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
}
