use std::thread::sleep;
use std::time::Duration;

use sdl2::TimerSubsystem;

use crate::constants::FRAME_TARGET_TIME;

pub struct Timer {
    fps: u8,
    interval: f64,
    last_tick: u32,
    last_second: u32,
    timer_subsystem: TimerSubsystem,
}

impl Timer {
    pub fn new(timer_subsystem: TimerSubsystem) -> Timer {
        let now = timer_subsystem.ticks();

        Timer {
            fps: 0,
            interval: FRAME_TARGET_TIME,
            last_tick: now,
            last_second: now,
            timer_subsystem: timer_subsystem,
        }
    }

    pub fn tick(&mut self) -> (bool, f32) {
        let now = self.timer_subsystem.ticks();
        let mut delta_time = (now - self.last_tick) as f32 / 1000.0;

        if now < self.last_tick + self.interval as u32 {
            sleep(Duration::from_secs_f32(delta_time));
            return (false, 0.0);
        }

        self.last_tick = now;
        self.fps += 1;
        if now - self.last_second > 1000 {
            println!("FPS: {}", self.fps);
            self.fps = 0;
            self.last_second = now;
        }

        if delta_time > 0.05 {
            delta_time = 0.05
        }
        return (true, delta_time);
    }
}
