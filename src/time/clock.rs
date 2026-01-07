use crate::core::Rules;

pub struct Clock {
    accumulator: f64,
    interval: f64,
}

impl Clock {
    pub fn new(level: u32) -> Self {
        Self {
            accumulator: 0.0,
            interval: Rules::gravity_interval(level),
        }
    }

    pub fn set_level(&mut self, level: u32) {
        self.interval = Rules::gravity_interval(level);
    }

    /// Returns true when a tick should occur
    pub fn tick(&mut self, dt: f64) -> bool {
        self.accumulator += dt;
        if self.accumulator >= self.interval {
            self.accumulator -= self.interval;
            true
        } else {
            false
        }
    }
}
