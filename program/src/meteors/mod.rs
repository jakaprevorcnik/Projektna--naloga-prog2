use bevy::prelude::*;

mod systems;
pub mod components;

use systems::*;

pub struct MeteorPlugin;
 impl Plugin for MeteorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_meteor,);
    }
 }
