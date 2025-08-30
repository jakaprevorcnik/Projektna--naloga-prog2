use bevy::prelude::*;

use crate::gameover::components::*;
use super::resources::{VisibleTextTimer, HiddenTextTimer};


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
// Na konzolo jaz ne bi nič pisala zares, ko bo ostalo urejeno.

pub fn display_score_gameover_text(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  score: Res<crate::resources::Score>,
  high_score: Res<crate::resources::HighScore>,
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
// Poseben pogoj za NewHighScore, da potem namesto High score: ... izpiše "NEW HIGH SCORE!" mal na večje

pub fn despawn_gameover_text (
  mut commands: Commands,
  text_query: Query<Entity, With<GameOverText>>
) {
  for text_entity in text_query.iter() {
    commands.entity(text_entity).despawn();
  }
}



pub fn tick_vh_text_timers(
    mut visible_text_timer: ResMut<VisibleTextTimer>,
    mut hidden_text_timer: ResMut<HiddenTextTimer>,
    time: Res<Time>
) {
    visible_text_timer.timer.tick(time.delta());
    hidden_text_timer.timer.tick(time.delta());
}


pub fn gameover_text_toggle_visibility (
    mut text_query: Query<&mut Visibility, With<BlinkingText>>,
    mut visible_text_timer: ResMut<VisibleTextTimer>,
    mut hidden_text_timer: ResMut<HiddenTextTimer>,
) {
    // Ali moram narediti poseben system, ki unpause-a oba timerja ob vstopu v state?
    // spawn-a se kot visible
    for mut visibility in text_query.iter_mut() {
        match *visibility {
            Visibility::Visible => {
                if visible_text_timer.timer.finished() {
                    *visibility = Visibility::Hidden;
                    visible_text_timer.timer.pause();
                    hidden_text_timer.timer.reset();
                    hidden_text_timer.timer.unpause();                    
                }
            }
            Visibility::Hidden => {
                if hidden_text_timer.timer.finished() {
                    *visibility = Visibility::Visible;
                    hidden_text_timer.timer.pause();
                    visible_text_timer.timer.reset();
                    visible_text_timer.timer.unpause();                    
                }                
            }
            _ => {} // Visibility::Inherited
        }
    }
}
