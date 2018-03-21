use std::time::SystemTime;

pub struct Game {

}

impl Game {
    pub fn new() -> Self {
        Game {}
    }

    pub fn update(&mut self, time: SystemTime) {
        for _ in 0..1_000_00 {}
    }
}