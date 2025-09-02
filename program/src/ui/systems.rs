use bevy::prelude::*;

use crate::ui::components::*;
use crate::ui::resources::{VisibleTextTimer, HiddenTextTimer};
use crate::ui::components::GameText;
use crate::events::AstronautMissed;


pub fn tick_vh_text_timers(
    mut visible_text_timer: ResMut<VisibleTextTimer>,
    mut hidden_text_timer: ResMut<HiddenTextTimer>,
    time: Res<Time>
) {
    visible_text_timer.timer.tick(time.delta());
    hidden_text_timer.timer.tick(time.delta());
}


pub fn ui_text_toggle_visibility (
    mut text_query: Query<&mut Visibility, With<BlinkingText>>,
    mut visible_text_timer: ResMut<VisibleTextTimer>,
    mut hidden_text_timer: ResMut<HiddenTextTimer>,
) {
    // Spawn-a se kot visible.
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


pub fn display_astronauts_game_text(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Text2d::new("Astronauts missed:"),
        TextFont {
            font: asset_server.load("fonts/Pixellettersfull-BnJ5.ttf"),
            font_size: 30.0,
            ..default()
        },
        TextColor::WHITE,
        TextLayout::new_with_justify(JustifyText::Right),
        Transform::from_xyz(-400., -256., -1.),
        GameText
    ));  

    for position in vec![60., 120., 180.].iter() {
        commands.spawn(
        (
            Sprite{
                image: asset_server.load("sprites/spaceAstronauts_004.png"),
                color: Color::srgba(0.8, 0.8, 0.8, 0.1),
                ..Default::default()
            },
            Transform::from_xyz(-520. + position, -312., -2.),
            GameText
        ));
    }
}


pub fn read_and_display_astronauts_missed(
    mut astronauts_missed_event_reader: EventReader<AstronautMissed>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in astronauts_missed_event_reader.read() {
        let position = event.counter as f32 * 60.;

        commands.spawn(
        (
            Sprite{
                image: asset_server.load("sprites/spaceAstronauts_004.png"),
                ..Default::default()
            },
            Transform::from_xyz(-520. + position, -312., -1.),
            GameText
        ));        
    }
}