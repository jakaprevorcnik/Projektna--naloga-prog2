use bevy::prelude::*;
use std::fs;
use std::path::Path;

const HIGH_SCORE_FILE: &str = "highscore.txt";

#[derive(Resource)]
pub struct GameTime {
    pub gametime: f32,
    timer: Timer,
} 

#[derive(Resource)]
pub struct Score {
    pub score: u32,
    pub last_time_update: f32, 
}

#[derive(Resource)]
pub struct HighScore {
    pub high_score: u32
}

impl GameTime {
    pub fn get_time(&self) -> f32 {
        self.gametime
    }
    
    pub fn update(&mut self, delta: std::time::Duration) {
        self.timer.tick(delta);
        if self.timer.just_finished() {
            self.gametime += 1.0;
        }
    }
    
    pub fn update_realtime(&mut self, delta: f32) {
        self.gametime += delta;
    }
    
    pub fn reset(&mut self) {
        self.gametime = 0.0;
        self.timer.reset();
    }
    
    pub fn get_seconds(&self) -> u32 {
        self.gametime as u32
    }
}

impl Default for GameTime {
    fn default() -> Self {
        GameTime {
            gametime: 0.0,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating)
        }
    }
}


impl Default for Score {
    fn default() -> Self {
        Score {
            score: 0,
            last_time_update: 0.0,
        }
    }
}

impl Score {
    pub fn get_score(&self) -> u32 {
        self.score
    }
    
    pub fn set_score(&mut self, new_score: u32) {
        self.score = new_score;
    }
    
    pub fn add_points(&mut self, points: u32) {
        self.score += points;
    }
    
    pub fn reset(&mut self) {
        self.score = 0;
        self.last_time_update = 0.0;
    }
    
    pub fn update_score_with_gametime(&mut self, game_time: &GameTime) {
        let current_time = game_time.get_time();
        
       
        if current_time - self.last_time_update >= 0.1 {
           
            let intervals_passed = ((current_time - self.last_time_update) / 0.1) as u32;
           
            self.score += intervals_passed;
            
            self.last_time_update += (intervals_passed as f32) * 0.1;
        }
    }
}

impl Default for HighScore {
    fn default() -> Self {
        let high_score = load_high_score();
        HighScore { high_score }
    }
}

impl HighScore {
    pub fn get(&self) -> u32 {
        self.high_score
    }
    
    pub fn update(&mut self, new_score: u32) {//-> bool {
        if new_score > self.high_score {
            self.high_score = new_score;
            save_high_score(new_score);
        }
    }
}

// High score persistence functions
fn load_high_score() -> u32 { // Naloži high score iz datoteke highscore.txt
    if Path::new(HIGH_SCORE_FILE).exists() {
        match fs::read_to_string(HIGH_SCORE_FILE) {
            Ok(content) => content.trim().parse().unwrap_or(0),
            Err(_) => 0, // Ce dobimo kakrsno koli napako, nastavi high score na 0
        }
    } else {
        0
    }
}

fn save_high_score(score: u32) { // Shrani nov high score v datoteko highscore.txt
    if let Err(e) = fs::write(HIGH_SCORE_FILE, score.to_string()) {
        eprintln!("Failed to save high score: {}", e);
    }
} //Ce hocemo resetirat highscore, ga samo nastavimo na 0 v highscore.txt