use bevy::prelude::*;

pub mod gameover;
pub mod mainmenu;
pub mod score;

pub mod systems;
pub mod components;
pub mod resources;

use resources::*;
use gameover::*;
use mainmenu::*;
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
    }
}