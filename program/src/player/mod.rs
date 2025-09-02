use bevy::prelude::*;

pub mod systems;
pub mod components;

use systems::*;
use crate::AppState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Kako se to naredi prav?
        app.add_systems(OnEnter(AppState::Game), spawn_player);

        app.add_systems(Update, (
            player_movement,
            confine_player_movement
        ));
        app.add_systems(Update, (
            // check_collision_meteor_player,
            shoot_bullet,
            bullet_movement,
            bullet_bigenemy_collision_system,
            bullets_despawn
        ).run_if(in_state(AppState::Game)));
        app.add_systems(OnEnter(AppState::GameOver), despawn_all_bullets);
    }
}