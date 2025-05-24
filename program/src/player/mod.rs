use bevy::prelude::*;

mod systems;
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
            check_collsion_meteor_player_rough,
            shoot_bullet,
            bullet_movement,
            bullet_meteor_collision_system
        ).run_if(in_state(AppState::Game)));
        app.add_systems(OnEnter(AppState::GameOver), despawn_all_bullets);
    }
}