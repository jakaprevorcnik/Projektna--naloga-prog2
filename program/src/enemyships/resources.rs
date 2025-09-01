use bevy::prelude::*;

pub const ENEMYSHIP_SPAWN_TIME: f32 = 8.0;

#[derive(Resource)]
pub struct EnemyShipSpawnTimer {
    pub timer: Timer,
}

impl Default for EnemyShipSpawnTimer {
    fn default() -> Self {
        EnemyShipSpawnTimer { 
            timer: Timer::from_seconds(ENEMYSHIP_SPAWN_TIME, TimerMode::Repeating) 
        }
    }
}

//Zaenkrat, da vidim, ali delajo druge stvari. Potem bi nekako z GameTime-om naredila, da se malo vedno pogosteje spawnajo-
// Pa malo mora bit random, pa za začetek je prepogosto.