use bevy::prelude::*;

use crate::ui::score::resources::*;
use crate::ui::score::components::*;
use crate::ui::components::GameText;


pub fn display_score_game_text(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  score: Res<Score>,
  high_score: Res<HighScore>,
) {
    
    let current_score = score.get_score();
    let high_score_value = high_score.get();
    

    commands.spawn((
        Text2d::new(format!("Your score:\n{}", current_score)),
        TextFont {
            font: asset_server.load("fonts/Pixellettersfull-BnJ5.ttf"),
            font_size: 30.0,
            ..default()
        },
        TextColor::WHITE,
        TextLayout::new_with_justify(JustifyText::Right),
        Transform::from_xyz(-384., 48., -1.), //Ne vem, kako narediti transform glede na desen rob.
        GameText,
        ScoreText,
    ));  

    commands.spawn((
        Text2d::new(format!("High score:\n{}", high_score_value)),
        TextFont {
            font: asset_server.load("fonts/Pixellettersfull-BnJ5.ttf"),
            font_size: 30.0,
            ..default()
        },
        TextColor::WHITE,
        TextLayout::new_with_justify(JustifyText::Right),
        Transform::from_xyz(-384., -48., -1.),
        GameText,
        HighScoreText,
    ));  
}

pub fn update_score_display(
    mut score_text_query: Query<&mut Text2d, With<ScoreText>>,
    score: Res<Score>,
) {
    let current_score = score.get_score();
    for mut text in score_text_query.iter_mut() {
        text.0 = format!("Your score:\n{}", current_score);
    }
}

pub fn despawn_score_game_text(
  mut commands: Commands,
  text_query: Query<Entity, With<GameText>>
) {
  for text_entity in text_query.iter() {
    commands.entity(text_entity).despawn();
  }
}






pub fn update_game_time( mut game_time: ResMut<GameTime>, time: Res<Time>) {
    game_time.update(time.delta());
}

pub fn reset_game_time(mut game_time: ResMut<GameTime>) {
    game_time.reset();
}

pub fn update_score_with_time(
    mut score: ResMut<Score>,
    game_time: Res<GameTime>
) {
    score.update_score_with_gametime(&game_time);
}

pub fn reset_score(mut score: ResMut<Score>) {
    score.reset();
}