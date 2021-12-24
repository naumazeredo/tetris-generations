use std::collections::BTreeMap;
use super::*;
use crate::game::{
    input::*,
    randomizer::*,
};

#[derive(Debug)]
pub struct SimulatedInputMapping {
    button_mapping: BTreeMap<String, SimulatedButton>,
}

impl SimulatedInputMapping {
    pub fn new() -> Self {
        let mut button_mapping = BTreeMap::new();

        button_mapping.insert(KEY_DOWN.to_owned(), SimulatedButton::new());
        button_mapping.insert(KEY_LEFT.to_owned(), SimulatedButton::new());
        button_mapping.insert(KEY_RIGHT.to_owned(), SimulatedButton::new());
        button_mapping.insert(KEY_ROTATE_CW.to_owned(), SimulatedButton::new());
        button_mapping.insert(KEY_ROTATE_CCW.to_owned(), SimulatedButton::new());
        button_mapping.insert(KEY_HOLD.to_owned(), SimulatedButton::new());
        button_mapping.insert(KEY_HARD_DROP.to_owned(), SimulatedButton::new());

        Self {
            button_mapping,
        }
    }

    pub fn button_mut(&mut self, name: String) -> &mut SimulatedButton {
        self.button_mapping.get_mut(&name)
            .expect(&format!("[main_menu][custom][preview] SimulatedInputMapping without button {}", name))
    }

    pub fn update(&mut self, timestamp: u64) {
        for button in self.button_mapping.values_mut() {
            button.update(timestamp);
        }
    }
}

impl InputMapping for SimulatedInputMapping {
    type ButtonType = SimulatedButton;

    fn button(&self, name: String) -> &Self::ButtonType {
        self.button_mapping.get(&name)
            .expect(&format!("[main_menu][custom][preview] SimulatedInputMapping without button {}", name))
    }

    fn update(&mut self, _app: &App) { }
}

#[derive(Debug)]
pub struct SimulatedButton {
    timestamp: u64,
    down: bool,
    pressed: bool,
    released: bool,
}

impl SimulatedButton {
    fn new() -> Self {
        Self {
            timestamp: 0,
            down:      false,
            pressed:   false,
            released:  false,
        }
    }

    pub fn press(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
        self.down = true;
        self.pressed = true;
        self.released = false;
    }

    pub fn release(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
        self.down = false;
        self.pressed = false;
        self.released = true;
    }

    pub fn update(&mut self, timestamp: u64) {
        if self.down {
            self.pressed  = timestamp == self.timestamp;
            self.released = false;
        } else {
            self.pressed  = false;
            self.released = timestamp == self.timestamp;
        }
    }
}

impl Button for SimulatedButton {
    fn timestamp(&self) -> u64 { self.timestamp }
    fn down(&self)     -> bool { self.down }
    fn pressed(&self)  -> bool { self.pressed }
    fn released(&self) -> bool { self.released }
}

#[derive(Debug)]
pub struct PlayfieldAnimation {
    pub(super) instance: RulesInstance,
    input_sequence: InputSequence,

    randomizer_start: Randomizer,
}

impl_imdraw_todo!(PlayfieldAnimation);

impl PlayfieldAnimation {
    pub fn new(
        rules: Rules,
        playfield: Playfield,
        randomizer: Randomizer,
        input_sequence: InputSequence
    ) -> Self {
        let randomizer_start = randomizer.clone();
        let instance = RulesInstance::new_preview(rules, playfield, randomizer);

        Self {
            randomizer_start,
            instance,
            input_sequence,
        }
    }

    pub fn update(&mut self, dt: u64, app: &mut App) -> bool {
        self.input_sequence.update(dt);
        let instance_updated = self.instance.update(dt, &self.input_sequence.input_mapping, app);

        if self.input_sequence.has_finished() {
            self.input_sequence.reset();

            let playfield = Playfield::new(
                self.instance.playfield().grid_size,
                self.instance.playfield().visible_height,
            );

            self.instance = RulesInstance::new_preview(
                self.instance.rules().clone(),
                playfield,
                self.randomizer_start.clone(),
            );

            true
        } else {
            instance_updated
        }
    }
}

#[derive(Debug)]
struct Event {
    timestamp: u64,
    button_name: &'static str,
    is_press: bool,
}

#[derive(Debug)]
pub struct InputSequence {
    duration: u64,
    events: Vec<Event>,

    // @Maybe these should be in another struct and InputSequence should only hold the sequence?
    input_mapping: SimulatedInputMapping,
    current_time: u64,
    next_event: u32,
}

impl InputSequence {
    pub fn reset(&mut self) {
        self.current_time  = 0;
        self.next_event = 0;
        self.input_mapping = SimulatedInputMapping::new();
    }

    pub fn update(&mut self, dt: u64) -> bool {
        self.current_time += dt;

        self.input_mapping.update(self.current_time);

        let mut has_updated = false;
        while self.next_event < self.events.len() as u32 &&
            self.events[self.next_event as usize].timestamp <= self.current_time
        {
            has_updated = true;

            let Event { button_name, is_press, .. } = self.events[self.next_event as usize];
            let button = &mut self.input_mapping.button_mut(button_name.to_owned());
            if is_press {
                button.press(self.current_time);
            } else {
                button.release(self.current_time);
            }

            self.next_event += 1;
        }

        has_updated
    }

    pub fn has_finished(&self) -> bool {
        self.current_time >= self.duration
    }
}

pub struct InputSequenceBuilder {
    current_time: u64,
    events: Vec<Event>,
}

impl InputSequenceBuilder {
    pub fn new() -> Self {
        Self {
            current_time: 0,
            events: Vec::new(),
        }
    }

    pub fn press(mut self, button_name: &'static str) -> Self {
        self.events.push(Event {
            timestamp: self.current_time,
            button_name,
            is_press: true,
        });

        self
    }

    pub fn release(mut self, button_name: &'static str) -> Self {
        self.events.push(Event {
            timestamp: self.current_time,
            button_name,
            is_press: false,
        });

        self
    }

    pub fn click(self, button_name: &'static str) -> Self {
        self.press(button_name).wait(20_000).release(button_name)
    }

    pub fn wait(mut self, duration: u64) -> Self {
        self.current_time += duration;
        self
    }

    pub fn build(self) -> InputSequence {
        InputSequence {
            duration: self.current_time,
            events:   self.events,

            input_mapping: SimulatedInputMapping::new(),
            current_time:  0,
            next_event: 0,
        }
    }
}
