use bevy::prelude::*;

mod systems;
pub mod components;
pub mod resources;

use resources::MeteorSpawnTimer;
use systems::*;
use crate::AppState;

pub struct MeteorPlugin;
 impl Plugin for MeteorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MeteorSpawnTimer>();
        app.add_systems(Update, 
            (tick_meteor_spawn_timer, spawn_meteors_over_time, meteor_movement, meteor_despawn)
            .run_if(in_state(AppState::Game)));
        app.add_systems(OnEnter(AppState::GameOver), despawn_all_meteors);
    }
 }
