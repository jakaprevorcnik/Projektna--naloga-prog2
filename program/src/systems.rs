use bevy::prelude::*;


use crate::AppState;
use crate::events::*;



pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
  }



pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
        mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for event in game_over_event_reader.read() {
        println!("Your final score is: {}", event.score.to_string());
        app_state_next_state.set(AppState::GameOver);
        println!("Entered AppState::GameOver");
    }
}




// To bo šlo vse posebi še. Kle nej bi ble sam menjave. To je zdaj vse za minimalno delujočo ...
pub fn game_over_to_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut simulation_state_next_state: ResMut<NextState<AppState>>,
) {
    if *app_state.get() == AppState::GameOver {
        if keyboard_input.just_pressed(KeyCode::Space) {
            simulation_state_next_state.set(AppState::Game);
            println!("New game started.");
        }
    }
}

#[derive(Component)]
pub struct GameOverText;

pub fn display_gameover_text(
  mut commands: Commands,
  asset_server: Res<AssetServer>
) {
    // Text with one section
    commands.spawn((
        Text::new("GAME OVER"),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: asset_server.load("fonts/Pixellettersfull-BnJ5.ttf"),
            font_size: 67.0,
            ..default()
        },
        TextColor::WHITE,
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(300.0),
            left: Val::Px(340.0), // Še za ugotovit tole
            ..default()
        },
        GameOverText,
    ));

    commands.spawn((
        Text::new("Press space to start new game"),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: asset_server.load("fonts/Pixellettersfull-BnJ5.ttf"),
            font_size: 30.0,
            ..default()
        },
        TextColor::WHITE,
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(120.0),
            left: Val::Px(340.0), // Še za ugotovit tole. Itak mora bit nekak relativno na screen ...
            ..default()
        },
        GameOverText,
    ));
}

pub fn despawn_gameover_text (
  mut commands: Commands,
  text_query: Query<Entity, With<GameOverText>>
) {
  for text_entity in text_query.iter() {
    commands.entity(text_entity).despawn();
  }
}