use bevy::prelude::*;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins
        // .set(WindowPlugin{
        //     primary_window: Some(Window{
        //         title: String::from("Space Rangers"),
        //         position: WindowPosition::Centered(MonitorSelection::Primary),
        //         resolution: Vec2::new(512., 512.).into(),
        //         ..Default::default()
        //     }),
        //     ..Default::default()
        // }))
    )
    .add_systems(Startup, (spawn_camera, spawn_player))
    .run();
}


#[derive(Component)]
struct Player {}

fn spawn_camera(mut commands: Commands) {
    commands
      .spawn_empty()
      .insert(Camera2d);
  }

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(
        (
            Sprite{
                image: asset_server.load("sprites/spaceShips_008.png"),
                ..default()
            },
            // Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            // SpriteBundle{
            //     transform: Transform::from_xyz(window.width() / 2.0, window_height() / 2.0, 0.0),
            //     sprite: asset_server.load("sprites/spaceShips_008.png"),
            //     ..default()
            // },
            Player {},
        )
    );
}