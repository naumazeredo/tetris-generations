pub mod render;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use sdl2::event::Event;
use crate::linalg::Vec2i;
use crate::app::{
    App,
    ImDraw,
    font_system::calculate_draw_text_size,
    renderer::color::{self, Color},
    utils::fnv_hasher::FNVHasher,
};

#[derive(ImDraw)]
pub(in crate::app) struct UiSystem {
    states: HashMap<Id, State>,
    uis: Vec<Ui>,

    input_focus: Option<Id>,
    input_state: String, // @Refactor buffer allocation instead of regular reallocs
    input_state_buffer: String, // @Refactor buffer allocation instead of regular reallocs
    input_variant: InputVariant,
    input_complete: bool, // @Refactor this should be removed when we add an UI default input mapping

    input_cursor_timestamp: u64,

    // Placer variables
    indentation: u8,
    cursor: Vec2i,
    same_line_cursor: Vec2i,

    // Frame variables
    found_input_focus: bool,
}

impl UiSystem {
    pub(in crate::app) fn new() -> Self {
        Self {
            states: HashMap::new(),
            uis: Vec::new(),

            input_focus: None,
            input_state: String::new(),
            input_state_buffer: String::new(),
            input_variant: InputVariant::Str { max_length: 8 },
            input_complete: false,

            input_cursor_timestamp: 0,

            indentation: 0,
            cursor: Vec2i::new(),
            same_line_cursor: Vec2i::new(),

            found_input_focus: false,
        }
    }

    pub(in crate::app) fn handle_input(&mut self, event: &Event) -> bool {
        use sdl2::keyboard::Scancode;

        match event {
            Event::TextInput { text, .. } => {
                if self.input_focus.is_some() {
                    self.add_input(text);
                    return true;
                }
            }

            Event::TextEditing { text, start, length, .. } => {
                println!("text editing ({}, {}): \"{}\"", start, length, text);
            }

            // @TODO use input mapping
            Event::KeyDown { scancode: Some(Scancode::Backspace), .. } => {
                if self.input_focus.is_some() {
                    self.input_state.pop();
                }
            }

            // @TODO use input mapping
            Event::KeyDown { scancode: Some(Scancode::Return), .. } => {
                if self.input_focus.is_some() {
                    self.input_complete = true;
                }
            }

            _ => {}
        }

        false
    }

    // Verifies if the current input state appending the new text is valid, if it is, update the
    // input state, otherwise do nothing.
    fn add_input(&mut self, text: &str) {
        let new_input = [self.input_state.as_str(), text].concat();

        match self.input_variant {
            InputVariant::Str { max_length } => {
                if new_input.len() <= max_length {
                    self.input_state = new_input;
                }
            }

            InputVariant::I32 { min, max, .. } => {
                // Accept the minus sign by itself
                if new_input == "-" {
                    if let Some(x) = min {
                        if x >= 0 { return; }
                    }
                    self.input_state = new_input.to_owned();
                } else {
                    // @FixMe without min/max, under/overflowing i32 won't saturate to min/max i32
                    match new_input.parse::<i32>() {
                        Ok(mut num) => {
                            if let Some(x) = min { num = std::cmp::max(num, x); }
                            if let Some(x) = max { num = std::cmp::min(num, x); }

                            self.input_state = format!("{}", num);
                        }
                        Err(_) => {},
                    }
                }
            }

            /*
            InputVariant::U32 { min, max } => {
                match new_input.parse::<u32>() {
                    Ok(num) => self.input_state = format!("{}", num),
                    Err(_) => {},
                }
            }
            */
        }
    }
}

#[derive(ImDraw)]
pub struct Ui {
    style: Style,
    layout: Layout,
    elements: Vec<Element>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ImDraw)]
struct Id(u64);

// @TODO macro this
impl Id {
    pub fn new(s: &str) -> Self {
        let mut hasher = FNVHasher::new();
        s.hash(&mut hasher);
        Self(hasher.finish())
    }

    pub fn add(self, s: &str) -> Self {
        let mut hasher = FNVHasher::cont(self.0);
        s.hash(&mut hasher);
        Self(hasher.finish())
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id({})", self.0)
    }
}

