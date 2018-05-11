use time::precise_time_ns;

/// A resource for the SPECS world giving the delta time between two updates
pub struct DeltaTime {
    pub delta_ms: f32,
    pub delta_ns: u64,
    old_time: u64,
}

impl DeltaTime {
    /// Creates the DeltaTime resource with correct initialization
    pub fn new() -> Self {
        DeltaTime {
            delta_ms: 0.,
            delta_ns: 0,
            old_time: precise_time_ns(),
        }
    }

    /// Updates the DeltaTime resource
    pub fn update(&mut self) {
        let new_time: u64 = precise_time_ns();
        self.delta_ns = new_time - self.old_time;
        self.delta_ms = self.delta_ns as f32 / 1000.0;
    }
}

impl Default for DeltaTime {
    fn default() -> Self {
        DeltaTime::new()
    }
}
