pub mod player;
mod astronauts;
pub mod events;
mod systems;
mod ui;
mod enemies;


use bevy::prelude::*;
use astronauts::AstronautPlugin;
use player::PlayerPlugin;
use enemies::EnemyPlugin;
use ui::UIPlugin;
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
    .add_plugins(UIPlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(AstronautPlugin)
    .add_plugins(EnemyPlugin)
    .add_systems(Startup, (spawn_camera, spawn_background))
    .add_systems(Update, (toggle_states, handle_game_over))
    .run();
}


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}