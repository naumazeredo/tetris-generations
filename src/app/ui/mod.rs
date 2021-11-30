mod button;
mod checkbox;
mod combobox;
mod input;
mod paged_box;
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
    renderer::Sprite,
    utils::fnv_hasher::FNVHasher,
};

pub use button::*;
pub use checkbox::*;
pub use combobox::*;
pub use input::*;
pub use paged_box::*;
pub use slider::*;
use style::*;
pub use text::*;

#[derive(ImDraw)]
pub(in crate::app) struct UiSystem {
    states:  HashMap<Id, State>,
    //layouts: HashMap<Id, Layout>, // @TODO calculate layout pre-render

    uis: Vec<Ui>,

    input_focus: Option<Id>,
    input_state: String, // @Refactor buffer allocation instead of regular reallocs
    input_state_buffer: String, // @Refactor buffer allocation instead of regular reallocs
    input_variant: InputVariant,
    input_complete: bool, // @Refactor this should be removed when we add an UI default input mapping
    input_cursor_timestamp: u64,

    modal_open: Option<Id>,
    modal_change: Option<Id>,

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

    fn get_ui<'a>(&'a mut self, index: u32) -> &'a mut Ui {
        &mut self.uis[index as usize]
    }

    fn top_ui<'a>(&'a mut self) -> &'a mut Ui {
        self.uis.last_mut().unwrap()
    }
}

pub struct UiBuilder {
    //id: Id,
    layout: Layout,
    style: Option<Style>,
    adjust_height: bool, // @TODO this should be part of Layout
}

impl UiBuilder {
    pub fn style(&mut self, style: Style) -> &mut Self {
        self.style = Some(style);
        self
    }

    pub fn fixed_height(&mut self) -> &mut Self {
        self.adjust_height = false;
        self
    }

    pub fn build(&mut self, app: &mut App<'_>) {
        let style = std::mem::take(&mut self.style);
        let style = style.unwrap_or_else(|| Style::default());

        // @Maybe move the block below to Ui
        // ---
        let index = app.ui_system.uis.len() as u32;
        let cursor = self.layout.pos + style.padding;

        let ui = Ui {
            index,

            style,
            layout: self.layout,
            elements: Vec::new(),
            modal_elements: Vec::new(),
            adjust_height: self.adjust_height,

            lines: Vec::new(),
            focused_line: None,

            cursor,
            same_line_cursor: cursor,
            padding: Vec2i::new(),
        };

        app.ui_system.uis.push(ui);
        // ---
    }
}

pub trait Placer: Copy + Clone {
    fn place_element(&mut self, id: Id, size: Vec2i, app: &mut App) -> Option<Layout>;

    fn ui<'a>(&mut self, app: &'a mut App) -> &'a mut Ui;
    fn draw_width(&mut self, app: &mut App) -> i32;
    fn cursor(&mut self, app: &mut App) -> Vec2i;

    fn same_line(&mut self, app: &mut App);

    // @Refactor this seems so bad...
    fn add_padding(&mut self, padding: Vec2i, app: &mut App);
    fn remove_padding(&mut self, app: &mut App);

    #[inline(always)] fn add_spacing(&mut self, app: &mut App) {
        let ui = self.ui(app);
        let spacing = ui.style.spacing;
        self.add_custom_spacing(spacing, app);
    }

    fn add_custom_spacing(&mut self, spacing: i32, app: &mut App);
}

#[derive(ImDraw)]
pub struct Ui {
    //id: Id,
    index: u32,

    style: Style,
    layout: Layout,

    elements: Vec<Element>,
    modal_elements: Vec<Element>,
    adjust_height: bool,

    lines: Vec<Line>,
    focused_line: Option<u32>,

    // @Refactor move these to UiPlacer?
    // Placer
    cursor: Vec2i,
    same_line_cursor: Vec2i,
    padding: Vec2i,
}

impl Ui {
    pub fn builder(layout: Layout) -> UiBuilder {
        UiBuilder {
            layout,
            style: None,
            adjust_height: true,
        }
    }

    fn index(&self) -> UiIndex { UiIndex(self.index) }

    fn line_draw_height(&self) -> i32 { self.style.line_height - 2 * self.style.line_padding.y }

    fn add_modal_element(&mut self, id: Id, layout: Layout) {
        self.modal_elements.push(Element { id, layout });
    }

    fn add_line(&mut self, widget_id: Id, layout: Layout) -> u32 {
        let index = self.lines.len() as u32;
        self.lines.push(Line { layout, widget_id });
        index
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ImDraw)]
struct UiIndex(u32); // index in uis array

impl Placer for UiIndex {
    fn place_element(&mut self, id: Id, size: Vec2i, app: &mut App) -> Option<Layout> {
        let ui = self.ui(app);

        let layout = Layout {
            pos: ui.cursor + ui.padding,
            size,
        };

        ui.elements.push(Element { id, layout });

        ui.same_line_cursor.x = ui.cursor.x + layout.size.x;
        ui.same_line_cursor.y = ui.cursor.y;

        ui.cursor.x = ui.layout.pos.x + ui.style.padding.x;
        ui.cursor.y += layout.size.y + 2 * ui.padding.y;

        if app.ui_system.input_focus == Some(id) {
            app.ui_system.found_input_focus = true;
        }

        let ui = app.ui_system.get_ui(self.0);
        if ui.adjust_height {
            // @TODO limit by layout max height
            ui.layout.size.y = ui.cursor.y - ui.layout.pos.y + ui.style.padding.y;
        }

        Some(layout)
    }

