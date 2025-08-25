use bevy::prelude::*;

#[derive(Resource)]
pub struct GameTime {
    gametime: u32,
    timer: Timer
} // Teče linearno, dokler si v Game state-u, vsakič od začetka mora bit. 
// Je sestavni del score-a in merilec za naraščajočo težavnost.

#[derive(Resource)]
pub struct Score {
    score: u32
}

#[derive(Resource)]
pub struct HighScore {
    high_score: u32
}

impl GameTime {
    pub fn get_time(&self) -> u32 {
        self.gametime
    }
    
    pub fn update(&mut self, delta: std::time::Duration) {
        self.timer.tick(delta);
        if self.timer.just_finished() {
            self.gametime += 1;
        }
    }
    pub fn reset(&mut self) {
        self.gametime = 0;
        self.timer.reset();
    }
}

impl Default for GameTime {
    fn default() -> Self {
        GameTime {gametime: 0,
        timer: Timer::from_seconds(1.0, TimerMode::Repeating)
        }
    }
}


impl Default for Score {
    fn default() -> Self {
        Score {score: 0}
    }
}

impl Default for HighScore {
    fn default() -> Self {
        HighScore {high_score: 0}
    }
}