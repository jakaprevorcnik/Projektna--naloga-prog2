use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::meteors::components::*;

pub fn spawn_meteor(
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