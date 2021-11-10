mod button;
mod checkbox;
mod combobox;
mod input;
mod render;
mod slider;
mod style;
mod text;

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

pub use button::*;
pub use checkbox::*;
pub use combobox::*;
pub use input::*;
pub use text::*;
pub use slider::*;
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

    modal_open: Option<Id>,
    modal_change: Option<Id>,

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

            modal_open: None,
            modal_change: None,

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
    modal_elements: Vec<Element>,
}

impl Ui {
    fn draw_width(&self) -> i32 {
        self.layout.size.x - 2 * self.style.padding
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ImDraw)]
struct Id(u64);

// @TODO macro this
impl Id {
    fn new(s: &str) -> Self {
        let mut hasher = FNVHasher::new();
        s.hash(&mut hasher);
        Self(hasher.finish())
    }

    fn add(self, s: &str) -> Self {
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
    disabled: bool,

    // Some if element is focusable, Some(true) if it's currently the focus, Some(false) otherwise
    //focus: Option<bool>,

    //custom_style: Option<Style>,

    variant: ElementVariant,
}

#[derive(Debug, ImDraw)]
enum ElementVariant {
    Text     { text: String, },
    Button   { text: String, },
    Checkbox { value: bool, },
    Input    {
        changed: bool,
        is_input_focus: bool,

        value_str: String,
        variant: InputVariant,
    },
    Slider {
        changed: bool,
        percent: f32,
        variant: SliderVariant,
    },
    Combobox {
        changed: bool,
        index: usize,
        text: String,
        scroll_top_index: usize,
    },
    ComboboxOption {
        selected: bool,
        text: String,
    },
    Separator,
    Scrollbar,
}

impl App<'_> {
    pub fn new_ui(&mut self, layout: Layout) {
        let style = Style::default();
        self.ui_system.cursor = layout.pos + Vec2i { x: style.padding, y: style.padding };
        self.ui_system.same_line_cursor = layout.pos + Vec2i { x: style.padding, y: style.padding };

        let ui = Ui {
            style,
            layout,
            elements: Vec::new(),
            modal_elements: Vec::new(),
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
            modal_elements: Vec::new(),
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

    // @Cleanup this seems unnecessary. The design will change and we will be able to remove this
    fn new_layout_right(&self, size: Vec2i) -> Layout {
        let ui = &self.ui_system.uis.last().unwrap();
        let pos = Vec2i {
            x: ui.layout.pos.x + ui.layout.size.x - size.x - ui.style.padding,
            y: self.ui_system.cursor.y,
        };

        Layout { pos, size }
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


    // @TODO no need for this same line/next line logic. We won't use it and it's making the whole
    //       design worse (this is too general)
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
        //self.ui_system.cursor.y += layout.size.y + spacing;
        self.ui_system.cursor.y += ui.style.line_height + spacing;

        if self.ui_system.input_focus == Some(id) {
            self.ui_system.found_input_focus = true;
        }
    }

    fn add_modal_element(&mut self, id: Id, layout: Layout) {
        let ui = &mut self.ui_system.uis.last_mut().unwrap();
        ui.modal_elements.push(Element { id, layout });
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

    fn is_mouse_hovering_clipped_layout(&self, layout: Layout) -> bool {
        let mouse_pos: Vec2i = self.get_mouse_position().into();

        let ui = &self.ui_system.uis.last().unwrap();
        let padding = Vec2i { x: ui.style.padding, y: ui.style.padding };
        let ui_layout = ui.layout;

        return
            mouse_pos.is_inside(ui_layout.pos, ui_layout.size - padding) &&
            mouse_pos.is_inside(layout.pos, layout.size)
        ;
    }

    fn is_mouse_hovering_layout(&self, layout: Layout) -> bool {
        let mouse_pos: Vec2i = self.get_mouse_position().into();
        mouse_pos.is_inside(layout.pos, layout.size)
    }

    // @TODO somehow refactor this function to be able to have a state tied to a deeper level
    //       of the app, instead of self
    fn update_state_interaction(&mut self, id: Id, layout: Layout) -> &mut State {
        // @TODO only update if mouse is inside the element container (we will need to propagate
        //       the container size)

        // Get mouse state
        let mouse_left_pressed = self.mouse_left_pressed();
        let mouse_left_released = self.mouse_left_released();
        let mouse_hovering = self.is_mouse_hovering_clipped_layout(layout);

        let mut state = self.ui_system.states.get_mut(&id).unwrap();

        // Update mouse interaction

        state.pressed = false;
        state.hovering = false;

        // Check modal opened
        if self.ui_system.modal_open.is_some() {
            state.down = false;
            return state;
        }

        // Handle mouse interactions
        if mouse_hovering {
            state.hovering = true;
            if mouse_left_pressed {
                state.down = true;
            } else if mouse_left_released {
                state.pressed = true;
            }
        }

        if mouse_left_released {
            state.down = false;
        }

        state
    }

    // @TODO somehow refactor this function to be able to have a state tied to a deeper level
    //       of the app, instead of self
    fn update_modal_state_interaction(&mut self, id: Id, layout: Layout) -> &mut State {
        // @TODO only update if mouse is inside the element container (we will need to propagate
        //       the container size)

        // Get mouse state
        let mouse_pos: Vec2i = self.get_mouse_position().into();
        let mouse_left_pressed = self.mouse_left_pressed();
        let mouse_left_released = self.mouse_left_released();
        let mouse_hovering = mouse_pos.is_inside(layout.pos, layout.size);

        let mut state = self.ui_system.states.get_mut(&id).unwrap();

        // Update mouse interaction

        state.pressed = false;
        state.hovering = false;

        // Handle mouse interactions
        if mouse_hovering {
            state.hovering = true;
            if mouse_left_pressed {
                state.down = true;
            } else if mouse_left_released {
                state.pressed = true;
            }
        }

        if mouse_left_released {
            state.down = false;
            self.ui_system.modal_change = None;
        }

        state
    }

    // @Refactor this function doesn't seem completely necessary. Maybe a new_frame would fit better
    pub(in crate::app) fn update_ui_system_input_state(&mut self) {
        if self.mouse_left_released() && self.ui_system.input_focus.is_some() {
            self.ui_system.input_complete = true;
        }

        if self.ui_system.input_complete {
            self.ui_system.input_focus = None;
            self.ui_system.input_state_buffer = std::mem::take(&mut self.ui_system.input_state);
        }

        self.ui_system.modal_open = self.ui_system.modal_change;
    }
}
