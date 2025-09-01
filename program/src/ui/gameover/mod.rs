use bevy::prelude::*;

pub mod systems;
pub mod components;

use systems::*;
use crate::AppState;

use super::resources::{VisibleTextTimer, HiddenTextTimer};
use super::systems::{tick_vh_text_timers, ui_text_toggle_visibility};


pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VisibleTextTimer>();
        app.init_resource::<HiddenTextTimer>();
        app.add_systems(OnEnter(AppState::GameOver), (display_gameover_text, display_score_gameover_text));
        app.add_systems(Update, (tick_vh_text_timers, ui_text_toggle_visibility)
            .run_if(in_state(AppState::GameOver)));
        app.add_systems(OnExit(AppState::GameOver), despawn_gameover_text);
    }
}