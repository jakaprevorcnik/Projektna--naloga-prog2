use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const PLAYER_SPEED: f32 = 500.0;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins
        // .set(WindowPlugin{
        //     primary_window: Some(Window{
        //         title: String::from("Space Rangers"),
        //         position: WindowPosition::Centered(MonitorSelection::Primary),
        //         resolution: Vec2::new(512., 512.).into(),
        //         resizable: false,
        //         ..Default::default()
        //     }),
        //     ..Default::default()
        // }))
    )
    .add_systems(Startup, (
        spawn_camera,
        spawn_player,
        spawn_meteor,
    ))
    .add_systems(Update, (
        player_movement,
        confine_player_movement
    ))
    .run();
}


#[derive(Component)]
pub struct Player {}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
  }

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            Sprite{
                image: asset_server.load("sprites/spaceShips_008.png"),
                ..Default::default()
            },
            Transform::from_xyz(0.0, - (window.height() / 2.5), 0.0).with_scale(Vec3::splat(0.5)),
            Player {},
        )
    );
}

#[derive(Component)]
struct Meteor {}

fn spawn_meteor(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            Sprite{
                image: asset_server.load("sprites/spaceMeteors_001.png"),
                ..Default::default()
            },
            Transform::from_xyz(0.0, window.height() / 2.5, 0.0),
            Meteor {},
        )
    );
}

fn player_movement(keyboard_input: 
    Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let x_min = 0.0 - window.width() / 4.0;
        let x_max = 0.0 + window.width() / 4.0;
        let _y_min = 0.0; //ni vazno ker se premika samo levo in desno
        let _y_max = 0.0;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the players y position.
        // if translation.y < y_min {
        //     translation.y = y_min;
        // } else if translation.y > y_max {
        //     translation.y = y_max;
        // }

        player_transform.translation = translation;
    }
}