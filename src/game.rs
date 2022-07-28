use std::{time::{Instant, Duration}, iter::Inspect};

use winit_input_helper::WinitInputHelper;

mod player;

pub(crate) const WORLD_MAP: [[u8; 8]; 8] = 
[
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1]
];

#[derive(Debug)]
pub struct Game {
    last_time: Instant,
    delta_time: Duration,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn draw(&self, pixel_buffer: &mut [u8]) {

    }

    pub fn simulate_logic(&mut self, now: Instant, input: &mut WinitInputHelper) {

    }

    pub fn is_paused(&self) -> bool {
        false
    }
}

impl Default for Game {
    fn default() -> Self {
        Self { last_time: Instant::now(), delta_time: Duration::default() }
    }
}