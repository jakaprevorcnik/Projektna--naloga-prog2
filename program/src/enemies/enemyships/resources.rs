use bevy::prelude::*;

pub const ENEMYSHIP_MIN_SPAWN_TIME: f32 = 5.0;
pub const ENEMYSHIP_MAX_SPAWN_TIME: f32 = 15.0;

#[derive(Resource)]
pub struct EnemyShipSpawnTimer {
    pub timer: Timer,
}

impl Default for EnemyShipSpawnTimer {
    fn default() -> Self {
        EnemyShipSpawnTimer { 
            timer: Timer::from_seconds(ENEMYSHIP_MAX_SPAWN_TIME, TimerMode::Once),
        }
    }
}

impl EnemyShipSpawnTimer {
    pub fn to_default(&mut self) {
        self.timer = Timer::from_seconds(ENEMYSHIP_MAX_SPAWN_TIME, TimerMode::Once);
    }

    pub fn set_new_timer(&mut self, time: f32) {
        self.timer = Timer::from_seconds(time, TimerMode::Once);
    }
}