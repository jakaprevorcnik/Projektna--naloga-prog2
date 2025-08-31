use bevy::prelude::*;

#[derive(Component)]
pub struct Astronaut {
    pub points_value: u32,
}

impl Default for Astronaut {
    fn default() -> Self {
        Self {
            points_value: 100,
        }
    }
}