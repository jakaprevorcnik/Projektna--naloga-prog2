use bevy::prelude::*;

pub mod gameover;
pub mod mainmenu;
pub mod score;

pub mod systems;
pub mod components;
pub mod resources;

use resources::*;
use systems::*;
use gameover::*;
use mainmenu::*;
use crate::AppState;
use score::resources::*;
use score::ScorePlugin;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VisibleTextTimer>();
        app.init_resource::<HiddenTextTimer>();
        app.init_resource::<GameTime>();
        app.init_resource::<Score>();
        app.init_resource::<HighScore>();
        app.add_plugins(MainMenuPlugin);
        app.add_plugins(GameOverPlugin);
        app.add_plugins(ScorePlugin);
        app.add_systems(OnEnter(AppState::Game), display_astronauts_game_text);
        app.add_systems(Update, read_and_display_astronauts_missed.run_if(in_state(AppState::Game)));
    }
}