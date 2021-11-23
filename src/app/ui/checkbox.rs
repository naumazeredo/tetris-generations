use crate::app::App;
use super::*;

pub struct CheckboxState {
    pub pressed: bool,
    pub down: bool,
    pub hovering: bool,
    pub changed: bool, // == pressed
}

pub struct Checkbox<'a> {
    label: &'a str,
    disabled: bool,
}

impl<'a> Checkbox<'a> {
    pub fn new(label: &str, value: &mut bool, app: &mut App) -> CheckboxState {
        Checkbox::builder(label).build(value, app)
    }

    pub fn builder<'b: 'a>(label: &'b str) -> Self {
        Self {
            label,
            disabled: false,
        }
    }

    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            disabled,
            ..self
        }
    }

    #[inline(always)] pub fn build(
        self,
        value: &mut bool,
        app: &mut App
    ) -> CheckboxState {
        self.build_with_placer(value, &mut app.ui_system.top_ui().index(), app)
    }

    #[inline(always)] pub fn build_with_placer<P: Placer>(
        self,
        value: &mut bool,
        placer: &mut P,
        app: &mut App
    ) -> CheckboxState {
        if let Some(state) = checkbox_internal(value, self, placer, app) {
            CheckboxState {
                pressed:  state.pressed,
                down:     state.down,
                hovering: state.hovering,
                changed:  state.pressed,
            }
        } else {
            CheckboxState {
                pressed:  false,
                down:     false,
                hovering: false,
                changed:  false,
            }
        }
    }
}

// -------------

fn new_checkbox(value: bool, disabled: bool) -> State {
    State {
        disabled,
        pressed:  false,
        down:     false,
        hovering: false,
        scroll:   0,
        variant: ElementVariant::Checkbox { value },
    }
}

fn checkbox_internal<'a, P: Placer>(
    value: &mut bool,
    checkbox: Checkbox,
    placer: &mut P,
    app: &'a mut App,
) -> Option<&'a State> {
    let ui = placer.ui(app);
    let spacing = ui.style.spacing;
    let line_padding = ui.style.line_padding;

    let checkbox_box_size = ui.line_draw_height();

    placer.add_padding(line_padding, app);

    let text_width = placer.draw_width(app) - checkbox_box_size - spacing;

    // Add label
    text_internal(
        Text::builder(checkbox.label)
            .disabled(checkbox.disabled)
            .max_width(text_width as u32),
        placer,
        app
    );

    placer.same_line(app);
    placer.add_spacing(app);

    // Update/create box state
    let id = Id::new(checkbox.label).add("#__checkbox");

    let size = Vec2i { x: checkbox_box_size as i32, y: checkbox_box_size as i32 };
    let layout = placer.place_element(id, size, app);

    placer.remove_padding(app);

    if layout.is_none() { return None; }
    let layout = layout.unwrap();

    app.ui_system.states.entry(id)
        .and_modify(|state| {
            state.disabled = checkbox.disabled;

            if let ElementVariant::Checkbox { value: v } = &mut state.variant {
                *v = *value;
            } else {
                unreachable!();
            }
        })
        .or_insert_with(|| new_checkbox(*value, checkbox.disabled));

    if !checkbox.disabled {
        let state = app.update_state_interaction(id, layout);
        if state.pressed {
            *value = !*value;

            // @XXX should we update the saved state or just be delayed by 1 frame?
            match &mut state.variant {
                ElementVariant::Checkbox { value: v } => *v = !*v,
                _ => unreachable!()
            }
        }

        Some(state)
    } else {
        Some(app.ui_system.states.get(&id).unwrap())
    }
}
