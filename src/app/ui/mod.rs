pub mod button;
pub mod checkbox;
pub mod input;
pub mod render;
pub mod slider;
pub mod style;
pub mod text;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use sdl2::event::Event;
use crate::linalg::Vec2i;
use crate::app::{
    App,
    ImDraw,
    font_system::calculate_draw_text_size,
    utils::fnv_hasher::FNVHasher,
};

use input::*;
use slider::*;
use style::*;

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
