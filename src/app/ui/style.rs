use crate::app::{
    ImDraw,
    renderer::color::{self, Color},
};
use crate::linalg::Vec2i;


#[derive(Clone, ImDraw)]
pub struct Style {
    //font_id: FontId,

    pub spacing: i32,
    pub padding: Vec2i,

    pub line_height:  i32,
    pub line_padding: Vec2i,

    pub background_color: Color,

    pub line_focus_background_color: Color,

    // @TODO move to specific files

    pub input_cursor_duration: u64,
    pub input_cursor_size: u32,
    pub input_cursor_padding: i32,

    //border_color: Color,
    //border_thickness: u32,

    pub text_color: Color,
    pub text_disabled_color: Color,
    //pub text_indent_size: i32,
    pub text_size: u32,

    pub box_color:          Color,
    pub box_hover_color:    Color,
    pub box_down_color:     Color,
    pub box_disabled_color: Color,
    pub box_padding: i32,
    pub box_width: u32,

    pub checkbox_unselected_color:          Color,
    pub checkbox_unselected_hover_color:    Color,
    pub checkbox_selected_color:            Color,
    pub checkbox_selected_hover_color:      Color,
    pub checkbox_unselected_disabled_color: Color,
    pub checkbox_selected_disabled_color:   Color,

    pub input_focused_color: Color,

    pub slider_box_padding:  i32,
    pub slider_cursor_width: u32,
    pub slider_cursor_hover_color:     Color,
    pub slider_cursor_unfocused_color: Color,
    pub slider_cursor_focused_color:   Color,
    pub slider_cursor_disabled_color:  Color,

    pub combobox_selected_option_color: Color,
    pub combobox_option_background_color: Color,

    pub scrollbar_width: u32,
    pub scrollbar_color: Color,

    pub paged_box_border: u32,
    pub paged_box_background:        Color,
    pub paged_box_index_background:  Color,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            // @Refactor next 4 values are only used for placing, maybe we should move them or copy
            //           to placers to remove App dependency
            spacing: 4,
            padding: Vec2i { x: 0, y: 12 },

            line_height:  28,

            // y should be less than line_height/2
            // y should be less than ui.height/2
            // x should be less than ui.width/2
            // @Maybe we can force line_height to be at least 2*y and UI to have min_size at least
            //        2*padding
            line_padding: Vec2i { x: 12, y: 1 },

            background_color: Color { r: 0.05, g: 0.05, b: 0.05, a: 1.0 },

            line_focus_background_color: Color { r: 0.05, g: 0.1, b: 0.0, a: 1.0 },

            input_cursor_duration: 500_000,
            input_cursor_size: 4,
            input_cursor_padding: 4,

            text_color: color::WHITE,
            text_disabled_color: Color { r: 0.6, g: 0.6, b: 0.6, a: 1.0 },
            //text_indent_size: 12,
            text_size: 16, // @Maybe font_size?

            box_color:          Color { r: 0.15, g: 0.15, b: 0.15, a: 1.0 },
            box_hover_color:    Color { r: 0.25, g: 0.25, b: 0.25, a: 1.0 },
            box_down_color:     Color { r: 0.6, g: 0.6, b: 0.6, a: 1.0 },
            box_disabled_color: Color { r: 0.05, g: 0.05, b: 0.05, a: 1.0 },
            box_padding: 4,
            box_width: 168,

            checkbox_unselected_color:          Color { r: 0.15, g: 0.15, b: 0.15, a: 1.0 },
            checkbox_unselected_hover_color:    Color { r: 0.25, g: 0.25, b: 0.25, a: 1.0 },
            checkbox_selected_color:            Color { r: 0.5, g: 0.5, b: 0.5, a: 1.0 },
            checkbox_selected_hover_color:      Color { r: 0.4, g: 0.4, b: 0.4, a: 1.0 },
            checkbox_unselected_disabled_color: Color { r: 0.1, g: 0.1, b: 0.1, a: 1.0 },
            checkbox_selected_disabled_color:   Color { r: 0.3, g: 0.3, b: 0.3, a: 1.0 },

            input_focused_color: Color { r: 0.8, g: 0.8, b: 1.0, a: 1.0 },

            slider_box_padding:  4,
            slider_cursor_width: 20,
            slider_cursor_hover_color:     Color { r: 0.4, g: 0.4, b: 0.4, a: 1.0 },
            slider_cursor_unfocused_color: Color { r: 0.5, g: 0.5, b: 0.5, a: 1.0 },
            slider_cursor_focused_color:   Color { r: 0.6, g: 0.6, b: 0.6, a: 1.0 },
            slider_cursor_disabled_color:  Color { r: 0.3, g: 0.3, b: 0.3, a: 1.0 },

            combobox_selected_option_color: Color { r: 0.3, g: 0.3, b: 0.3, a: 1.0 },
            combobox_option_background_color: Color { r: 0.2, g: 0.2, b: 0.2, a: 1.0 },

            scrollbar_width: 4,
            scrollbar_color: Color { r: 0.8, g: 0.8, b: 0.8, a: 0.8 },

            paged_box_border: 2,
            paged_box_background:        Color { r: 0.03, g: 0.03, b: 0.03, a: 1.0 },
            paged_box_index_background:  Color { r: 0.1, g: 0.1, b: 0.1, a: 1.0 },
        }
    }
}
