use bevy::prelude::*;

mod systems;
pub mod components;

use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);

        app.add_systems(Update, (
            player_movement,
            confine_player_movement

        ));
    }

}