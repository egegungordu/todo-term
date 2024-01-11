pub struct ActionDisplay {
    action: String,
    tick_count: u32,
    reset_tick_count: u32,
}

impl Default for ActionDisplay {
    fn default() -> Self {
        Self {
            action: String::new(),
            tick_count: 0,
            reset_tick_count: 20,
        }
    }
}

impl ActionDisplay {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, action: &str) {
        self.tick_count = 0;
        self.action = action.to_string();
    }

    pub fn get(&self) -> &str {
        &self.action
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        if self.tick_count >= self.reset_tick_count {
            self.action.clear();
            self.tick_count = 0;
        }
    }
}
