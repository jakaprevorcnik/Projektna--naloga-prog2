use bevy::prelude::*;


pub const VISIBLE_TEXT_TIME: f32 = 0.8;
pub const HIDDEN_TEXT_TIME: f32 = 0.4;


#[derive(Resource)]
pub struct VisibleTextTimer {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct HiddenTextTimer {
    pub timer: Timer,
}


impl Default for VisibleTextTimer {
    fn default() -> Self {
        VisibleTextTimer { 
            timer: Timer::from_seconds(VISIBLE_TEXT_TIME, TimerMode::Once) 
        }
    }
}

impl Default for HiddenTextTimer {
    fn default() -> Self {
        HiddenTextTimer { 
            timer: Timer::from_seconds(HIDDEN_TEXT_TIME, TimerMode::Once) 
        }
    }
}