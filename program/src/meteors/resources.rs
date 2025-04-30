use bevy::prelude::*;

pub const METEOR_SPAWN_TIME: f32 = 3.0;

#[derive(Resource)]
pub struct MeteorSpawnTimer {
    pub timer: Timer,
}

impl Default for MeteorSpawnTimer {
    fn default() -> Self {
        MeteorSpawnTimer { 
            timer: Timer::from_seconds(METEOR_SPAWN_TIME, TimerMode::Repeating) 
        }
    }
}