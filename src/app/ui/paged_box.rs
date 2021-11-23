use crate::app::App;
use super::*;

#[derive(Copy, Clone, Debug)]
pub struct PagedBox {
    id: Id,
    lines_per_page: u32,
}

// @Refactor this is not matching the other widgets methods...
impl PagedBox {
    pub fn builder(name: &str, lines_per_page: u32) -> Self {
        Self {
            id: Id::new(name).add("#__pagedbox"),
            lines_per_page,
        }
    }

    #[inline(always)] pub fn build(
        self,
        app: &mut App
    ) -> Option<PagedBoxPlacer> {
        self.build_with_placer(&mut app.ui_system.top_ui().index(), app)
    }

    #[inline(always)] pub fn build_with_placer<P: Placer>(
        self,
        placer: &mut P,
        app: &mut App
    ) -> Option<PagedBoxPlacer> {
        paged_box_internal(self, placer, app)
    }
}

// @Refactor this should be PagedBoxState to match other widgets
#[derive(Copy, Clone, Debug)]
pub struct PagedBoxPlacer {
    ui_index: UiIndex,
    id: Id,

    layout: Layout,
    cursor: Vec2i,
    same_line_cursor: Vec2i,
    padding: Vec2i,

    // @Hack this is a workaround. Widgets should return when the first element can't be placed and
    //       leave the placer in the original state no elements can be placed
    spacing_count: u32,
    next_on_same_line: bool,
}

impl Placer for PagedBoxPlacer {
    fn place_element(&mut self, id: Id, size: Vec2i, app: &mut App) -> Option<Layout> {
        // Check if the element should be placed and update the number of lines in paged box
        let state = app.ui_system.states.get_mut(&self.id).unwrap();

        let is_visible;
        if let ElementVariant::PagedBox {
            lines_per_page,
            current_page,
            num_lines,
        } = &mut state.variant {
            let current_line = *num_lines - (self.next_on_same_line as u32);

            is_visible = current_line / *lines_per_page == *current_page;

            if !self.next_on_same_line {
                *num_lines += 1;
            }
        } else {
            unreachable!();
        }

        if !is_visible {
            let ui = self.ui(app);
            self.cursor.x -= self.spacing_count as i32 * ui.style.spacing;
            self.spacing_count = 0;

            self.next_on_same_line = false;
            return None;
        }

        // Element if visible, so should be placed

        let ui = self.ui(app);

        let layout = Layout {
            pos: self.cursor + self.padding,
            size,
        };

        ui.elements.push(Element { id, layout });

        self.spacing_count = 0;
        self.next_on_same_line = false;

        self.same_line_cursor.x = self.cursor.x + layout.size.x;
        self.same_line_cursor.y = self.cursor.y;

        self.cursor.x = self.layout.pos.x;
        self.cursor.y += layout.size.y + 2 * self.padding.y;

        if app.ui_system.input_focus == Some(id) {
            app.ui_system.found_input_focus = true;
        }

        Some(layout)
    }

    fn ui<'a>(&mut self, app: &'a mut App) -> &'a mut Ui { self.ui_index.ui(app) }

    fn same_line(&mut self, _app: &mut App) {
        self.cursor = self.same_line_cursor;
        self.next_on_same_line = true;
    }

    fn draw_width(&mut self, _app: &mut App) -> i32 {
        self.layout.size.x - 2 * self.padding.x
    }

    fn add_padding(&mut self, padding: Vec2i, _app: &mut App) {
        self.padding += padding;
    }

    fn remove_padding(&mut self, _app: &mut App) {
        self.padding = Vec2i::new();
    }

    fn add_spacing(&mut self, app: &mut App) {
        let ui = self.ui(app);
        self.spacing_count += 1;
        self.cursor.x += ui.style.spacing;
    }
}

// ----

fn new_paged_box(paged_box: PagedBox) -> State {
    State {
        disabled: false,
        pressed:  false,
        down:     false,
        hovering: false,
        scroll:   0,
        variant: ElementVariant::PagedBox {
            lines_per_page: paged_box.lines_per_page,
            current_page: 0,
            num_lines: 0,
        },
    }
}

fn paged_box_internal<P: Placer>(
    paged_box: PagedBox,
    placer: &mut P,
    app: &mut App,
) -> Option<PagedBoxPlacer> {
    let id = paged_box.id;

    let ui = placer.ui(app);
    let size = Vec2i {
        x: ui.layout.size.x,
        y: ui.style.line_height * (paged_box.lines_per_page + 1) as i32 +
            2 * ui.style.paged_box_border as i32, // Colored border on top and a spacing bottom
    };

    let layout = placer.place_element(id, size, app);
    if layout.is_none() { return None; }
    let layout = layout.unwrap();

    // @Refactor and_modify only after state check since we need the number of lines.
    app.ui_system.states.entry(id).or_insert_with(|| new_paged_box(paged_box));

    let state = app.update_state_interaction(id, layout);

    // @Refactor scroll can be consumed by a nested widgets, so using it here is a problem.
    //           This can only be fixed when we move layout and interaction to the state and update
    //           it on rendering
    if state.scroll != 0 {
        if let ElementVariant::PagedBox {
            lines_per_page,
            current_page,
            num_lines,
        } = &mut state.variant {
            if state.scroll > 0 {
                *current_page = current_page.saturating_sub(1);
            } else if *current_page < (num_lines.saturating_sub(1) / *lines_per_page) {
                *current_page += 1;
            }
        } else {
            unreachable!();
        }
    }

    // @Refactor and_modify only after state check since we need the number of lines.
    app.ui_system.states.entry(id)
        .and_modify(|state| {
            if let ElementVariant::PagedBox {
                //lines_per_page,
                num_lines,
                ..
            } = &mut state.variant {
                //*lines_per_page = paged_box.lines_per_page;
                *num_lines = 0;
            } else {
                unreachable!();
            }
        });

    let ui = placer.ui(app);
    let cursor = layout.pos + Vec2i { x: 0, y: ui.style.paged_box_border as i32 };
    Some(PagedBoxPlacer {
        ui_index: placer.ui(app).index(),
        id,
        layout,
        cursor,
        same_line_cursor: cursor,
        padding: Vec2i::new(),
        spacing_count: 0,
        next_on_same_line: false,
    })
}
