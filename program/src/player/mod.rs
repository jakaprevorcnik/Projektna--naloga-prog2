use bevy::prelude::*;

pub mod systems;
pub mod components;
mod resources;

use systems::*;
use resources::BulletShootTimer;
use crate::AppState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BulletShootTimer>();

        app.add_systems(OnEnter(AppState::Game), spawn_player);

        app.add_systems(Update, (
            player_movement,
            confine_player_movement
        ));
        app.add_systems(Update, (
            tick_bullet_timer,
            shoot_bullet,
            bullet_movement,
            bullet_bigenemy_collision_system,
            bullets_despawn
        ).run_if(in_state(AppState::Game)));
        
        app.add_systems(OnEnter(AppState::GameOver), despawn_all_bullets);
    }
}