#[derive(Clone, ImDraw)]
pub struct Style {
    //font_id: FontId,
    spacing:     i32,
    indent_size: i32,
    padding:     i32,

    background_color: Color,

    input_cursor_duration: u64,
    input_cursor_size: u32,
    input_cursor_padding: i32,

    //border_color: Color,
    //border_thickness: u32,

    text_color: Color,
    font_size:  u32,
    //header_font_size: u32,

    box_color:       Color,
    box_hover_color: Color,
    box_down_color:  Color,

    button_padding: i32,

    checkbox_box_size: u32,
    checkbox_unselected_color:       Color,
    checkbox_unselected_hover_color: Color,
    checkbox_selected_color:         Color,
    checkbox_selected_hover_color:   Color,

    input_box_width: u32,
    input_box_padding: i32,
    input_focused_color: Color,

    slider_box_width:    u32,
    slider_box_height:   u32,
    slider_box_padding:  i32,
    slider_box_color:    Color,
    slider_cursor_width: u32,
    slider_cursor_hover_color:     Color,
    slider_cursor_unfocused_color: Color,
    slider_cursor_focused_color:   Color,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            spacing:     12,
            indent_size: 12,
            padding:     12,

            background_color: color::BLACK,

            input_cursor_duration: 500_000,
            input_cursor_size: 4,
            input_cursor_padding: 4,

            text_color: color::WHITE,
            font_size: 20,

            box_color:       Color { r: 0.3, g: 0.3, b: 0.3, a: 0.5 },
            box_hover_color: Color { r: 0.5, g: 0.5, b: 0.5, a: 0.5 },
            box_down_color:  Color { r: 1.0, g: 1.0, b: 1.0, a: 0.5 },

            button_padding: 8,

            checkbox_box_size: 24,
            checkbox_unselected_color:       Color { r: 0.3, g: 0.3, b: 0.3, a: 0.5 },
            checkbox_unselected_hover_color: Color { r: 0.5, g: 0.5, b: 0.5, a: 0.5 },
            checkbox_selected_color:         Color { r: 1.0, g: 1.0, b: 1.0, a: 0.5 },
            checkbox_selected_hover_color:   Color { r: 0.8, g: 0.8, b: 0.8, a: 0.5 },

            input_box_width: 48,
            input_box_padding: 4,
            input_focused_color: Color { r: 0.8, g: 0.8, b: 1.0, a: 0.5 },

            slider_box_width:    128,
            slider_box_height:   20,
            slider_box_padding:  4,
            slider_box_color:    Color { r: 0.2, g: 0.2, b: 0.2, a: 0.5 },
            slider_cursor_width: 12,
            slider_cursor_hover_color:     Color { r: 0.8, g: 0.8, b: 0.8, a: 0.5 },
            slider_cursor_unfocused_color: Color { r: 0.5, g: 0.5, b: 0.5, a: 0.5 },
            slider_cursor_focused_color:   Color { r: 1.0, g: 1.0, b: 1.0, a: 0.5 },
        }
    }
}

#[derive(Copy, Clone, Default, Debug, ImDraw)]
pub struct Layout {
    pub pos: Vec2i,
    pub size: Vec2i,
}

#[derive(ImDraw)]
struct Element {
    id: Id,
    layout: Layout,
}

#[derive(Debug, ImDraw)]
struct State {
    pressed: bool, // = true only if down and mouse released on top of the button
    down: bool,
    hovering: bool,

    // Some if element is focusable, Some(true) if it's currently the focus, Some(false) otherwise
    //focus: Option<bool>,

    //custom_style: Option<Style>,

    variant: ElementVariant,
}

#[derive(Debug, ImDraw)]
enum ElementVariant {
    Text     { text: String, },
    Button   { text: String, },
    Checkbox { value: bool },
    Input    {
        input_focus: Option<bool>,
        input_complete: bool,

        value_str: String,
        variant: InputVariant,
    },
    Slider {
        percent: f32,
        variant: SliderVariant,
    }
}

#[derive(Copy, Clone, Debug, ImDraw)]
enum InputVariant {
    Str { max_length: usize },

    // @TODO macro this to multiple types
    I32 { value: i32, min: Option<i32>, max: Option<i32> },
    //U32 { value: u32, min: Option<u32>, max: Option<u32> },
}

