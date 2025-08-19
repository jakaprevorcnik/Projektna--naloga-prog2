use bevy::prelude::*;

use crate::gameover::components::*;
use super::resources::ToggleTextTimer;

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
}
// Manjka navodilo za gameover, ko bo gameover in izpis score-a na ekranu. Na konzolo jaz ne bi nič pisala zares.

pub fn despawn_gameover_text (
  mut commands: Commands,
  text_query: Query<Entity, With<GameOverText>>
) {
  for text_entity in text_query.iter() {
    commands.entity(text_entity).despawn();
  }
}



pub fn tick_toggle_text_timer(
    mut toggle_text_timer: ResMut<ToggleTextTimer>,
    time: Res<Time>
) {
    toggle_text_timer.timer.tick(time.delta());
}


pub fn gameover_text_toggle_visibility (
    mut text_query: Query<&mut Visibility, With<BlinkingText>>,
    toggle_text_timer: Res<ToggleTextTimer>
) {
    if toggle_text_timer.timer.finished() {
        for mut visibility in text_query.iter_mut() {
            visibility.toggle_visible_hidden();
        };
    }
}
