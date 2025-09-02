use bevy::prelude::*;

mod systems;
pub mod components;

use systems::*;
use crate::AppState;

pub struct EnemyBulletPlugin;
 impl Plugin for EnemyBulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,
            (tick_enemybullet_timer, shoot_enemy_bullet, enemybullet_movement, enemybullets_despawn)
            .run_if(in_state(AppState::Game)));
        app.add_systems(OnEnter(AppState::GameOver), despawn_all_enemybullets);
    }
 }
