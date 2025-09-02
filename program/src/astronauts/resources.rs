use bevy::prelude::*;

pub const ASTRONAUT_SPAWN_TIME: f32 = 5.0;

#[derive(Resource)]
pub struct AstronautSpawnTimer {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct AstronautsMissedCounter {
    pub counter: u32
}


impl Default for AstronautSpawnTimer {
    fn default() -> Self {
        AstronautSpawnTimer { 
            timer: Timer::from_seconds(ASTRONAUT_SPAWN_TIME, TimerMode::Repeating) 
        }
    }
}


impl Default for AstronautsMissedCounter {
    fn default() -> Self {
        AstronautsMissedCounter { counter: 0 }
    }
}

impl AstronautsMissedCounter {
    pub fn missed(&mut self) {
        self.counter += 1;
    }

    pub fn reset(&mut self) {
        self.counter = 0;
    }
}