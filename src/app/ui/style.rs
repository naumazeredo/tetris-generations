use crate::app::{
    ImDraw,
    renderer::color::{self, Color},
};


#[derive(Clone, ImDraw)]
pub struct Style {
    //font_id: FontId,
    pub spacing:     i32,
    pub indent_size: i32,
    pub padding:     i32,
    pub line_height: i32,

    pub background_color: Color,

    // @TODO move to specific files

    pub input_cursor_duration: u64,
    pub input_cursor_size: u32,
    pub input_cursor_padding: i32,

    //border_color: Color,
    //border_thickness: u32,

    pub text_color: Color,
    pub font_size:  u32,
    //header_font_size: u32,

    pub box_color:       Color,
    pub box_hover_color: Color,
    pub box_down_color:  Color,
    pub box_padding: i32,
    pub box_width: u32,

    pub checkbox_unselected_color:       Color,
    pub checkbox_unselected_hover_color: Color,
    pub checkbox_selected_color:         Color,
    pub checkbox_selected_hover_color:   Color,

    pub input_focused_color: Color,

    pub slider_box_padding:  i32,
    pub slider_box_color:    Color,
    pub slider_cursor_width: u32,
    pub slider_cursor_hover_color:     Color,
    pub slider_cursor_unfocused_color: Color,
    pub slider_cursor_focused_color:   Color,

    pub combobox_selected_option_color: Color,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            spacing:     4,
            indent_size: 12,
            padding:     12,
            line_height: 28,

            background_color: Color { r: 0.1, g: 0.0, b: 0.1, a: 1.0 },

            input_cursor_duration: 500_000,
            input_cursor_size: 4,
            input_cursor_padding: 4,

            text_color: color::WHITE,
            font_size: 20,

            box_color:       Color { r: 0.3, g: 0.3, b: 0.3, a: 0.5 },
            box_hover_color: Color { r: 0.5, g: 0.5, b: 0.5, a: 0.5 },
            box_down_color:  Color { r: 1.0, g: 1.0, b: 1.0, a: 0.5 },
            box_padding: 4,
            box_width: 168,

            checkbox_unselected_color:       Color { r: 0.3, g: 0.3, b: 0.3, a: 0.5 },
            checkbox_unselected_hover_color: Color { r: 0.5, g: 0.5, b: 0.5, a: 0.5 },
            checkbox_selected_color:         Color { r: 1.0, g: 1.0, b: 1.0, a: 0.5 },
            checkbox_selected_hover_color:   Color { r: 0.8, g: 0.8, b: 0.8, a: 0.5 },

            input_focused_color: Color { r: 0.8, g: 0.8, b: 1.0, a: 0.5 },

            slider_box_padding:  4,
            slider_box_color:    Color { r: 0.2, g: 0.2, b: 0.2, a: 0.5 },
            slider_cursor_width: 20,
            slider_cursor_hover_color:     Color { r: 0.8, g: 0.8, b: 0.8, a: 0.5 },
            slider_cursor_unfocused_color: Color { r: 0.5, g: 0.5, b: 0.5, a: 0.5 },
            slider_cursor_focused_color:   Color { r: 1.0, g: 1.0, b: 1.0, a: 0.5 },

            combobox_selected_option_color: Color { r: 1.0, g: 1.0, b: 1.0, a: 0.5 },
        }
    }
}