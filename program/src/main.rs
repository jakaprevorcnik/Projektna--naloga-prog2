pub mod player;
mod meteors;
pub mod events;
mod systems;
mod gameover;


use bevy::prelude::*;
use meteors::MeteorPlugin;
use player::PlayerPlugin;
use gameover::GameOverPlugin;
use events::GameOver;
use crate::systems::*;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins
        .set(WindowPlugin{
            primary_window: Some(Window{
                title: String::from("Space Rangers"),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resolution: Vec2::new(1024., 800.).into(),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
    .init_state::<AppState>()
    .add_event::<GameOver>()
    .add_plugins(PlayerPlugin)
    .add_plugins(MeteorPlugin)
    .add_plugins(GameOverPlugin)
    .add_systems(Startup, spawn_camera)
    // Dodala 16. 5., ni urejeno ...
    .add_systems(Update, handle_game_over)
    .run();
}


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    // MainMenu, // trenutno še ne obstaja, drugače bo to default.
    Game,
    GameOver,
}