use bevy::prelude::*;

use crate::gameover::components::BlinkingText;
use super::components::*;


pub fn display_mainmenu_text(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
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
        MainMenuText,
        BlinkingText,
    ));

    commands.spawn((
        Text2d::new("Work in progress"),
        TextFont {
            font: asset_server.load("fonts/Pixellettersfull-BnJ5.ttf"),
            font_size: 40.0,
            ..default()
        },
        TextColor::WHITE,
        Transform::from_xyz(0.0, 0., 0.0),
        Visibility::Visible,
        MainMenuText,
    ));
}

pub fn display_score_mainmenu_text(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  high_score: Res<crate::resources::HighScore>,
) {
    let high_score_value = high_score.get();
    
    // Only show high score if it's greater than 0 (something has been played)
    if high_score_value > 0 {
        commands.spawn((
            Text2d::new(format!("High score: {}", high_score_value)),
            TextFont {
                font: asset_server.load("fonts/Pixellettersfull-BnJ5.ttf"),
                font_size: 30.0,
                ..default()
            },
            TextColor::WHITE,
            Transform::from_xyz(0.0, -256., 0.0),
            MainMenuText,
        ));  
    }
}


pub fn despawn_mainmenu_text (
  mut commands: Commands,
  text_query: Query<Entity, With<MainMenuText>>
) {
  for text_entity in text_query.iter() {
    commands.entity(text_entity).despawn();
  }
}



pub fn spawn_main_menu_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(
        (
            Sprite{
                image: asset_server.load("sprites/Space Rangers - Main Menu.png"),
                ..Default::default()
            },
            Transform::from_xyz(0.0, 0.0, -10.),
            MenuImage
        )
    );
}

pub fn despawn_main_menu_screen_image(
    mut commands: Commands,
    mut image_query: Query<Entity, With<MenuImage>>     
) {
    for image in image_query.iter() {
        commands.entity(image).despawn();
    }
}