    fn ui<'a>(&mut self, app: &'a mut App) -> &'a mut Ui { app.ui_system.get_ui(self.0) }

    fn cursor(&mut self, app: &mut App) -> Vec2i { self.ui(app).cursor }

    #[inline(always)] fn same_line(&mut self, app: &mut App) {
        let ui = self.ui(app);
        ui.cursor = ui.same_line_cursor;
    }

    fn draw_width(&mut self, app: &mut App) -> i32 {
        let ui = self.ui(app);
        ui.layout.size.x - 2 * (ui.style.padding.x + ui.padding.x)
    }

    fn add_padding(&mut self, padding: Vec2i, app: &mut App) {
        let ui = self.ui(app);
        ui.padding += padding;
    }

    fn remove_padding(&mut self, app: &mut App) {
        let ui = self.ui(app);
        ui.padding = Vec2i::new();
    }

    fn add_custom_spacing(&mut self, spacing: i32, app: &mut App) {
        let ui = self.ui(app);
        ui.cursor.x += spacing;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ImDraw)]
pub struct Id(u64);

// @TODO macro this
impl Id {
    fn new<H: Hash + ?Sized>(s: &H) -> Self {
        let mut hasher = FNVHasher::new();
        s.hash(&mut hasher);
        Self(hasher.finish())
    }

    fn add<H: Hash + ?Sized>(self, s: &H) -> Self {
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
    disabled: bool,
    pressed: bool, // = true only if down and mouse released on top of the button
    down: bool,
    hovering: bool,
    scroll: i32,

    focused: bool,

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
    Scrollbar,

    PagedBox {
        lines_per_page: u32,
        current_page: u32,
        num_lines: u32,
    },

    /*
    Sprite {
        sprite: Sprite,
    }
    */

    // @TODO Framebuffer

    Separator,
}

#[derive(Copy, Clone, ImDraw)]
struct Line {
    //id: Id,
    layout: Layout,
    widget_id: Id,
    // next/prev_line: Option<Id>,
    // left/right_line? interaction with left/right paged box
}

impl App<'_> {
    // -----------------
    // private functions
    // -----------------

    fn calculate_text_size(&self, text: &str) -> Vec2i {
        let ui = &self.ui_system.uis.last().unwrap();
        let size = calculate_draw_text_size(
            &self.font_system,
            text,
            self.font_system.default_font_id,
            ui.style.text_size as f32,
        );

        Vec2i {
            x: (size.x + 0.5) as i32,
            y: (size.y + 0.5) as i32,
        }
    }

    fn is_mouse_hovering_clipped_layout(&self, layout: Layout) -> bool {
        let mouse_pos: Vec2i = self.get_mouse_position().into();

        let ui = &self.ui_system.uis.last().unwrap();
        let padding = ui.style.padding;
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
    //       of the app, instead of self (tie to UiSystem instead)
    fn update_state_interaction(&mut self, id: Id, layout: Layout) -> &mut State {
        // @TODO only update if mouse is inside the element container (we will need to propagate
        //       the container size)

        // Get mouse state
        let mouse_left_pressed = self.mouse_left_pressed();
        let mouse_left_released = self.mouse_left_released();
        let mouse_hovering = self.is_mouse_hovering_clipped_layout(layout);
        let scroll = self.mouse_scroll();

        let mut state = self.ui_system.states.get_mut(&id).unwrap();

        // Update mouse interaction

        state.pressed = false;
        state.hovering = false;
        state.scroll = 0;

        // Check modal opened
        if self.ui_system.modal_open.is_some() {
            state.down = false;
            return state;
        }

        // Handle mouse interactions
        if mouse_hovering {
            state.hovering = true;
            state.scroll = scroll;
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
        let scroll = self.mouse_scroll();

        let mut state = self.ui_system.states.get_mut(&id).unwrap();

        // Update mouse interaction

        state.pressed = false;
        state.hovering = false;
        state.scroll = 0;

        // Handle mouse interactions
        if mouse_hovering {
            state.hovering = true;
            state.scroll = scroll;
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

    // @TODO UI controller support
    fn update_line_state_interaction(&mut self, ui_index: u32, line_index: u32) {
        // @TODO only update if mouse is inside the element container (we will need to propagate
        //       the container size)

        // Get mouse state
        let line = self.ui_system.get_ui(ui_index).lines[line_index as usize];
        let mouse_hovering = self.is_mouse_hovering_clipped_layout(line.layout);

        // Check modal opened
        if self.ui_system.modal_open.is_some() {
            return;
        }

        // Get widget state
        let mut state = self.ui_system.states.get_mut(&line.widget_id).unwrap();
        state.focused = false;

        // Handle mouse interactions
        if mouse_hovering {
            state.focused = true;
            self.ui_system.get_ui(ui_index).focused_line = Some(line_index);
        }
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
