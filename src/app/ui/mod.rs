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
    input_flags: InputFlags,
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
            input_flags: InputFlags::default(),
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

    fn try_update_input(&mut self, new_input: &str) {
        if self.input_flags.contains(InputFlags::INTEGER) {
            if self.input_flags.contains(InputFlags::SIGNED) {
                // Accept the minus sign by itself
                if new_input == "-" {
                    self.input_state = new_input.to_owned();
                    return;
                } else {
                    match new_input.parse::<i32>() {
                        Ok(num) => self.input_state = format!("{}", num),
                        Err(_) => {},
                    }
                }
            } else {
                match new_input.parse::<u32>() {
                    Ok(num) => self.input_state = format!("{}", num),
                    Err(_) => {},
                }
            }
        } else {
            unreachable!();
        }
    }

    fn add_input(&mut self, text: &str) {
        if self.input_flags.contains(InputFlags::INTEGER) {
            let s = [self.input_state.as_str(), text].concat();
            self.try_update_input(&s);
        } else {
            self.input_state.push_str(text);
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

    // If it's focusable, and if has focus or not
    input_focus: Option<bool>,
    input_flags: InputFlags,
    input_complete: bool,

    //custom_style: Option<Style>,

    variant: ElementVariant,
}

#[derive(Debug, ImDraw)]
enum ElementVariant {
    Text     { text: String, },
    Button   { text: String, },
    Checkbox { value: bool },
    Input    { value: String },
}

impl State {
    fn new_text(text: &str) -> Self {
        Self {
            pressed: false,
            down: false,
            hovering: false,
            input_focus: None,
            input_flags: InputFlags::default(),
            input_complete: false,
            variant: ElementVariant::Text { text: text.to_owned() },
        }
    }

    fn new_button(text: &str) -> Self {
        Self {
            pressed: false,
            down: false,
            hovering: false,
            input_focus: None,
            input_flags: InputFlags::default(),
            input_complete: false,
            variant: ElementVariant::Button { text: text.to_owned() },
        }
    }

    fn new_checkbox(value: bool) -> Self {
        Self {
            pressed: false,
            down: false,
            hovering: false,
            input_focus: None,
            input_flags: InputFlags::default(),
            input_complete: false,
            variant: ElementVariant::Checkbox { value },
        }
    }

    fn new_input_i32(value: i32) -> Self {
        Self {
            pressed: false,
            down: false,
            hovering: false,
            input_focus: Some(false),
            input_flags: InputFlags::INTEGER | InputFlags::SIGNED,
            input_complete: false,
            variant: ElementVariant::Input { value: format!("{}", value) },
        }
    }

    fn new_input_str(value: &str) -> Self {
        Self {
            pressed: false,
            down: false,
            hovering: false,
            input_focus: Some(false),
            input_flags: InputFlags::default(),
            input_complete: false,
            variant: ElementVariant::Input { value: value.to_owned() },
        }
    }
}

bitflags! {
    #[derive(Default, ImDraw)]
    struct InputFlags : u8 {
        const INTEGER = 0b00000001;
        const SIGNED  = 0b00000010;
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

    pub fn checkbox(&mut self, text: &str, value: &mut bool) {
        // Add text
        self.text(text);
        self.same_line();

        // Update/create box state
        let id = Id::new(text).add("#checkbox");

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

    pub fn input_i32(&mut self, text: &str, value: &mut i32) {
        // Add text
        self.text(text);
        self.same_line();

        let id = Id::new(text).add("#input");

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
            .or_insert_with(|| State::new_input_i32(*value));

        let state = self.update_state_interaction(id, layout);
        if state.input_complete {
            if let ElementVariant::Input { value: v } = &mut state.variant {
                if !v.is_empty() {
                    *value = i32::from_str_radix(&v, 10).unwrap_or_default();
                }
            } else { panic!("text focusable element that is not an ElementVariant::Input"); }
        }

        self.add_element(id, layout);
    }

    pub fn input_str(&mut self, text: &str, value: &mut String) {
        // Add text
        self.text(text);
        self.same_line();

        let id = Id::new(text).add("#input");

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
            .or_insert_with(|| State::new_input_str(&value));

        let state = self.update_state_interaction(id, layout);
        if state.input_complete {
            if let ElementVariant::Input { value: v } = &mut state.variant {
                *value = v.clone();
            } else { panic!("text focusable element that is not an ElementVariant::Input"); }
        }

        self.add_element(id, layout);
    }

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
        state.input_complete = false;

        // Handle input focus lost and input completion before mouse interactions
        if state.input_focus == Some(true) {
            if (mouse_left_released && !mouse_hovering) || self.ui_system.input_complete {
                // Input completion

                state.input_complete = true;
                state.input_focus = Some(false);

                // Update the input value to the input_state.
                // The input_state is saved into input_state_buffer since ui elements are in immediate
                // mode and the logic to handle having a focused input and clicking on a different
                // input element would be tricky. Thus, we have a App.update_ui_system function that
                // stores the input_state into input_state_buffer when we have to update the element
                // input
                if let ElementVariant::Input { value } = &mut state.variant {
                    *value = std::mem::take(&mut self.ui_system.input_state_buffer);
                } else { panic!("text focusable element that is not an ElementVariant::Input"); }
            } else if self.ui_system.input_focus.is_none() {
                // Input focus lost

                state.input_focus = Some(false);
            }
        }

        // Handle mouse interactions
        if mouse_hovering {
            state.hovering = true;
            if mouse_left_pressed {
                state.down = true;
            } else if mouse_left_released {
                state.pressed = true;

                if state.input_focus == Some(false) {
                    state.input_focus = Some(true);

                    self.ui_system.input_focus = Some(id);
                    self.ui_system.input_flags = state.input_flags;
                    println!("focus change: {}", id);
                    println!("input flags: {:?}", self.ui_system.input_flags);

                    // Update input_state to the current input value.
                    if let ElementVariant::Input { value } = &mut state.variant {
                        self.ui_system.input_state = value.clone();
                    } else { panic!("text focusable element that is not an ElementVariant::Input"); }

                    self.ui_system.input_cursor_timestamp = timestamp;
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
