use std::panic::Location;
use crate::app::{
    App,
    renderer,
};
use super::*;

#[derive(Default)]
pub struct TextureState {
    pub pressed:  bool,
    pub down:     bool,
    pub hovering: bool,
}

pub struct Texture {
    texture: renderer::Texture,
}

impl Texture {
    #[track_caller]
    pub fn new(texture: renderer::Texture, app: &mut App) -> TextureState {
        Texture::builder(texture).build(app)
    }

    pub fn builder(texture: renderer::Texture) -> Self {
        Self { texture }
    }

    #[track_caller]
    #[inline(always)] pub fn build(
        self,
        app: &mut App
    ) -> TextureState {
        self.build_with_placer(&mut app.ui_system.top_ui().index(), app)
    }

    #[track_caller]
    pub fn build_with_placer<P: Placer>(
        self,
        placer: &mut P,
        app: &mut App
    ) -> TextureState {
        let id = Id::new(Location::caller());

        texture_internal(id, self, placer, app).and_then(|state| {
            Some(TextureState {
                pressed:  state.pressed,
                down:     state.down,
                hovering: state.hovering,
            })
        }).unwrap_or_default()
    }
}

// -------------

fn new_texture(texture: Texture) -> State {
    State {
        disabled: false,
        pressed:  false,
        down:     false,
        hovering: false,
        scroll:   0,
        focused: false,
        variant: ElementVariant::Texture { texture: texture.texture },
    }
}

fn texture_internal<'a, P: Placer>(
    id: Id,
    texture: Texture,
    placer: &mut P,
    app: &'a mut App,
) -> Option<&'a State> {
    let draw_width = placer.draw_width(app);
    let placer_pos = placer.cursor(app);

    // Texture layout
    let widget_layout = Layout {
        pos: placer_pos + Vec2i { x: (draw_width - texture.texture.w as i32) / 2, y: 0 },
        size: Vec2i { x: texture.texture.w as i32, y: texture.texture.h as i32 },
    };

    // The texture will be centered in the placer
    let size = Vec2i { x: draw_width, y: texture.texture.h as i32 };
    let layout = placer.place_element(id, size, app);

    if layout.is_none() { return None; }

    app.ui_system.states.entry(id)
        .or_insert_with(|| new_texture(texture));

    // Update widget state
    let state = app.update_state_interaction(id, widget_layout);
    Some(state)
}
