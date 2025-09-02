use bevy::prelude::*;

const ENEMYSHIP_MIN_SPAWN_TIME: f32 = 5.0;
const ENEMYSHIP_MAX_SPAWN_TIME: f32 = 15.0;
const ENEMYSHIP_UPDATE_SPAWN_SCORE: f32 = 0.1;

#[derive(Resource)]
pub struct EnemyShipSpawnTimers {
    pub min_timer: Timer,
    pub max_timer: Timer,
    pub update_timer: Timer,
    pub spawn_score: f32
}

impl Default for EnemyShipSpawnTimers {
    fn default() -> Self {
        EnemyShipSpawnTimers { 
            min_timer: Timer::from_seconds(ENEMYSHIP_MIN_SPAWN_TIME, TimerMode::Once),
            max_timer: Timer::from_seconds(ENEMYSHIP_MAX_SPAWN_TIME, TimerMode::Repeating),
            update_timer: Timer::from_seconds(ENEMYSHIP_UPDATE_SPAWN_SCORE, TimerMode::Repeating),
            spawn_score: 0.0 
        }
    }
}

impl EnemyShipSpawnTimers {
    pub fn reset_spawn_score(&mut self) {
        self.spawn_score = 0.0;
    }

    pub fn update_spawn_score(&mut self, add_score: f32) {
        self.spawn_score += add_score;
    }
}

//Zaenkrat, da vidim, ali delajo druge stvari. Potem bi nekako z GameTime-om naredila, da se malo vedno pogosteje spawnajo-
// Pa malo mora bit random, pa za začetek je prepogosto.