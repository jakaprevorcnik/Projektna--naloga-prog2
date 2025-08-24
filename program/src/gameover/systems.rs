use bevy::prelude::*;

use crate::gameover::components::*;
use super::resources::{VisibleTextTimer, HiddenTextTimer};

use crate::AppState;



// Kje in kako naj bo tale funkcija, še ne vem.
pub fn game_over_to_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // app_state: Res<State<AppState>>,
    mut simulation_state_next_state: ResMut<NextState<AppState>>,
) {
    // if *app_state.get() == AppState::GameOver {
        if keyboard_input.just_pressed(KeyCode::Space) {
            simulation_state_next_state.set(AppState::Game);
            println!("New game started.");
        }
    // }
}


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
) {
    // še ne obstaja pravi score
    let score: u32 = 0;
    let high_score: u32 = 0;

    commands.spawn((
        Text2d::new(format!("Your score: {}", score)),
        TextFont {
            font: asset_server.load("fonts/Pixellettersfull-BnJ5.ttf"),
            font_size: 40.0,
            ..default()
        },
        TextColor::WHITE,
        Transform::from_xyz(0.0, 0., 0.0),
        GameOverText,
    ));

    commands.spawn((
        Text2d::new(format!("High score: {}", high_score)),
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
