pub mod player;
mod meteors;



use bevy::prelude::*;
use meteors::MeteorPlugin;
use player::PlayerPlugin;

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
    //)
    .add_plugins(PlayerPlugin)
    .add_plugins(MeteorPlugin)
    .add_systems(Startup, spawn_camera)
    .run();
}




fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
  }



