pub struct Animator {
    lock_pop_timer: f64,
    line_clear_timer: f64,
}

impl Animator {
    pub fn new() -> Self {
        Self {
            lock_pop_timer: 0.0,
            line_clear_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        if self.lock_pop_timer > 0.0 {
            self.lock_pop_timer -= dt;
        }
        if self.line_clear_timer > 0.0 {
            self.line_clear_timer -= dt;
        }
    }

    pub fn trigger_lock_pop(&mut self) {
        self.lock_pop_timer = 0.12; // 120ms
    }

    pub fn trigger_line_clear(&mut self) {
        self.line_clear_timer = 0.2;
    }

    /// Returns scale factor for lock pop animation (1.0 to 1.15 and back)
    pub fn lock_pop_scale(&self) -> f32 {
        if self.lock_pop_timer > 0.0 {
            let t = (self.lock_pop_timer / 0.12) as f32;
            1.0 + 0.15 * (t * std::f32::consts::PI).sin()
        } else {
            1.0
        }
    }

    #[allow(dead_code)]
    pub fn line_clear_alpha(&self) -> f32 {
        if self.line_clear_timer > 0.0 {
            (self.line_clear_timer / 0.2) as f32
        } else {
            1.0
        }
    }
}
