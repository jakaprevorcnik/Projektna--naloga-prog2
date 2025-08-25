pub mod player;
mod meteors;
pub mod events;
mod systems;
mod gameover; 
mod mainmenu;
pub mod resources;


use bevy::prelude::*;
use mainmenu::MainMenuPlugin;
use meteors::MeteorPlugin;
use player::PlayerPlugin;
use gameover::GameOverPlugin;
use events::GameOver;
use resources::GameTime;
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
    .init_resource::<GameTime>()
    .add_event::<GameOver>()
    .add_plugins(PlayerPlugin)
    .add_plugins(MeteorPlugin)
    .add_plugins((GameOverPlugin, MainMenuPlugin))
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, (toggle_states, handle_game_over))
    //neurejeno
    .add_systems(OnEnter(AppState::Game), (display_score_game_text, reset_game_time))
    .add_systems(Update, update_game_time.run_if(in_state(AppState::Game)))
    .add_systems(OnExit(AppState::Game), despawn_score_game_text)
    .run();
}


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}