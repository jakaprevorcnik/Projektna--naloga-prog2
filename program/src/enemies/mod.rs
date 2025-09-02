use bevy::prelude::*;

pub mod meteors;
pub mod enemyships;

pub mod systems;
pub mod components;

use systems::*;
use crate::AppState;

use meteors::*;
use enemyships::*;
use meteors::resources::MeteorSpawnTimer;
use enemyships::resources::EnemyShipSpawnTimers;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MeteorSpawnTimer>();
        app.init_resource::<EnemyShipSpawnTimers>();
        app.add_plugins(MeteorPlugin);
        app.add_plugins(EnemyShipPlugin);
        app.add_systems(Update, check_collision_player_enemy
            .run_if(in_state(AppState::Game)));
    }
}