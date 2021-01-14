// @Refactor maybe use std::time?
// @Refactor create a type to hold the USec, MSec, Sec (different types to be type checked)
//extern crate sdl2;

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

    pub fn new_frame(&mut self, timer_subsystem: &sdl2::TimerSubsystem) {
        self.frame_count += 1;

        let current_time = get_current_time(&timer_subsystem);
        self.real_frame_duration = current_time - self.last_time;
        self.real_time += self.real_frame_duration;
        self.last_time = current_time;

        self.game_frame_duration =
            (self.scale * (self.real_frame_duration as f64)) as u64;
        self.game_time += self.game_frame_duration;
    }

    #[allow(dead_code)]
    pub fn pause(&mut self) {
        self.last_scale = self.scale;
        self.scale = 0.0;
    }

    #[allow(dead_code)]
    pub fn resume(&mut self) {
        self.scale = self.last_scale;
    }
}

fn get_current_time(timer_subsystem: &sdl2::TimerSubsystem) -> u64 {
    let counter = timer_subsystem.performance_counter();
    let frequency = timer_subsystem.performance_frequency();

    counter * 1_000_000 / frequency
}
