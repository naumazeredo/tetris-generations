// Time System

// [x] rename to TimeSystem
// [ ] use std::time instead of SDL timer_subsystem

use crate::app::{ App, ImDraw };

#[derive(ImDraw)]
pub(in crate::app) struct TimeSystem {
    pub(in crate::app) frame_count: u32,
    pub(in crate::app) real_time: u64, // @Rename not actually the real time!
    pub(in crate::app) real_frame_duration: u64,
    pub(in crate::app) game_time: u64,
    pub(in crate::app) game_frame_duration: u64,
    pub(in crate::app) scale: f64,
    pub(in crate::app) frame_start_time: u64,

    current_time: u64,
    last_scale: f64,
}

impl TimeSystem {
    pub(in crate::app) fn new(timer_subsystem: sdl2::TimerSubsystem) -> Self {
        Self {
            frame_count: 0,
            real_time: 0,
            real_frame_duration: 0,
            game_time: 0,
            game_frame_duration: 0,
            scale: 1.0,
            frame_start_time: 0,
            current_time: system_time(&timer_subsystem),
            last_scale: 1.0,
        }
    }
}

impl App<'_> {
    pub fn new_frame(&mut self) {
        let time_system = &mut self.time_system;

        time_system.frame_count += 1;
        time_system.frame_start_time = time_system.real_time;

        let current_time = system_time(&self.sdl_context.timer_subsystem);
        time_system.real_frame_duration = current_time - time_system.current_time;
        time_system.current_time = current_time;
    }

    pub(in crate::app) fn advance_time(&mut self, real_time_delta: u64) {
        let time_system = &mut self.time_system;

        time_system.real_time += time_system.real_frame_duration;

        time_system.game_frame_duration = (time_system.scale * (real_time_delta as f64)) as u64;
        time_system.game_time += time_system.game_frame_duration;
    }

    pub fn is_paused(&self) -> bool {
        self.time_system.scale == 0.0
    }

    pub fn pause(&mut self) {
        if self.is_paused() { return; }

        let time_system = &mut self.time_system;
        time_system.last_scale = time_system.scale;
        time_system.scale = 0.0;
    }

    pub fn set_time_scale(&mut self, scale: f64) {
        assert!(scale >= 0.0 && scale <= 1.0);
        self.time_system.scale = scale;
    }

    pub fn resume(&mut self) {
        if !self.is_paused() { return; }

        let time_system = &mut self.time_system;
        time_system.scale = time_system.last_scale;
    }

    /*
    pub fn last_frame_duration(&self) -> f32 {
        to_seconds(self.time_system.game_frame_duration)
    }
    */

    // @TODO remove this. User should either use dt or real duration
    pub fn last_frame_duration(&self) -> u64 {
        self.time_system.game_frame_duration
    }

    pub fn last_frame_real_duration(&self) -> u64 {
        self.time_system.real_frame_duration
    }

    pub fn game_time(&self) -> f32 {
        to_seconds(self.time_system.game_time)
    }

    pub fn game_timestamp(&self) -> u64 {
        self.time_system.game_time
    }

    pub fn set_game_timestamp(&mut self, new_timestamp: u64) {
        self.time_system.game_time = new_timestamp;
    }

    pub fn real_timestamp(&self) -> u64 {
        self.time_system.real_time
    }

    pub fn restart_time_system(&mut self) {
        self.time_system = TimeSystem::new(self.sdl_context.timer_subsystem.clone());
    }

    pub fn frame(&self) -> u32 {
        self.time_system.frame_count
    }

    pub fn system_time(&self) -> u64 {
        system_time(&self.sdl_context.timer_subsystem)
    }
}

fn system_time(timer_subsystem: &sdl2::TimerSubsystem) -> u64 {
    let counter = timer_subsystem.performance_counter() as u128;
    let frequency = timer_subsystem.performance_frequency() as u128;

    (counter * 1_000_000 / frequency) as u64
}

// @Refactor use types for time/duration
pub fn to_seconds(usecs: u64) -> f32 {
    usecs as f32 / 1_000_000.
}