#[derive(Copy, Clone, Debug, ImDraw)]
enum SliderVariant {
    // @TODO macro this to multiple types
    I32 { value: i32, min: i32, max: i32 },
    //U32 { value: u32, min: u32, max: u32 },
}

impl State {
    fn new_text(text: &str) -> Self {
        Self {
            pressed: false,
            down: false,
            hovering: false,
            variant: ElementVariant::Text { text: text.to_owned() },
        }
    }

    fn new_button(text: &str) -> Self {
        Self {
            pressed: false,
            down: false,
            hovering: false,
            variant: ElementVariant::Button { text: text.to_owned() },
        }
    }

    fn new_checkbox(value: bool) -> Self {
        Self {
            pressed: false,
            down: false,
            hovering: false,
            variant: ElementVariant::Checkbox { value },
        }
    }

    fn new_input_i32(value: i32, min: Option<i32>, max: Option<i32>) -> Self {
        Self {
            pressed: false,
            down: false,
            hovering: false,
            variant: ElementVariant::Input {
                input_focus: Some(false),
                input_complete: false,

                value_str: format!("{}", value),
                variant: InputVariant::I32 { value, min, max },
            },
        }
    }

    fn new_input_str(value: &str, max_length: usize) -> Self {
        Self {
            pressed: false,
            down: false,
            hovering: false,
            variant: ElementVariant::Input {
                input_focus: Some(false),
                input_complete: false,

                value_str: value.to_owned(),
                variant: InputVariant::Str { max_length },
            },
        }
    }

    fn new_slider_i32(value: i32, min: i32, max: i32) -> Self {
        let percent = (value - min) as f32 / (max - min) as f32;
        Self {
            pressed: false,
            down: false,
            hovering: false,
            variant: ElementVariant::Slider {
                percent,
                variant: SliderVariant::I32 { value, min, max },
            },
        }
    }
}

