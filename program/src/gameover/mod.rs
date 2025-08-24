use bevy::prelude::*;

mod systems;
mod resources;
mod components;

use systems::*;
use crate::AppState;

use resources::{VisibleTextTimer, HiddenTextTimer};

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VisibleTextTimer>();
        app.init_resource::<HiddenTextTimer>();
        app.add_systems(OnEnter(AppState::GameOver), (display_gameover_text, display_score_gameover_text));
        app.add_systems(Update, (tick_vh_text_timers, gameover_text_toggle_visibility, game_over_to_game)
            .run_if(in_state(AppState::GameOver)));
        app.add_systems(OnEnter(AppState::Game), despawn_gameover_text);
    }
}