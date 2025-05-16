pub mod player;
mod meteors;
pub mod events;
mod systems;


use bevy::prelude::*;
use meteors::MeteorPlugin;
use player::PlayerPlugin;
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
    .add_systems(Startup, spawn_camera)
    // Dodala 16. 5., ni urejeno ...
    .add_systems(Update, handle_game_over)
    .add_systems(Update, game_over_to_game)
    .add_systems(Update, display_gameover_text.run_if(in_state(AppState::GameOver)))
    .add_systems(OnEnter(AppState::Game), despawn_gameover_text)
    .run();
}


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    // MainMenu, // trenutno še ne obstaja, drugače bo to default.
    Game,
    GameOver,
}