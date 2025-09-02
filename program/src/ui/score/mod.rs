use bevy::prelude::*;

mod systems;
pub mod resources;
mod components;

use crate::AppState;
use crate::ui::score::resources::*;
use crate::ui::score::systems::*;
use crate::ui::systems::display_astronauts_game_text;

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameTime>();
        app.init_resource::<Score>();
        app.init_resource::<HighScore>();
        app.add_systems(OnEnter(AppState::Game), 
            (display_score_game_text, reset_game_time, reset_score, display_astronauts_game_text));
        app.add_systems(Update, 
            (update_game_time, update_score_with_time, update_score_display)
            .run_if(in_state(AppState::Game)));
        app.add_systems(OnExit(AppState::Game), despawn_score_game_text);
    }
}