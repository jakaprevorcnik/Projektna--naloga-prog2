use bevy::prelude::*;

pub const ENEMYSHIP_MIN_SPAWN_TIME: f32 = 5.0;
pub const ENEMYSHIP_MAX_SPAWN_TIME: f32 = 15.0;

#[derive(Resource)]
pub struct EnemyShipSpawnTimers {
    pub timer: Timer,
}

impl Default for EnemyShipSpawnTimers {
    fn default() -> Self {
        EnemyShipSpawnTimers { 
            timer: Timer::from_seconds(ENEMYSHIP_MAX_SPAWN_TIME, TimerMode::Once),
        }
    }
}

impl EnemyShipSpawnTimers {
    pub fn to_default(&mut self) {
        self.timer = Timer::from_seconds(ENEMYSHIP_MAX_SPAWN_TIME, TimerMode::Once);
    }

    pub fn set_new_timer(&mut self, time: f32) {
        self.timer = Timer::from_seconds(time, TimerMode::Once);
    }
}

//Zaenkrat, da vidim, ali delajo druge stvari. Potem bi nekako z GameTime-om naredila, da se malo vedno pogosteje spawnajo-
// Pa malo mora bit random, pa za začetek je prepogosto.