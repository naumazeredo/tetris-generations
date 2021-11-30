use std::panic::Location;
use crate::app::App;
use super::*;

#[derive(Copy, Clone, Debug)]
pub struct PagedBox {
    lines_per_page: u32,
}

// @Refactor this is not matching the other widgets methods...
impl PagedBox {
    pub fn builder(lines_per_page: u32) -> Self {
        Self {
            lines_per_page,
        }
    }

    #[track_caller]
    #[inline(always)] pub fn build(
        self,
        app: &mut App
    ) -> Option<PagedBoxPlacer> {
        self.build_with_placer(&mut app.ui_system.top_ui().index(), app)
    }

    // @Refactor this should return PagedBoxState to match other widgets
    #[track_caller]
    pub fn build_with_placer<P: Placer>(
        self,
        placer: &mut P,
        app: &mut App
    ) -> Option<PagedBoxPlacer> {
        let id = Id::new(Location::caller());
        paged_box_internal(id, self, placer, app)
    }
}

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
    spacing: i32,
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
            self.cursor.x -= self.spacing;
            self.spacing = 0;

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

        self.spacing = 0;
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

    fn cursor(&mut self, _app: &mut App) -> Vec2i { self.cursor }

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

    fn add_custom_spacing(&mut self, spacing: i32, app: &mut App) {
        let ui = self.ui(app);
        self.spacing += spacing;
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
        focused: false,
        variant: ElementVariant::PagedBox {
            lines_per_page: paged_box.lines_per_page,
            current_page: 0,
            num_lines: 0,
        },
    }
}

fn paged_box_internal<P: Placer>(
    id: Id,
    paged_box: PagedBox,
    placer: &mut P,
    app: &mut App,
) -> Option<PagedBoxPlacer> {
    let ui = placer.ui(app);
    let size = Vec2i {
        x: ui.layout.size.x,
        y: ui.style.line_height * (paged_box.lines_per_page + 1) as i32 +
        //y: ui.style.line_height * paged_box.lines_per_page as i32 +
            2 * ui.style.paged_box_border as i32, // Colored border on top and a spacing bottom
    };

    let layout = placer.place_element(id, size, app);
    if layout.is_none() { return None; }
    let layout = layout.unwrap();

    // @Refactor and_modify only after state check since we need the number of lines.
    app.ui_system.states.entry(id).or_insert_with(|| new_paged_box(paged_box));

    let state = app.update_state_interaction(id, layout);
    let scroll = -state.scroll;

    /*
    // @TODO move ui layout to be done just before rendering. Centering the index line is very
    //       annoying we need the whole information about paged box (current lines, num lines, etc)
    //       and it just seems as a waste of time
    // Index line

    // get this information while we have the state to avoid doing it again inside the if
    let index_text;
    if let ElementVariant::PagedBox {
        lines_per_page,
        current_page,
        num_lines,
    } = &mut state.variant {
        index_text = format!("{}/{}",
            *current_page + 1,
            (num_lines.saturating_sub(1) / *lines_per_page) + 1,
        );
    } else {
        unreachable!();
    }

    let ui = placer.ui(app);
    let size = Vec2i {
        x: ui.layout.size.x,
        y: ui.style.line_height,
    };

    let index_layout = placer.place_element(id, size, app);
    if let Some(index_layout) = index_layout {
        let prev_arrow_size = app.calculate_text_size("<");
        let next_arrow_size = app.calculate_text_size(">");
        let index_size = app.calculate_text_size(&index_text);

        let ui = placer.ui(app);

        let index_text_size = prev_arrow_size.x + index_size.x + next_arrow_size.x +
            2 * ui.style.spacing;

        let line_padding = ui.style.line_padding; // @TODO (add_custom_padding) add_padding should use line_padding by default
        placer.add_padding(line_padding, app);
        let draw_width = placer.draw_width(app);
        placer.add_custom_spacing((draw_width - index_text_size) / 2, app);
    }
    */

    // @Refactor scroll can be consumed by a nested widgets, so using it here is a problem.
    //           This can only be fixed when we move layout and interaction to the state and update
    //           it on rendering
    if scroll != 0 {
        let state = app.ui_system.states.get_mut(&id).unwrap();

        if let ElementVariant::PagedBox {
            lines_per_page,
            current_page,
            num_lines,
        } = &mut state.variant {
            if scroll < 0 {
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
        spacing: 0,
        next_on_same_line: false,
    })
}