impl<S> App<'_, S> {
    pub fn new_ui(&mut self, layout: Layout) {
        let style = Style::default();
        self.ui_system.cursor = layout.pos + Vec2i { x: style.padding, y: style.padding };
        self.ui_system.same_line_cursor = layout.pos + Vec2i { x: style.padding, y: style.padding };

        let ui = Ui {
            style,
            layout,
            elements: Vec::new(),
        };

        self.ui_system.uis.push(ui);
    }

    pub fn new_ui_with_style(&mut self, layout: Layout, style: Style) {
        self.ui_system.cursor = layout.pos + Vec2i { x: style.padding, y: style.padding };
        self.ui_system.same_line_cursor = layout.pos + Vec2i { x: style.padding, y: style.padding };

        let ui = Ui {
            style,
            layout,
            elements: Vec::new(),
        };

        self.ui_system.uis.push(ui);
    }

    //------------------
    // Text
    //------------------

    pub fn text(&mut self, text: &str) {
        let id = Id::new(text);
        let layout = self.new_layout(self.calculate_text_size(text));

        self.add_element(id, layout);

        // @TODO check if text should be updated
        //       Maybe create a function that compares the strings (or the string ids) and swap the
        //       contents in case they are different
        self.ui_system.states.entry(id)
            .or_insert_with(|| State::new_text(text));
    }

    //------------------
    // Button
    //------------------

    pub fn button(&mut self, text: &str) -> bool {
        // @Maybe add text using the app.text method instead of calculating everything

        let id = Id::new(text);

        // @TODO cleanup these ui.last_mut().unwrap() calls
        // Calculate element size
        let button_padding = self.ui_system.uis.last().unwrap().style.button_padding;
        let padding = Vec2i { x: button_padding, y: button_padding };
        let size = self.calculate_text_size(text) + 2 * padding;
        let layout = self.new_layout(size);

        self.add_element(id, layout);

        self.ui_system.states.entry(id)
            .or_insert_with(|| State::new_button(text));

        let state = self.update_state_interaction(id, layout);
        state.pressed
    }

    //------------------
    // Checkbox
    //------------------

    pub fn checkbox(&mut self, label: &str, value: &mut bool) {
        // Add label
        self.text(label);
        self.same_line();

        // Update/create box state
        let id = Id::new(label).add("#checkbox");

        let ui = &self.ui_system.uis.last().unwrap();
        let checkbox_box_size = ui.style.checkbox_box_size;
        let size = Vec2i { x: checkbox_box_size as i32, y: checkbox_box_size as i32 };
        let layout = self.new_layout(size);

        self.ui_system.states.entry(id)
            .and_modify(|state| {
                match &mut state.variant {
                    ElementVariant::Checkbox { value: v } => *v = *value,
                    _ => unreachable!()
                }
            })
            .or_insert_with(|| State::new_checkbox(*value));

        let state = self.update_state_interaction(id, layout);
        if state.pressed {
            *value = !*value;

            // @XXX should we update the saved state or just be delayed by 1 frame?
            match &mut state.variant {
                ElementVariant::Checkbox { value: v } => *v = !*v,
                _ => unreachable!()
            }
        }

        self.add_element(id, layout);
    }

    //------------------
    // Input integer
    //------------------

    fn input_i32_internal(&mut self, label: &str, value: &mut i32, min: Option<i32>, max: Option<i32>) {
        // Add label
        self.text(label);
        self.same_line();

        let id = Id::new(label).add("#input");

        let ui = &self.ui_system.uis.last().unwrap();
        let size = Vec2i {
            x: ui.style.input_box_width as i32,
            y: ui.style.font_size as i32 + 2 * ui.style.input_box_padding,
        };
        let layout = Layout {
            pos: Vec2i {
                x: self.ui_system.cursor.x,
                y: self.ui_system.cursor.y - ui.style.input_box_padding,
            },
            size
        };

        self.ui_system.states.entry(id)
            .and_modify(|state| {
                // Update the value
                if let ElementVariant::Input {
                    value_str,
                    variant,
                    ..
                } = &mut state.variant {
                    if let InputVariant::I32 { value: v, .. } = variant {
                        if *v != *value {
                            *v = *value;
                            *value_str = format!("{}", *value);
                        }
                    } else {
                        unreachable!();
                    }
                }
            })
            .or_insert_with(|| State::new_input_i32(*value, min, max));

        let state = self.update_state_interaction(id, layout);
        if let ElementVariant::Input {
            input_complete: true,
            value_str,
            variant,
            ..
        } = &mut state.variant {
            *value = i32::from_str_radix(&value_str, 10).unwrap_or_default();
            *value_str = format!("{}", *value);

            if let InputVariant::I32 { value: v, .. } = variant {
                *v = *value;
            } else {
                unreachable!();
            }
        }

        self.add_element(id, layout);
    }

    pub fn input_i32(&mut self, label: &str, value: &mut i32) {
        self.input_i32_internal(label, value, None, None);
    }

    pub fn input_i32_range(&mut self, label: &str, value: &mut i32, min: i32, max: i32) {
        self.input_i32_internal(label, value, Some(min), Some(max));
    }

    //------------------
    // Input string
    //------------------

    fn input_str_internal(&mut self, label: &str, value: &mut String, max_length: usize) {
        // Add label
        self.text(label);
        self.same_line();

        let id = Id::new(label).add("#input");

        let ui = &self.ui_system.uis.last().unwrap();
        let size = Vec2i {
            x: ui.style.input_box_width as i32,
            y: ui.style.font_size as i32 + 2 * ui.style.input_box_padding,
        };
        let layout = Layout {
            pos: Vec2i {
                x: self.ui_system.cursor.x,
                y: self.ui_system.cursor.y - ui.style.input_box_padding,
            },
            size
        };

        // @TODO we should update the input state in case referenced value changed
        self.ui_system.states.entry(id)
            .or_insert_with(|| State::new_input_str(&value, max_length));

        let state = self.update_state_interaction(id, layout);
        if let ElementVariant::Input {
            input_complete: true,
            value_str,
            ..
        } = &mut state.variant {
            *value = value_str.clone();
        }

        self.add_element(id, layout);
    }

    pub fn input_str(&mut self, label: &str, value: &mut String) {
        self.input_str_internal(label, value, 64);
    }

    pub fn input_str_with_max_length(&mut self, label: &str, value: &mut String, max_length: usize) {
        self.input_str_internal(label, value, max_length);
    }

    //------------------
    // Slider integer
    //------------------

    // @TODO macro this to multiple types
    // @TODO accept a format string for the type
    pub fn slider_i32(&mut self, label: &str, value: &mut i32, min: i32, max: i32) {
        // Add label
        self.text(label);
        self.same_line();

        let id = Id::new(label).add("#slider");

        let ui = &self.ui_system.uis.last().unwrap();
        let size = Vec2i {
            x: ui.style.slider_box_width as i32,
            y: ui.style.slider_box_height as i32,
        };
        let layout = Layout {
            pos: Vec2i {
                x: self.ui_system.cursor.x,
                y: self.ui_system.cursor.y + (ui.style.font_size as i32 - ui.style.slider_box_height as i32) / 2,
            },
            size
        };

        // @TODO we should update the input state in case referenced value changed
        self.ui_system.states.entry(id)
            .and_modify(|state| {
                if let ElementVariant::Slider {
                    percent,
                    variant: SliderVariant::I32 { value: v, min, max },
                } = &mut state.variant {
                    if *v != *value {
                        *v = *value;
                        *percent = (*v - *min) as f32 / (*max - *min) as f32;
                    }
                } else {
                    unreachable!();
                }
            })
            .or_insert_with(|| State::new_slider_i32(*value, min, max));

        // Copy values that require self reference access
        let mouse_pos_x = self.get_mouse_position().0 as i32;
        let ui = &self.ui_system.uis.last().unwrap();
        let slider_box_padding = ui.style.slider_box_padding;
        let slider_cursor_width = ui.style.slider_cursor_width;

        let state = self.update_state_interaction(id, layout);
        if state.down {
            let mouse_pos_x = mouse_pos_x - layout.pos.x - slider_box_padding;
            let mouse_pos_x = mouse_pos_x as f32 - slider_cursor_width as f32 / 2.0;
            let cursor_horizontal_space =
                layout.size.x - 2 * slider_box_padding - slider_cursor_width as i32;

            let mut new_percent = mouse_pos_x / cursor_horizontal_space as f32;
            if new_percent < 0.0 { new_percent = 0.0; }
            if new_percent > 1.0 { new_percent = 1.0; }

            if let ElementVariant::Slider {
                percent,
                variant: SliderVariant::I32 { value: v, min, max },
            } = &mut state.variant {
                *v = ((*max - *min) as f32 * new_percent + (*min as f32)).round() as i32;
                *percent = (*v - *min) as f32 / (*max - *min) as f32;
                *value = *v;
            } else {
                unreachable!();
            }
        }

        self.add_element(id, layout);

        // Add number value
        self.same_line();
        // @TODO cache this string
        self.text(&format!("{}", value));
    }

    //------------------
    // Layout functions
    //------------------

    pub fn indent(&mut self) {
        let ui = &mut self.ui_system.uis.last_mut().unwrap();
        let indent_size = ui.style.indent_size;
        self.ui_system.cursor.x += indent_size;
        self.ui_system.indentation += 1;
    }

    pub fn unindent(&mut self) {
        let ui = &mut self.ui_system.uis.last_mut().unwrap();
        let indent_size = ui.style.indent_size;
        self.ui_system.cursor.x -= indent_size;
        self.ui_system.indentation -= 1;
    }

    /*
    pub fn padding(&mut self) {
        let ui = &mut self.ui_system.uis.last_mut().unwrap();
        let padding = ui.style.padding;
        self.ui_system.cursor += Vec2i { x: padding, y: 0 };
    }
    */

    pub fn same_line(&mut self) {
        let ui = &mut self.ui_system.uis.last_mut().unwrap();
        let padding = ui.style.padding;

        self.ui_system.cursor = self.ui_system.same_line_cursor;
        self.ui_system.cursor += Vec2i { x: padding, y: 0 };
    }

    // -----------------
    // private functions
    // -----------------

    fn new_layout(&self, size: Vec2i) -> Layout {
        Layout { pos: self.ui_system.cursor, size }
    }

    fn calculate_text_size(&self, text: &str) -> Vec2i {
        let ui = &self.ui_system.uis.last().unwrap();
        calculate_draw_text_size(
            &self.font_system,
            text,
            self.font_system.default_font_id,
            ui.style.font_size as f32,
        ).into()
    }

    fn add_element(&mut self, id: Id, layout: Layout) {
        let ui = &mut self.ui_system.uis.last_mut().unwrap();
        ui.elements.push(Element { id, layout });

        let ui_layout = ui.layout;
        let padding = ui.style.padding;
        let spacing = ui.style.spacing;
        let indent_size = ui.style.indent_size;

        self.ui_system.same_line_cursor.x = self.ui_system.cursor.x + layout.size.x;
        self.ui_system.same_line_cursor.y = self.ui_system.cursor.y;

        self.ui_system.cursor.x = ui_layout.pos.x + padding + indent_size * self.ui_system.indentation as i32;
        self.ui_system.cursor.y += layout.size.y + spacing;

        if self.ui_system.input_focus == Some(id) {
            self.ui_system.found_input_focus = true;
        }
    }

    /*
    fn begin_element(&mut self, element: Element) {
        self.add_element(element, Vec2i::new());
        self.indent();
    }

    fn end_element(&mut self) {
        self.unindent();
    }
    */

    fn get_state(&self, id: Id) -> &State {
        self.ui_system.states.get(&id).unwrap()
    }

    // @TODO somehow refactor this function to be able to have a state tied to a deeper level
    //       of the app, instead of self
    fn update_state_interaction(&mut self, id: Id, layout: Layout) -> &mut State {
        // Get mouse state
        let mouse_pos: Vec2i = self.get_mouse_position().into();
        let mouse_left_pressed = self.mouse_left_pressed();
        let mouse_left_released = self.mouse_left_released();
        let mouse_hovering = mouse_pos.is_inside(layout.pos, layout.size);
        let timestamp = self.real_timestamp();

        let mut state = self.ui_system.states.get_mut(&id).unwrap();

        // Update mouse interaction

        state.pressed = false;
        state.hovering = false;

        // Handle input focus lost and input completion before mouse interactions
        if let ElementVariant::Input { input_focus, input_complete, value_str, ..  } = &mut state.variant {
            *input_complete = false;
            if *input_focus == Some(true) {
                if (mouse_left_released && !mouse_hovering) || self.ui_system.input_complete {
                    // Input completion

                    *input_complete = true;
                    *input_focus = Some(false);

                    // Update the input value to the input_state.
                    // The input_state is saved into input_state_buffer since ui elements are in immediate
                    // mode and the logic to handle having a focused input and clicking on a different
                    // input element would be tricky. Thus, we have a App.update_ui_system function that
                    // stores the input_state into input_state_buffer when we have to update the element
                    // input
                    *value_str = std::mem::take(&mut self.ui_system.input_state_buffer);
                } else if self.ui_system.input_focus.is_none() {
                    // Input focus lost

                    *input_focus = Some(false);
                }
            }
        }

        // Handle mouse interactions
        if mouse_hovering {
            state.hovering = true;
            if mouse_left_pressed {
                state.down = true;
            } else if mouse_left_released {
                state.pressed = true;

                if let ElementVariant::Input {
                    input_focus,
                    variant,
                    value_str,
                    ..
                } = &mut state.variant {
                    if *input_focus == Some(false) {
                        *input_focus = Some(true);

                        self.ui_system.input_focus = Some(id);
                        self.ui_system.input_variant = *variant;

                        println!("focus change: {}", id);
                        println!("input variant: {:?}", self.ui_system.input_variant);

                        // Update input_state to the current input value.
                        self.ui_system.input_state = value_str.clone();

                        self.ui_system.input_cursor_timestamp = timestamp;
                    }
                }
            }
        }

        if mouse_left_released {
            state.down = false;
        }

        state
    }

    // @Refactor this function doesn't seem completely necessary. Maybe a new_frame would fit better
    pub(in crate::app) fn update_ui_system(&mut self) {
        if self.mouse_left_released() && self.ui_system.input_focus.is_some() {
            self.ui_system.input_complete = true;
        }

        if self.ui_system.input_complete {
            self.ui_system.input_focus = None;
            self.ui_system.input_state_buffer = std::mem::take(&mut self.ui_system.input_state);
        }
    }
}
