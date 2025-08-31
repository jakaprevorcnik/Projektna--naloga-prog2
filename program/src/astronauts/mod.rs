use bevy::prelude::*;

mod systems;
pub mod components;
mod resources;

use resources::AstronautSpawnTimer;
use systems::*;
use crate::AppState;

pub struct AstronautPlugin;

impl Plugin for AstronautPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AstronautSpawnTimer>();
        app.add_systems(Update, (
            tick_astronaut_spawn_timer,
            spawn_astronauts_over_time,
            astronaut_movement,
            check_astronaut_collection,
            astronaut_despawn
        ).run_if(in_state(AppState::Game)));
        app.add_systems(OnEnter(AppState::GameOver), despawn_all_astronauts);
    }
}