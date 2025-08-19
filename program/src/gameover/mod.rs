use bevy::prelude::*;

mod systems;
mod resources;
mod components;

use systems::*;
use crate::AppState;

use resources::ToggleTextTimer;

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ToggleTextTimer>();
        app.add_systems(OnEnter(AppState::GameOver), display_gameover_text);
        app.add_systems(Update, (tick_toggle_text_timer, gameover_text_toggle_visibility, game_over_to_game)
            .run_if(in_state(AppState::GameOver)));
        app.add_systems(OnEnter(AppState::Game), despawn_gameover_text);
    }
}