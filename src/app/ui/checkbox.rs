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

    pub fn build(self, value: &mut bool, app: &mut App) -> CheckboxState {
        let state = app.checkbox_internal(value, self);

        CheckboxState {
            pressed: state.pressed,
            down:    state.down,
            hovering: state.hovering,
            changed: state.pressed,
        }
    }
}

// -------------

fn new_checkbox(value: bool, disabled: bool) -> State {
    State {
        pressed: false,
        down: false,
        hovering: false,
        disabled,
        variant: ElementVariant::Checkbox { value },
    }
}

impl App<'_> {
    fn checkbox_internal(&mut self, value: &mut bool, checkbox: Checkbox) -> &State {
        // Add label
        self.text_internal(Text::builder(checkbox.label).disabled(checkbox.disabled));
        self.same_line();

        // Update/create box state
        let id = Id::new(checkbox.label).add("#__checkbox");

        let ui = &self.ui_system.uis.last().unwrap();
        let checkbox_box_size = ui.style.line_height;

        let size = Vec2i { x: checkbox_box_size as i32, y: checkbox_box_size as i32 };
        let layout = self.new_layout_right(size);
        self.add_element(id, layout);

        self.ui_system.states.entry(id)
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
            let state = self.update_state_interaction(id, layout);
            if state.pressed {
                *value = !*value;

                // @XXX should we update the saved state or just be delayed by 1 frame?
                match &mut state.variant {
                    ElementVariant::Checkbox { value: v } => *v = !*v,
                    _ => unreachable!()
                }
            }

            state
        } else {
            self.ui_system.states.get(&id).unwrap()
        }
    }
}
