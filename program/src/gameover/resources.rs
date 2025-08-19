use bevy::prelude::*;

pub const TOGGLE_TEXT_TIME: f32 = 0.5;

#[derive(Resource)]
pub struct ToggleTextTimer {
    pub timer: Timer,
}

impl Default for ToggleTextTimer {
    fn default() -> Self {
        ToggleTextTimer { 
            timer: Timer::from_seconds(TOGGLE_TEXT_TIME, TimerMode::Repeating) 
        }
    }
}