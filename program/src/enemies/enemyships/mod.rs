use bevy::prelude::*;

mod systems;
pub mod components;
pub mod resources;
pub mod enemybullets;

use resources::EnemyShipSpawnTimers;
use systems::*;
use crate::AppState;
use enemybullets::EnemyBulletPlugin;

pub struct EnemyShipPlugin;
 impl Plugin for EnemyShipPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyShipSpawnTimers>();
        app.add_systems(Update, 
            (tick_enemyship_spawn_timer, spawn_enemyships_over_time, enemyship_movement, enemyship_despawn, confine_enemyship_movement)
            .run_if(in_state(AppState::Game)));
        app.add_systems(OnEnter(AppState::Game), reset_enemyship_spawn_timers);
        app.add_systems(OnEnter(AppState::GameOver), despawn_all_enemyships);
        app.add_plugins(EnemyBulletPlugin);
    }
 }
