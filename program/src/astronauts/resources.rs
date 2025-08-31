use bevy::prelude::*;

pub const ASTRONAUT_SPAWN_TIME: f32 = 5.0;

#[derive(Resource)]
pub struct AstronautSpawnTimer {
    pub timer: Timer,
}

impl Default for AstronautSpawnTimer {
    fn default() -> Self {
        AstronautSpawnTimer { 
            timer: Timer::from_seconds(ASTRONAUT_SPAWN_TIME, TimerMode::Repeating) 
        }
    }
}
