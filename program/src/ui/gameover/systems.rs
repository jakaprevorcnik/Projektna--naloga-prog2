use bevy::prelude::*;

use crate::ui::gameover::components::*;
use crate::ui::components::BlinkingText;
use crate::ui::score::resources::{Score, HighScore};


pub fn display_gameover_text(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Text2d::new("GAME OVER"),
        TextFont {
            font: asset_server.load("fonts/Pixellettersfull-BnJ5.ttf"),
            font_size: 67.0,
            ..default()
        },
        TextColor::WHITE,
        Transform::from_xyz(0.0, 100., 0.0),
        GameOverText,
    ));

    commands.spawn((
        Text2d::new("Press space to start new game"),
        TextFont {
            font: asset_server.load("fonts/Pixellettersfull-BnJ5.ttf"),
            font_size: 30.0,
            ..default()
        },
        TextColor::WHITE,
        Transform::from_xyz(0.0, -200., 0.0),
        Visibility::Visible,
        GameOverText,
        BlinkingText,
    ));

    commands.spawn((
        Text2d::new("Press Esc to return to the main menu"),
        TextFont {
            font: asset_server.load("fonts/Pixellettersfull-BnJ5.ttf"),
            font_size: 22.0,
            ..default()
        },
        TextColor::WHITE,
        Transform::from_xyz(0.0, -280., 0.0),
        Visibility::Visible,
        GameOverText,
        BlinkingText,
    ));
}


pub fn display_score_gameover_text(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  score: Res<Score>,
  high_score: Res<HighScore>,
) {
    let current_score = score.get_score();
    let high_score_value = high_score.get();

    commands.spawn((
        Text2d::new(format!("Your score: {}", current_score)),
        TextFont {
            font: asset_server.load("fonts/Pixellettersfull-BnJ5.ttf"),
            font_size: 40.0,
            ..default()
        },
        TextColor::WHITE,
        Transform::from_xyz(0.0, 0., 0.0),
        GameOverText,
    ));

    // Check if this was a new high score
    if current_score == high_score_value && current_score > 0 {
        commands.spawn((
            Text2d::new("NEW HIGH SCORE!"),
            TextFont {
                font: asset_server.load("fonts/Pixellettersfull-BnJ5.ttf"),
                font_size: 50.0,
                ..default()
            },
            TextColor(Color::linear_rgba(1.0, 0.8, 0.0, 1.0)), // Gold color
            Transform::from_xyz(0.0, -50., 0.0),
            GameOverText,
        ));
    } else {
        commands.spawn((
            Text2d::new(format!("High score: {}", high_score_value)),
            TextFont {
                font: asset_server.load("fonts/Pixellettersfull-BnJ5.ttf"),
                font_size: 30.0,
                ..default()
            },
            TextColor::WHITE,
            Transform::from_xyz(0.0, -50., 0.0),
            GameOverText,
        ));
    }
}

pub fn despawn_gameover_text (
  mut commands: Commands,
  text_query: Query<Entity, With<GameOverText>>
) {
  for text_entity in text_query.iter() {
    commands.entity(text_entity).despawn();
  }
}




