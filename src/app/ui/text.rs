use std::panic::Location;
use crate::app::App;
use super::*;

pub struct Text<'a> {
    text: &'a str,
    max_width: Option<u32>,
    disabled: bool,
}

impl<'a> Text<'a> {
    #[track_caller]
    pub fn new(text: &str, app: &mut App) {
        Text::builder(text).build(app);
    }

    pub fn builder<'b: 'a>(text: &'b str) -> Self {
        Self {
            text,
            disabled: false,
            max_width: None,
        }
    }

    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            disabled,
            ..self
        }
    }

    pub fn max_width(self, max_width: u32) -> Self {
        Self {
            max_width: Some(max_width),
            ..self
        }
    }

    #[track_caller]
    #[inline(always)] pub fn build(self, app: &mut App) -> Option<()> {
        self.build_with_placer(&mut app.ui_system.top_ui().index(), app)
    }

    #[track_caller]
    pub fn build_with_placer<P: Placer>(
        self,
        placer: &mut P,
        app: &mut App
    ) -> Option<()> {
        let id = Id::new(Location::caller());

        let line_padding = placer.ui(app).style.line_padding;
        placer.add_padding(line_padding, app);
        let opt = text_internal(id, self, placer, app);
        placer.remove_padding(app);
        opt
    }
}

// ----

fn new_text(text: &str, disabled: bool) -> State {
    State {
        disabled,
        pressed:  false,
        down:     false,
        hovering: false,
        scroll: 0,
        focused: false,
        variant: ElementVariant::Text { text: text.to_owned() },
    }
}

// @TODO return a state like all other widgets
pub(super) fn text_internal<P: Placer>(
    id: Id,
    text: Text,
    placer: &mut P,
    app: &mut App,
) -> Option<()> {
    let render_size = app.calculate_text_size(text.text);
    let ui = placer.ui(app);

    let size = Vec2i {
        x: text.max_width.unwrap_or(render_size.x as u32) as i32,
        y: ui.line_draw_height(),
    };

    if placer.place_element(id, size, app).is_none() { return None; }

    // @TODO check if text should be updated
    //       Maybe create a function that compares the strings (or the string ids) and swap the
    //       contents in case they are different
    app.ui_system.states.entry(id)
        .and_modify(|state| {
            state.disabled = text.disabled;
        })
        .or_insert_with(|| new_text(text.text, text.disabled));

    Some(())
}
