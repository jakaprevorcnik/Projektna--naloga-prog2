use bevy::prelude::*;

#[derive(Resource)]
pub struct GameTime {
    gametime: u32
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



impl Default for GameTime {
    fn default() -> Self {
        GameTime {gametime: 0}
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