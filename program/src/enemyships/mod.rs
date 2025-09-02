use bevy::prelude::*;

mod systems;
pub mod components;
mod resources;

use resources::EnemyShipSpawnTimer;
use systems::*;
use crate::AppState;

pub struct EnemyShipPlugin;
 impl Plugin for EnemyShipPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyShipSpawnTimer>();
        app.add_systems(Update, 
            (tick_enemyship_spawn_timer, spawn_enemyships_over_time, enemyship_movement, enemyship_despawn, confine_enemyship_movement)
            .run_if(in_state(AppState::Game)));
        app.add_systems(Update,
            (tick_enemybullet_timer, shoot_enemy_bullet, enemybullet_movement, enemybullets_despawn)
            .run_if(in_state(AppState::Game)));
        app.add_systems(OnEnter(AppState::GameOver), (despawn_all_enemyships, despawn_all_enemybullets));
    }
 }
