use bevy::prelude::*;

use crate::ui::components::*;
use crate::ui::resources::{VisibleTextTimer, HiddenTextTimer};


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