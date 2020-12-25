extern crate sdl2;

use super::engine::Engine;

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
            last_time: get_current_time(timer_subsystem),
            last_scale: 1.0,
        }
    }
}

pub fn new_frame(engine: &mut Engine) {
    engine.time.frame_count += 1;

    let current_time = get_current_time(&engine.timer_subsystem);
    engine.time.real_frame_duration = current_time - engine.time.last_time;
    engine.time.real_time += engine.time.real_frame_duration;
    engine.time.last_time = current_time;

    engine.time.game_frame_duration =
        (engine.time.scale * (engine.time.real_frame_duration as f64)) as u64;
    engine.time.game_time += engine.time.game_frame_duration;
}

pub fn pause(engine: &mut Engine) {
    engine.time.last_scale = engine.time.scale;
    engine.time.scale = 0.0;
}

pub fn resume(engine: &mut Engine) {
    engine.time.scale = engine.time.last_scale;
}

fn get_current_time(timer_subsystem: &sdl2::TimerSubsystem) -> u64 {
    let counter = timer_subsystem.performance_counter();
    let frequency = timer_subsystem.performance_frequency();

    counter * 1_000_000 / frequency
}
