// Time System

// [ ] rename to TimeSystem
// [ ] maybe use std::time instead of SDL timer_subsystem
// [ ] create newtype for duration intervals
// [ ] clone timer_subsystem instead of referencing it

// @Refactor maybe use std::time?
// @Refactor create a type to hold the USec, MSec, Sec (different types to be type checked)

use super::{App, GameState};

// @Rename Time -> TimeSystem
pub struct Time {
    pub frame_count: u32,
    pub real_time: u64,
    pub real_frame_duration: u64,
    pub game_time: u64,
    pub game_frame_duration: u64,
    pub scale: f64,

    last_time: u64,
    last_scale: f64,
}

impl Time {
    pub fn new(timer_subsystem: &sdl2::TimerSubsystem) -> Self {
        Self {
            frame_count: 0,
            real_time: 0,
            real_frame_duration: 0,
            game_time: 0,
            game_frame_duration: 0,
            scale: 1.0,
            last_time: system_time(timer_subsystem),
            last_scale: 1.0,
        }
    }

    pub fn frame_duration(&self) -> f32 {
        self.game_frame_duration as f32 / 1_000_000.
    }
}

fn system_time(timer_subsystem: &sdl2::TimerSubsystem) -> u64 {
    let counter = timer_subsystem.performance_counter() as u128;
    let frequency = timer_subsystem.performance_frequency() as u128;

    (counter * 1_000_000 / frequency) as u64
}

impl<S: GameState> App<'_, S> {
    pub fn new_frame(&mut self) {
        let time_system = &mut self.time;

        time_system.frame_count += 1;

        let current_time = system_time(&self.sdl_context.timer_subsystem);
        time_system.real_frame_duration = current_time - time_system.last_time;
        time_system.real_time += time_system.real_frame_duration;
        time_system.last_time = current_time;

        time_system.game_frame_duration =
            (time_system.scale * (time_system.real_frame_duration as f64)) as u64;
        time_system.game_time += time_system.game_frame_duration;
    }

    pub fn pause(&mut self) {
        let time_system = &mut self.time;
        time_system.last_scale = time_system.scale;
        time_system.scale = 0.0;
    }

    pub fn resume(&mut self) {
        let time_system = &mut self.time;
        time_system.scale = time_system.last_scale;
    }
}
